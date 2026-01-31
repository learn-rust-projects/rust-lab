# Io-Uring Runtime Design Document

## 1. Overview

这是一个基于 Linux `io_uring` 接口的异步运行时原型。目标是演示如何通过 `Submission Queue (SQ)` 和 `Completion Queue (CQ)` 构建真正的异步 I/O 驱动，而非简单的线程池或阻塞模拟。

## 2. Architecture

整体架构采用 **Single Threaded Reactor** 模式。

### 2.1 Core Components

- **Reactor (Driver)**:
  - 持有 `io_uring::IoUring` 实例。
  - 维护 I/O 操作的状态 (`Lifecycle`)。
  - 负责提交 SQE (Submission Queue Entry)。
  - 负责轮询 CQE (Completion Queue Entry) 并唤醒对应的 `Waker`。

- **Lifecycle Store (`Slab<OpState>`)**:
  - 使用 `slab` 库存储所有进行中的 I/O 操作状态。
  - Key (Index) 作为 `user_data` 传递给 io_uring，用于在 completion 时找回上下文。

- **OpState**:
  - `Submitted`: 请求已提交，等待完成。包含 `Waker`。
  - `Completed`: 请求已完成，包含结果 (`i32` 结果码)。

- **Future (`UringFuture`)**:
  - 用户侧的异步句柄。
  - `poll` 阶段：1. 若未提交，向 Reactor 注册请求，获得 `Key`，状态转为 `Submitted`，返回 `Pending`。2. 若已完成，从 Reactor 获取结果，返回 `Ready`。

### 2.2 Workflow

1. **Submission**:
   - 用户调用异步 I/O (e.g., `AsyncFile::read`).
   - 创建 `UringFuture`。
   - `Future::poll` 被调用。
   - Future 向 Reactor 申请 Slot，构建 SQE，填入 `user_data = key`。
   - Reactor 提交请求到 Ring。
   - Future 返回 `Poll::Pending`。

2. **Completion**:
   - Runtime 循环调用 `reactor.wait()`。
   - `io_uring_enter` 陷入内核等待事件。
   - 内核写入 CQE。
   - Reactor 获取 CQE，提取 `user_data` (即 Key)。
   - Reactor 根据 Key 找到 `OpState`，将状态更新为 `Completed(result)`，并调用 `waker.wake()`。

3. **Wakeup**:
   - Executor 收到唤醒信号，再次 `poll` 对应的 Task。
   - Task 再次 `poll` `UringFuture`。
   - `UringFuture` 发现状态为 `Completed`，返回 `Poll::Ready(result)`。

## 3. Interfaces

### 3.1 Runtime

```rust
pub struct Runtime {
    reactor: Rc<Reactor>,
}

impl Runtime {
    pub fn new() -> io::Result<Self>;
    pub fn block_on<F: Future>(&mut self, future: F) -> F::Output;
}
```

### 3.2 Async I/O (Example)

```rust
pub struct AsyncFile {
    fd: RawFd,
}

impl AsyncFile {
    pub fn read(&self, buf: &mut [u8]) -> impl Future<Output = io::Result<usize>>;
}
```

## 4. Dependencies

- `io-uring`: Rust bindings for io_uring.
- `slab`: Efficient indexed storage.
- `scoped-tls`: Thread-local reactor access (optional, or use explicit passing).
