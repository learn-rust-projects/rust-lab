// 可以在 main.rs 中添加一个单元测试模块
#[cfg(test)] // 仅在运行测试时编译此模块
mod tests {
    use std::{
        io::{self, Read, Write},
        process::{Command, Stdio},
    };

    #[test]
    fn check_pipe_ipc_works() -> io::Result<()> {
        let input_data = "Hello Pipe\n";
        // 创建并启动一个新的外部命令
        // rev，并设置其标准输入、标准输出均可在父进程中读写。
        // rev 是 Linux/Unix 系统自带的命令，用于反转输入的每一行字符串。
        // 第一个管道用于 父写 → 子读（stdin）
        // 第二个管道用于 子写 → 父读（stdout）
        let mut child = Command::new("rev")
            .stdin(Stdio::piped())// 父 → 子
            .stdout(Stdio::piped())// 子 → 父
            .spawn()?;

        // 这行代码的目的，是从 child（类型为
        // std::process::Child）中取出其标准输入的写入端（父进程一侧的 pipe
        // 句柄），用于后续写入数据给子进程。
        // ChildStdin 是标准输入（stdin）的句柄，代表父进程可写入的数据通道。
        // 句柄是操作系统提供的一个抽象概念，表示对某个资源（如文件、管道、
        // 网络连接等）的引用或访问点。这里可以是文件、管道、网络连接、继承等。
        // 只有管道（Pipe）才能让父进程获取 Some(T)
        let mut child_stdin = child.stdin.take().expect("Failed to open child stdin pipe");
        println!("父进程：正在通过管道写入数据：\"{}\"", input_data.trim());
        child_stdin.write_all(input_data.as_bytes())?;
        drop(child_stdin);

        let mut child_stdout = child
            .stdout
            .take()
            .expect("Failed to open child stdout pipe");
        let mut output_buffer = String::new();
        println!("父进程：正在通过管道读取子进程的输出...");
        child_stdout.read_to_string(&mut output_buffer)?;

        let status = child.wait()?;

        println!("子进程已退出，状态：{}", status);

        // ... (省略部分结果打印) ...
        println!("子进程原始输出：\"{}\"", output_buffer.trim());

        let expected_output = "epiP olleH";
        if output_buffer.trim() == expected_output {
            println!("\n✅ 验证成功：输出与预期相符。");
        } else {
            eprintln!("\n❌ 验证失败：输出不符！");
        }

        Ok(())
    }
    #[test]
    fn test_os_pipe() -> io::Result<()> {
        // 1. 创建匿名管道，返回 (读取端, 写入端)
        let (mut read_pipe, mut write_pipe) = os_pipe::pipe()?;

        let data_to_send = "Hello from the writer side!";

        // 2. 写入数据到管道
        println!("写入端正在发送数据: '{}'", data_to_send);
        write_pipe.write_all(data_to_send.as_bytes())?;

        // 关闭写入端非常重要，否则读取端可能永远不会收到 EOF
        // 如果不关闭，read_to_string可能会一直阻塞
        drop(write_pipe);

        // 3. 从管道读取数据
        let mut buffer = String::new();
        read_pipe.read_to_string(&mut buffer)?;

        println!("读取端收到的数据: '{}'", buffer);
        Ok(())
    }
}
