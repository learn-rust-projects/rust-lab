use std::{
    cell::RefCell,
    io,
    task::{Context, Poll, Waker},
};

use io_uring::{IoUring, cqueue, squeue};
use scoped_tls::scoped_thread_local;
use slab::Slab;

scoped_thread_local!(pub static CURRENT_REACTOR: Reactor);

/// I/O 操作的状态
#[derive(Debug)]
enum Lifecycle {
    /// 已提交给 io_uring，等待完成
    Submitted,
    /// io_uring 已完成，结果已就绪，等待 Future 取走
    Completed(i32),
}

/// Slab 中存储的条目
struct Entry {
    lifecycle: Lifecycle,
    waker: Option<Waker>,
}

pub struct Reactor {
    ring: RefCell<IoUring>,
    ops: RefCell<Slab<Entry>>,
}

impl Reactor {
    pub fn new() -> io::Result<Self> {
        let ring = IoUring::new(1024)?;
        Ok(Self {
            ring: RefCell::new(ring),
            ops: RefCell::new(Slab::with_capacity(1024)),
        })
    }

    pub fn with_current<F, R>(f: F) -> R
    where
        F: FnOnce(&Reactor) -> R,
    {
        if CURRENT_REACTOR.is_set() {
            CURRENT_REACTOR.with(|reactor| f(reactor))
        } else {
            panic!("Reactor is not running");
        }
    }

    /// 提交一个 I/O 操作，返回在 Slab 中的 Key
    ///
    /// # Safety
    /// `sqe` 必须是有效的 Submission Queue Entry
    pub unsafe fn submit(&self, sqe: squeue::Entry, waker: Waker) -> io::Result<usize> {
        let mut ops = self.ops.borrow_mut();

        // 1. 分配 Slot
        let key = ops.insert(Entry {
            lifecycle: Lifecycle::Submitted,
            waker: Some(waker),
        });

        // 2. 准备 SQE
        let sqe = sqe.user_data(key as u64);

        // 3. 尝试推入 submission queue
        let mut ring = self.ring.borrow_mut();

        if ring.submission().is_full() {
            // 如果满了，先提交当前的
            ring.submit()?;
            drop(ring);

            // 必须释放 ops，因为 wait() -> process_completions() 需要获取 ops 锁
            drop(ops);

            // 处理完成事件，腾出 CQ，间接可能帮助（虽然 SQ 和 CQ 独立）
            // 实际上如果 SQ 满了，我们应该等待 SQE 被消费，但 io_uring 通常 SQ 很大。
            // 这里简单策略：等待一下
            self.wait()?;

            // 重新获取锁
            ops = self.ops.borrow_mut();
            ring = self.ring.borrow_mut();

            if ring.submission().is_full() {
                drop(ring);
                ops.remove(key);
                return Err(io::Error::other("Submission queue is full"));
            }
        }

        let push_result = unsafe { ring.submission().push(&sqe) };
        if push_result.is_err() {
            drop(ring);
            ops.remove(key);
            return Err(io::Error::other("Failed to push to submission queue"));
        }

        // 4. 立即通知内核
        ring.submit()?;

        Ok(key)
    }

    /// 驱动 Reactor，阻塞等待至少一个事件完成
    pub fn wait(&self) -> io::Result<()> {
        let mut ring = self.ring.borrow_mut();

        // 提交所有积压的请求，并等待至少 1 个完成
        ring.submit_and_wait(1)?;

        // 获取所有 CQE
        let completions: Vec<cqueue::Entry> = ring.completion().collect();

        drop(ring); // 释放借用

        let mut ops = self.ops.borrow_mut();

        for cqe in completions {
            let key = cqe.user_data() as usize;
            let result = cqe.result();

            if let Some(entry) = ops.get_mut(key) {
                entry.lifecycle = Lifecycle::Completed(result);
                if let Some(waker) = entry.waker.take() {
                    waker.wake();
                }
            }
        }

        Ok(())
    }

    /// 检查某个操作的状态
    pub fn poll_op(&self, key: usize, cx: &mut Context<'_>) -> Poll<io::Result<i32>> {
        let mut ops = self.ops.borrow_mut();

        if let Some(entry) = ops.get_mut(key) {
            match entry.lifecycle {
                Lifecycle::Submitted => {
                    // 更新 Waker
                    if entry
                        .waker
                        .as_ref()
                        .is_none_or(|w| !w.will_wake(cx.waker()))
                    {
                        entry.waker = Some(cx.waker().clone());
                    }
                    Poll::Pending
                }
                Lifecycle::Completed(res) => {
                    // 操作完成，移除 Entry 并返回结果
                    ops.remove(key);
                    Poll::Ready(Ok(res))
                }
            }
        } else {
            Poll::Ready(Err(io::Error::new(io::ErrorKind::NotFound, "Op not found")))
        }
    }

    /// 阻塞运行一个 Future 直到完成
    pub fn block_on<F: std::future::Future>(self, future: F) -> F::Output {
        let reactor = self;
        CURRENT_REACTOR.set(&reactor, || {
            let mut future = std::pin::pin!(future);
            let waker = Waker::from(std::sync::Arc::new(DummyWaker));
            let mut cx = Context::from_waker(&waker);

            loop {
                match future.as_mut().poll(&mut cx) {
                    Poll::Ready(val) => return val,
                    Poll::Pending => {
                        reactor.wait().expect("Reactor wait failed");
                    }
                }
            }
        })
    }
}

struct DummyWaker;
impl std::task::Wake for DummyWaker {
    fn wake(self: std::sync::Arc<Self>) {}
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use io_uring::{opcode, types};

    use super::*;
    use crate::future::UringFuture;

    #[test]
    fn test_nop() {
        let reactor = Reactor::new().unwrap();

        let result = reactor.block_on(async {
            let nop = opcode::Nop::new().build();
            UringFuture::new(nop).await
        });

        assert!(result.is_ok());
    }

    #[test]
    fn test_timeout() {
        let reactor = Reactor::new().unwrap();

        let start = std::time::Instant::now();
        let result = reactor.block_on(async {
            let ts = types::Timespec::new().nsec(10_000_000); // 10ms
            let timeout = opcode::Timeout::new(&ts).build();
            UringFuture::new(timeout).await
        });

        assert!(result.is_ok());
        // io_uring timeouts return -ETIME if they expire
        let res = result.unwrap();
        assert!(res == -libc::ETIME || res == 0, "Result was {}", res);

        // Ensure it actually slept
        assert!(start.elapsed() >= Duration::from_millis(10));
    }

    #[test]
    fn test_file_read() {
        use std::{io::Write, os::unix::io::AsRawFd};

        let mut temp = tempfile::NamedTempFile::new().unwrap();
        temp.write_all(b"Hello io_uring").unwrap();
        temp.flush().unwrap();

        let file = std::fs::File::open(temp.path()).unwrap();
        let fd = file.as_raw_fd();
        let mut buf = vec![0u8; 20];

        let reactor = Reactor::new().unwrap();
        let n = reactor
            .block_on(async {
                let read =
                    opcode::Read::new(types::Fd(fd), buf.as_mut_ptr(), buf.len() as u32).build();
                UringFuture::new(read).await
            })
            .unwrap();

        assert_eq!(n, 14); // "Hello io_uring".len()
        assert_eq!(&buf[..14], b"Hello io_uring");
    }
}
