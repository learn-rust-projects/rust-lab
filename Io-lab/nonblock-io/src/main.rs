// 导入标准库中的必要模块
use std::{
    io::{self, Read, Write},  // 输入输出相关功能，包括读写trait
    os::fd::{AsRawFd, RawFd}, // 文件描述符相关功能，用于获取原始文件描述符
};

// 导入libc库中的常量和函数，用于底层系统调用
use libc::{F_GETFL, F_SETFL, O_NONBLOCK, fcntl};

/// 设置文件描述符标志（如 O_NONBLOCK）
///
/// # 参数
/// * `fd` - 要设置的文件描述符
/// * `flags` - 要设置的标志位（如 O_NONBLOCK）
///
/// # 返回值
/// * `io::Result<()>` - 成功返回Ok(())，失败返回错误
///
/// # 实现原理
/// 使用fcntl系统调用获取当前文件描述符标志，然后添加新的标志并设置回去
fn set_fl(fd: RawFd, flags: i32) -> io::Result<()> {
    unsafe {
        // 获取当前文件描述符的标志
        let val = fcntl(fd, F_GETFL);
        if val < 0 {
            return Err(io::Error::last_os_error());
        }
        // 设置新的标志，使用位运算添加标志位
        if fcntl(fd, F_SETFL, val | flags) < 0 {
            return Err(io::Error::last_os_error());
        }
    }
    Ok(())
}

/// 清除文件描述符标志
///
/// # 参数
/// * `fd` - 要清除标志的文件描述符
/// * `flags` - 要清除的标志位
///
/// # 返回值
/// * `io::Result<()>` - 成功返回Ok(())，失败返回错误
///
/// # 实现原理
/// 使用fcntl系统调用获取当前文件描述符标志，然后使用位运算清除指定的标志位
fn clr_fl(fd: RawFd, flags: i32) -> io::Result<()> {
    unsafe {
        // 获取当前文件描述符的标志
        let val = fcntl(fd, F_GETFL);
        if val < 0 {
            return Err(io::Error::last_os_error());
        }
        // 清除指定的标志位，使用位运算取反后与操作
        if fcntl(fd, F_SETFL, val & !flags) < 0 {
            return Err(io::Error::last_os_error());
        }
    }
    Ok(())
}

/// 主程序逻辑：非阻塞写出
///
/// # 泛型参数
/// * `R` - 实现了Read trait的读取器
/// * `W` - 实现了Write和AsRawFd trait的写入器
///
/// # 参数
/// * `reader` - 数据源，从中读取数据
/// * `writer` - 目标写入器，向其写入数据
///
/// # 返回值
/// * `io::Result<()>` - 成功返回Ok(())，失败返回错误
///
/// # 实现原理
/// 1. 从reader读取所有数据到缓冲区
/// 2. 设置writer为非阻塞模式
/// 3. 循环写入数据，处理非阻塞写入可能的部分写入和EAGAIN错误
/// 4. 恢复writer的原始阻塞状态
pub fn run_nonblock<R: Read, W: Write + AsRawFd>(mut reader: R, writer: &mut W) -> io::Result<()> {
    // 创建缓冲区并读取所有数据
    let mut buf = Vec::new();
    reader.read_to_end(&mut buf)?;
    eprintln!("read {} bytes", buf.len());

    // 获取写入器的原始文件描述符
    let fd = writer.as_raw_fd();
    // 设置文件描述符为非阻塞模式
    set_fl(fd, O_NONBLOCK)?;

    // 循环写入数据，直到所有数据都写入完成
    let mut written = 0;
    while written < buf.len() {
        // 获取尚未写入的数据切片
        let slice = &buf[written..];
        unsafe {
            // 使用libc的write函数进行底层写入
            let ret = libc::write(fd, slice.as_ptr() as *const libc::c_void, slice.len());
            if ret > 0 {
                // 写入成功，更新已写入字节数
                eprintln!("nwrite = {}, errno = 0", ret);
                written += ret as usize;
            } else if ret == -1 {
                // 写入失败，获取错误码
                // 这是 Rust 标准库提供的 获取最近一次操作系统错误 的方法。
                let err = io::Error::last_os_error();
                // raw_os_error() 返回一个 Option<i32>，即底层操作系统的错误码（errno 值）。
                let errno = err.raw_os_error().unwrap_or(-1);
                eprintln!("nwrite = -1, errno = {}", errno);
                // 如果不是EAGAIN错误（表示暂时无法写入），则返回错误
                if errno != libc::EAGAIN {
                    return Err(err);
                }
                // 如果是EAGAIN错误，继续循环尝试写入
                // 是 EAGAIN → 说明缓冲区暂时满了，可以等一会或继续循环尝试写入
            }
        }
    }

    // 恢复文件描述符的原始状态（清除非阻塞标志）
    clr_fl(fd, O_NONBLOCK)?;
    Ok(())
}

