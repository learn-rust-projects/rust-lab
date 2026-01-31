use std::{
    future::Future,
    io,
    pin::Pin,
    task::{Context, Poll},
};

use io_uring::squeue;

use crate::reactor::Reactor;

pub struct UringFuture {
    sqe: Option<squeue::Entry>,
    key: Option<usize>,
}

impl UringFuture {
    pub fn new(sqe: squeue::Entry) -> Self {
        Self {
            sqe: Some(sqe),
            key: None,
        }
    }
}

impl Future for UringFuture {
    type Output = io::Result<i32>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if let Some(key) = self.key {
            // 已经提交过，检查状态
            Reactor::with_current(|reactor| reactor.poll_op(key, cx))
        } else {
            // 尚未提交，进行提交
            let sqe = self.sqe.take().expect("Future polled after completion");
            let waker = cx.waker().clone();

            Reactor::with_current(|reactor| unsafe {
                match reactor.submit(sqe, waker) {
                    Ok(key) => {
                        self.key = Some(key);
                        Poll::Pending
                    }
                    Err(e) => Poll::Ready(Err(e)),
                }
            })
        }
    }
}
