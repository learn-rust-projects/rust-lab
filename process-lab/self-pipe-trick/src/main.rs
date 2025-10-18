use std::{
    io::{
        Read, Write, {self},
    },
    os::unix::io::FromRawFd,
    thread,
    time::Duration,
};

use mio::{
    Events, Interest, Poll, Token,
    unix::pipe::{Receiver, Sender},
}; // mio 1.1 提供 Unix pipe 接口

const PIPE_TOKEN: Token = Token(0);

fn main() -> io::Result<()> {
    println!("--- Self-Pipe Trick 演示（mio 1.1 安全版） ---");

    // 1️⃣ 创建管道
    let (mut sender, mut receiver) = create_pipe()?;

    // 2️⃣ 创建 Poll
    let mut poll = Poll::new()?;
    let mut events = Events::with_capacity(128);

    // 3️⃣ 注册读取端
    poll.registry()
        .register(&mut receiver, PIPE_TOKEN, Interest::READABLE)?;

    println!("[初始化] 管道已注册到 Poll");

    // 4️⃣ 后台线程写入
    let handle = thread::spawn(move || {
        thread::sleep(Duration::from_secs(3));
        println!("[后台线程] 写入唤醒信号...");
        let _ = sender.write_all(&[b'W']);
    });

    // 5️⃣ 主线程等待事件
    poll.poll(&mut events, None)?;
    for event in &events {
        if event.token() == PIPE_TOKEN {
            let mut buf = [0u8; 1];
            receiver.read_exact(&mut buf)?;
            println!("[主线程] 收到唤醒信号 '{}'", buf[0] as char);
        }
    }

    handle.join().unwrap();
    Ok(())
}

/// 使用 Unix pipe 创建 Sender/Receiver
fn create_pipe() -> io::Result<(Sender, Receiver)> {
    let (read_fd, write_fd) = create_anonymous_pipe()?;
    let sender = unsafe { Sender::from_raw_fd(write_fd) };
    let receiver = unsafe { Receiver::from_raw_fd(read_fd) };
    Ok((sender, receiver))
}

/// 创建匿名管道
#[cfg(unix)]
fn create_anonymous_pipe() -> io::Result<(i32, i32)> {
    let mut fds = [0; 2];
    let res = unsafe { libc::pipe(fds.as_mut_ptr()) };
    if res != 0 {
        Err(io::Error::last_os_error())
    } else {
        Ok((fds[0], fds[1]))
    }
}

#[cfg(not(unix))]
fn create_anonymous_pipe() -> io::Result<(i32, i32)> {
    Err(io::Error::new(io::ErrorKind::Other, "Unix only"))
}