/// 主函数
///
/// # 实现原理
/// 从标准输入读取数据，使用非阻塞方式写入到标准输出
fn main() -> io::Result<()> {
    // 获取标准输入和标准输出
    let stdin = io::stdin();
    let stdout = io::stdout();
    // 锁定标准输出，确保写入的原子性
    let mut stdout_lock = stdout.lock();
    // 调用非阻塞写入函数
    run_nonblock(stdin.lock(), &mut stdout_lock)
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;
    use std::io::{Seek, SeekFrom}; // ← 必须引入 Seek trait

    use tempfile::tempfile;

    use super::*;

    /// 测试非阻塞写入是否能完整输出所有数据
    #[test]
    fn test_nonblock_writes_complete_output() {
        // 构造虚拟输入数据（相当于 bigfile）
        let input_data = vec![b'X'; 200_000];
        let reader = Cursor::new(input_data.clone());

        // 创建临时文件作为输出（模拟 stdout 重定向）
        let mut out_file = tempfile().expect("create temp output file failed");

        // 调用核心逻辑
        run_nonblock(reader, &mut out_file).expect("nonblock run failed");

        // 校验输出一致性
        // 刷新文件缓冲区确保所有数据都写入磁盘
        out_file.seek(SeekFrom::Start(0)).unwrap();

        let mut result = Vec::new();
        out_file.read_to_end(&mut result).unwrap();

        assert_eq!(result, input_data, "非阻塞写入后的输出内容应与输入完全一致");
    }

    use std::os::unix::io::RawFd;

    use libc::pipe;
    /// 测试当管道缓冲区满时，非阻塞写入的行为
    #[test]
    fn test_nonblock_full_pipe() {
        // 创建管道
        let mut fds = [0; 2];
        unsafe {
            assert_eq!(pipe(fds.as_mut_ptr()), 0);
        }
        let rfd = fds[0]; // 读端
        let wfd = fds[1]; // 写端

        // 设置写端为非阻塞
        set_fl(wfd, O_NONBLOCK).expect("set nonblock failed");

        // 构造超大数据，超过系统管道缓冲区（通常为64KB）
        let big_data = vec![b'A'; 1_000_000];

        // 尝试写入
        let mut total_written = 0;
        loop {
            unsafe {
                let ret = libc::write(
                    wfd,
                    big_data[total_written..].as_ptr() as *const libc::c_void,
                    big_data.len() - total_written,
                );
                if ret > 0 {
                    // 写入成功，更新已写入字节数
                    total_written += ret as usize;
                } else if ret == -1 {
                    // 写入失败，获取错误码
                    let errno = io::Error::last_os_error().raw_os_error().unwrap();
                    eprintln!("write returned -1, errno = {}", errno);
                    // 当缓冲区满时，非阻塞写返回 EAGAIN
                    assert_eq!(errno, libc::EAGAIN);
                    break;
                }
            }
        }

        // 关闭管道两端
        unsafe {
            libc::close(rfd);
            libc::close(wfd);
        }
    }
}
