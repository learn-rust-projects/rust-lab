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

    #[test]
    fn test_fork_pipe_echo_grep() -> io::Result<()> {
        use std::{
            ffi::CString,
            os::fd::{AsRawFd, FromRawFd},
        };

        use libc::{close, dup2, execl, fork, pipe, waitpid};

        println!("=== 测试: 使用fork实现 echo 12313 | grep 12 ===\n");

        // 1. 创建管道
        let mut pipe_fds: [libc::c_int; 2] = [0; 2];
        unsafe {
            if pipe(pipe_fds.as_mut_ptr()) == -1 {
                return Err(io::Error::last_os_error());
            }
        }

        let (pipe_read, pipe_write) = (pipe_fds[0], pipe_fds[1]);

        // 2. 第一次fork - 创建grep进程
        let grep_pid = unsafe { fork() };

        match grep_pid {
            -1 => {
                // fork失败
                unsafe {
                    close(pipe_read);
                    close(pipe_write);
                }
                return Err(io::Error::last_os_error());
            }
            0 => {
                // grep子进程
                println!("grep子进程 (PID: {}) 启动", unsafe { libc::getpid() });

                // 关闭不需要的管道端
                unsafe { close(pipe_write) };

                // 重定向标准输入到管道读取端
                unsafe {
                    dup2(pipe_read, libc::STDIN_FILENO);
                    close(pipe_read);
                }

                // 执行grep命令
                let grep_cmd = CString::new("/bin/grep").unwrap();
                let pattern = CString::new("12 12").unwrap();

                unsafe {
                    execl(
                        grep_cmd.as_ptr(),
                        grep_cmd.as_ptr(),
                        pattern.as_ptr(),
                        std::ptr::null() as *const libc::c_char,
                    );
                }

                // 如果execl失败，执行到这里
                eprintln!("grep执行失败: {}", io::Error::last_os_error());
                unsafe { libc::exit(1) };
            }
            _ => {
                // 父进程继续
                println!("父进程继续，grep子进程PID: {}", grep_pid);
            }
        }

        // 3. 第二次fork - 创建echo进程
        let echo_pid = unsafe { fork() };

        match echo_pid {
            -1 => {
                // fork失败
                unsafe {
                    close(pipe_read);
                    close(pipe_write);
                }
                return Err(io::Error::last_os_error());
            }
            0 => {
                // echo子进程
                println!("echo子进程 (PID: {}) 启动", unsafe { libc::getpid() });

                // 关闭不需要的管道端
                unsafe { close(pipe_read) };

                // 重定向标准输出到管道写入端
                unsafe {
                    dup2(pipe_write, libc::STDOUT_FILENO);
                    close(pipe_write);
                }

                // 执行echo命令
                let echo_cmd = CString::new("/bin/echo").unwrap();
                let text = CString::new("12313").unwrap();

                unsafe {
                    execl(
                        echo_cmd.as_ptr(),
                        echo_cmd.as_ptr(),
                        text.as_ptr(),
                        std::ptr::null() as *const libc::c_char,
                    );
                }

                // 如果execl失败，执行到这里
                eprintln!("echo执行失败: {}", io::Error::last_os_error());
                unsafe { libc::exit(1) };
            }
            _ => {
                // 父进程继续
                println!("父进程继续，echo子进程PID: {}", echo_pid);
            }
        }

        // 4. 父进程关闭管道两端（子进程已经接管）
        unsafe {
            close(pipe_read);
            close(pipe_write);
        }

        // 5. 等待子进程结束
        let mut grep_status: libc::c_int = 0;
        let mut echo_status: libc::c_int = 0;

        unsafe {
            waitpid(grep_pid, &mut grep_status, 0);
            waitpid(echo_pid, &mut echo_status, 0);
        }
        println!("\n子进程状态:");
        println!("  grep进程 (PID: {}) 退出状态: {}", grep_pid, grep_status);
        println!("  echo进程 (PID: {}) 退出状态: {}", echo_pid, echo_status);

        // 6. 验证结果
        if grep_status == 0 {
            println!("\n✅ 测试成功: grep找到了匹配的模式 '12'");
        } else {
            println!("\n❌ 测试失败: grep未找到匹配的模式 '12'");
        }

        Ok(())
    }

    #[test]
    fn test_fork_pipe_echo_grep_execv() -> io::Result<()> {
        use std::{
            ffi::CString,
            os::fd::{AsRawFd, FromRawFd},
        };

        use libc::{close, dup2, execv, fork, pipe, waitpid};

        println!("=== 测试: 使用execv实现 echo 12313 | grep 12 ===\n");
        println!("特点: 参数数组，不搜索PATH，适用于参数数量动态变化的命令\n");

        // 1. 创建管道
        let mut pipe_fds: [libc::c_int; 2] = [0; 2];
        unsafe {
            if pipe(pipe_fds.as_mut_ptr()) == -1 {
                return Err(io::Error::last_os_error());
            }
        }

        let (pipe_read, pipe_write) = (pipe_fds[0], pipe_fds[1]);

        // 2. 第一次fork - 创建grep进程
        let grep_pid = unsafe { fork() };

        match grep_pid {
            -1 => {
                unsafe {
                    close(pipe_read);
                    close(pipe_write);
                }
                return Err(io::Error::last_os_error());
            }
            0 => {
                // grep子进程 - 使用execv
                println!("grep子进程 (PID: {}) 启动 - 使用execv", unsafe {
                    libc::getpid()
                });

                unsafe { close(pipe_write) };
                unsafe {
                    dup2(pipe_read, libc::STDIN_FILENO);
                    close(pipe_read);
                }

                // execv使用参数数组
                let grep_cmd = CString::new("/bin/grep").unwrap();
                let grep_args: [*const libc::c_char; 3] = [
                    CString::new("grep").unwrap().as_ptr(),
                    CString::new("12").unwrap().as_ptr(),
                    std::ptr::null(),
                ];

                unsafe {
                    execv(grep_cmd.as_ptr(), grep_args.as_ptr());
                }

                eprintln!("grep执行失败: {}", io::Error::last_os_error());
                unsafe { libc::exit(1) };
            }
            _ => {
                println!("父进程继续，grep子进程PID: {}", grep_pid);
            }
        }

        // 3. 第二次fork - 创建echo进程
        let echo_pid = unsafe { fork() };

        match echo_pid {
            -1 => {
                unsafe {
                    close(pipe_read);
                    close(pipe_write);
                }
                return Err(io::Error::last_os_error());
            }
            0 => {
                // echo子进程 - 使用execv
                println!("echo子进程 (PID: {}) 启动 - 使用execv", unsafe {
                    libc::getpid()
                });

                unsafe { close(pipe_read) };
                unsafe {
                    dup2(pipe_write, libc::STDOUT_FILENO);
                    close(pipe_write);
                }

                // execv使用参数数组
                let echo_cmd = CString::new("/bin/echo").unwrap();
                let echo_args: [*const libc::c_char; 3] = [
                    CString::new("echo").unwrap().as_ptr(),
                    CString::new("12313").unwrap().as_ptr(),
                    std::ptr::null(),
                ];

                unsafe {
                    execv(echo_cmd.as_ptr(), echo_args.as_ptr());
                }

                eprintln!("echo执行失败: {}", io::Error::last_os_error());
                unsafe { libc::exit(1) };
            }
            _ => {
                println!("父进程继续，echo子进程PID: {}", echo_pid);
            }
        }

        // 父进程关闭管道，等待子进程
        unsafe {
            close(pipe_read);
            close(pipe_write);
        }

        let mut grep_status: libc::c_int = 0;
        let mut echo_status: libc::c_int = 0;

        unsafe {
            waitpid(grep_pid, &mut grep_status, 0);
            waitpid(echo_pid, &mut echo_status, 0);
        }

        println!("\n子进程状态:");
        println!("  grep进程 (PID: {}) 退出状态: {}", grep_pid, grep_status);
        println!("  echo进程 (PID: {}) 退出状态: {}", echo_pid, echo_status);

        if grep_status == 0 {
            println!("\n✅ execv测试成功: grep找到了匹配的模式 '12'");
        } else {
            println!("\n❌ execv测试失败: grep未找到匹配的模式 '12'");
        }

        Ok(())
    }

    #[test]
    fn test_fork_pipe_echo_grep_execlp() -> io::Result<()> {
        use std::{
            ffi::CString,
            os::fd::{AsRawFd, FromRawFd},
        };

        use libc::{close, dup2, execlp, fork, pipe, waitpid};

        println!("=== 测试: 使用execlp实现 echo 12313 | grep 12 ===\n");
        println!("特点: 可变参数列表，搜索PATH，适用于系统命令\n");

        // 1. 创建管道
        let mut pipe_fds: [libc::c_int; 2] = [0; 2];
        unsafe {
            if pipe(pipe_fds.as_mut_ptr()) == -1 {
                return Err(io::Error::last_os_error());
            }
        }

        let (pipe_read, pipe_write) = (pipe_fds[0], pipe_fds[1]);

        // 2. 第一次fork - 创建grep进程
        let grep_pid = unsafe { fork() };

        match grep_pid {
            -1 => {
                unsafe {
                    close(pipe_read);
                    close(pipe_write);
                }
                return Err(io::Error::last_os_error());
            }
            0 => {
                // grep子进程 - 使用execlp
                println!("grep子进程 (PID: {}) 启动 - 使用execlp", unsafe {
                    libc::getpid()
                });

                unsafe { close(pipe_write) };
                unsafe {
                    dup2(pipe_read, libc::STDIN_FILENO);
                    close(pipe_read);
                }

                // execlp在PATH中搜索grep命令
                let grep_cmd = CString::new("grep").unwrap();
                let pattern = CString::new("12").unwrap();

                unsafe {
                    execlp(
                        grep_cmd.as_ptr(),
                        grep_cmd.as_ptr(),
                        pattern.as_ptr(),
                        std::ptr::null() as *const libc::c_char,
                    );
                }

                eprintln!("grep执行失败: {}", io::Error::last_os_error());
                unsafe { libc::exit(1) };
            }
            _ => {
                println!("父进程继续，grep子进程PID: {}", grep_pid);
            }
        }

        // 3. 第二次fork - 创建echo进程
        let echo_pid = unsafe { fork() };

        match echo_pid {
            -1 => {
                unsafe {
                    close(pipe_read);
                    close(pipe_write);
                }
                return Err(io::Error::last_os_error());
            }
            0 => {
                // echo子进程 - 使用execlp
                println!("echo子进程 (PID: {}) 启动 - 使用execlp", unsafe {
                    libc::getpid()
                });

                unsafe { close(pipe_read) };
                unsafe {
                    dup2(pipe_write, libc::STDOUT_FILENO);
                    close(pipe_write);
                }

                // execlp在PATH中搜索echo命令
                let echo_cmd = CString::new("echo").unwrap();
                let text = CString::new("12313").unwrap();

                unsafe {
                    execlp(
                        echo_cmd.as_ptr(),
                        echo_cmd.as_ptr(),
                        text.as_ptr(),
                        std::ptr::null() as *const libc::c_char,
                    );
                }

                eprintln!("echo执行失败: {}", io::Error::last_os_error());
                unsafe { libc::exit(1) };
            }
            _ => {
                println!("父进程继续，echo子进程PID: {}", echo_pid);
            }
        }

        // 父进程关闭管道，等待子进程
        unsafe {
            close(pipe_read);
            close(pipe_write);
        }

        let mut grep_status: libc::c_int = 0;
        let mut echo_status: libc::c_int = 0;

        unsafe {
            waitpid(grep_pid, &mut grep_status, 0);
            waitpid(echo_pid, &mut echo_status, 0);
        }

        println!("\n子进程状态:");
        println!("  grep进程 (PID: {}) 退出状态: {}", grep_pid, grep_status);
        println!("  echo进程 (PID: {}) 退出状态: {}", echo_pid, echo_status);

        if grep_status == 0 {
            println!("\n✅ execlp测试成功: grep找到了匹配的模式 '12'");
        } else {
            println!("\n❌ execlp测试失败: grep未找到匹配的模式 '12'");
        }

        Ok(())
    }

    #[test]
    fn test_fork_pipe_echo_grep_execvp() -> io::Result<()> {
        use std::{
            ffi::CString,
            os::fd::{AsRawFd, FromRawFd},
        };

        use libc::{close, dup2, execvp, fork, pipe, waitpid};

        println!("=== 测试: 使用execvp实现 echo 12313 | grep 12 ===\n");
        println!("特点: 参数数组，搜索PATH，适用于动态参数的系统命令\n");

        // 1. 创建管道
        let mut pipe_fds: [libc::c_int; 2] = [0; 2];
        unsafe {
            if pipe(pipe_fds.as_mut_ptr()) == -1 {
                return Err(io::Error::last_os_error());
            }
        }

        let (pipe_read, pipe_write) = (pipe_fds[0], pipe_fds[1]);

        // 2. 第一次fork - 创建grep进程
        let grep_pid = unsafe { fork() };

        match grep_pid {
            -1 => {
                unsafe {
                    close(pipe_read);
                    close(pipe_write);
                }
                return Err(io::Error::last_os_error());
            }
            0 => {
                // grep子进程 - 使用execvp
                println!("grep子进程 (PID: {}) 启动 - 使用execvp", unsafe {
                    libc::getpid()
                });

                unsafe { close(pipe_write) };
                unsafe {
                    dup2(pipe_read, libc::STDIN_FILENO);
                    close(pipe_read);
                }

                // execvp使用参数数组并在PATH中搜索
                let grep_cmd = CString::new("grep").unwrap();
                let grep_args: [*const libc::c_char; 3] = [
                    CString::new("grep").unwrap().as_ptr(),
                    CString::new("12").unwrap().as_ptr(),
                    std::ptr::null(),
                ];

                unsafe {
                    execvp(grep_cmd.as_ptr(), grep_args.as_ptr());
                }

                eprintln!("grep执行失败: {}", io::Error::last_os_error());
                unsafe { libc::exit(1) };
            }
            _ => {
                println!("父进程继续，grep子进程PID: {}", grep_pid);
            }
        }

        // 3. 第二次fork - 创建echo进程
        let echo_pid = unsafe { fork() };

        match echo_pid {
            -1 => {
                unsafe {
                    close(pipe_read);
                    close(pipe_write);
                }
                return Err(io::Error::last_os_error());
            }
            0 => {
                // echo子进程 - 使用execvp
                println!("echo子进程 (PID: {}) 启动 - 使用execvp", unsafe {
                    libc::getpid()
                });

                unsafe { close(pipe_read) };
                unsafe {
                    dup2(pipe_write, libc::STDOUT_FILENO);
                    close(pipe_write);
                }

                // execvp使用参数数组并在PATH中搜索
                let echo_cmd = CString::new("echo").unwrap();
                let echo_args: [*const libc::c_char; 3] = [
                    CString::new("echo").unwrap().as_ptr(),
                    CString::new("12313").unwrap().as_ptr(),
                    std::ptr::null(),
                ];

                unsafe {
                    execvp(echo_cmd.as_ptr(), echo_args.as_ptr());
                }

                eprintln!("echo执行失败: {}", io::Error::last_os_error());
                unsafe { libc::exit(1) };
            }
            _ => {
                println!("父进程继续，echo子进程PID: {}", echo_pid);
            }
        }

        // 父进程关闭管道，等待子进程
        unsafe {
            close(pipe_read);
            close(pipe_write);
        }

        let mut grep_status: libc::c_int = 0;
        let mut echo_status: libc::c_int = 0;

        unsafe {
            waitpid(grep_pid, &mut grep_status, 0);
            waitpid(echo_pid, &mut echo_status, 0);
        }

        println!("\n子进程状态:");
        println!("  grep进程 (PID: {}) 退出状态: {}", grep_pid, grep_status);
        println!("  echo进程 (PID: {}) 退出状态: {}", echo_pid, echo_status);

        if grep_status == 0 {
            println!("\n✅ execvp测试成功: grep找到了匹配的模式 '12'");
        } else {
            println!("\n❌ execvp测试失败: grep未找到匹配的模式 '12'");
        }

        Ok(())
    }

    #[test]
    fn test_command_execl_style() -> io::Result<()> {
        println!("=== 测试: Command模拟execl风格 ===\n");
        println!("特点: 固定参数，完整路径，类似于execl\n");

        // 模拟execl风格：固定参数，完整路径
        let output = Command::new("/bin/echo").arg("12313").output()?;

        println!("命令执行状态: {}", output.status);
        println!("输出: {}", String::from_utf8_lossy(&output.stdout));

        // 验证结果
        let output_str = String::from_utf8_lossy(&output.stdout);
        if output_str.trim() == "12313" {
            println!("✅ execl风格测试成功");
        } else {
            println!("❌ execl风格测试失败");
        }

        Ok(())
    }

    #[test]
    fn test_command_execv_style() -> io::Result<()> {
        println!("=== 测试: Command模拟execv风格 ===\n");
        println!("特点: 动态参数数组，完整路径，类似于execv\n");

        // 模拟execv风格：动态参数数组
        let args = vec!["-l", "/tmp"];
        let output = Command::new("/bin/ls").args(&args).output()?;

        println!("命令执行状态: {}", output.status);
        println!(
            "输出行数: {}",
            String::from_utf8_lossy(&output.stdout).lines().count()
        );

        // 验证结果
        if output.status.success() {
            println!("✅ execv风格测试成功");
        } else {
            println!("❌ execv风格测试失败");
        }

        Ok(())
    }

    #[test]
    fn test_command_execlp_style() -> io::Result<()> {
        println!("=== 测试: Command模拟execlp风格 ===\n");
        println!("特点: 固定参数，PATH搜索，类似于execlp\n");

        // 模拟execlp风格：PATH搜索，固定参数
        let output = Command::new("echo").arg("Hello from Command!").output()?;

        println!("命令执行状态: {}", output.status);
        println!("输出: {}", String::from_utf8_lossy(&output.stdout));

        // 验证结果
        let output_str = String::from_utf8_lossy(&output.stdout);
        let trimmed_output = output_str.trim();
        if trimmed_output == "Hello from Command!" {
            println!("✅ execlp风格测试成功");
        } else {
            println!("❌ execlp风格测试失败");
        }

        Ok(())
    }

    #[test]
    fn test_command_execvp_style() -> io::Result<()> {
        println!("=== 测试: Command模拟execvp风格 ===\n");
        println!("特点: 动态参数数组，PATH搜索，类似于execvp\n");

        // 模拟execvp风格：PATH搜索，动态参数数组
        let args = vec!["hello", "/etc/passwd"];
        let output = Command::new("grep").args(&args).output()?;

        println!("命令执行状态: {}", output.status);

        // grep在/etc/passwd中找不到"hello"时返回1，这是正常行为
        if output.status.code() == Some(0) || output.status.code() == Some(1) {
            println!("✅ execvp风格测试成功");
            println!(
                "   退出状态 {} 是预期的（0=找到匹配，1=未找到）",
                output.status.code().unwrap()
            );
        } else {
            println!("❌ execvp风格测试失败，状态: {:?}", output.status.code());
        }

        Ok(())
    }

    #[test]
    fn test_command_pipe_echo_grep() -> io::Result<()> {
        println!("=== 测试: 使用Command实现 echo 12313 | grep 12 ===\n");

        // 使用Command的管道功能实现 echo 12313 | grep 12
        let echo_output = Command::new("echo")
            .arg("12313")
            .stdout(std::process::Stdio::piped())
            .spawn()?
            .stdout
            .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "无法获取echo输出"))?;

        let grep_output = Command::new("grep").arg("12").stdin(echo_output).output()?;

        println!("grep命令执行状态: {}", grep_output.status);
        println!("grep输出: {}", String::from_utf8_lossy(&grep_output.stdout));

        // 验证结果
        if grep_output.status.success() {
            println!("✅ Command管道测试成功: grep找到了匹配的模式 '12'");
        } else {
            println!("❌ Command管道测试失败: grep未找到匹配的模式 '12'");
        }

        Ok(())
    }

    #[test]
    fn test_command_advanced_pipe() -> io::Result<()> {
        println!("=== 测试: 使用Command实现复杂管道 ===\n");

        // 实现更复杂的管道：echo "hello world" | grep "hello" | wc -l
        let echo_process = Command::new("echo")
            .arg("hello world")
            .stdout(std::process::Stdio::piped())
            .spawn()?;

        let grep_process = Command::new("grep")
            .arg("hello")
            .stdin(echo_process.stdout.unwrap())
            .stdout(std::process::Stdio::piped())
            .spawn()?;

        let wc_output = Command::new("wc")
            .arg("-l")
            .stdin(grep_process.stdout.unwrap())
            .output()?;

        println!("wc命令执行状态: {}", wc_output.status);
        println!(
            "行数统计结果: {}",
            String::from_utf8_lossy(&wc_output.stdout).trim()
        );

        // 验证结果
        let line_count: i32 = String::from_utf8_lossy(&wc_output.stdout)
            .trim()
            .parse()
            .unwrap_or(-1);
        if line_count == 1 {
            println!("✅ 复杂管道测试成功: 正确统计了1行");
        } else {
            println!("❌ 复杂管道测试失败: 期望1行，实际{}行", line_count);
        }

        Ok(())
    }

    #[test]
    fn test_command_env_variables() -> io::Result<()> {
        println!("=== 测试: Command环境变量设置 ===\n");

        // 设置环境变量并执行命令
        let output = Command::new("env")
            .env("CUSTOM_VAR", "test_value")
            .output()?;

        let env_output = String::from_utf8_lossy(&output.stdout);
        println!("环境变量输出:");
        for line in env_output.lines() {
            if line.contains("CUSTOM_VAR") {
                println!("  {}", line);
            }
        }

        // 验证环境变量设置
        if env_output.contains("CUSTOM_VAR=test_value") {
            println!("✅ 环境变量测试成功");
        } else {
            println!("❌ 环境变量测试失败");
        }

        Ok(())
    }

    #[test]
    fn test_command_working_directory() -> io::Result<()> {
        println!("=== 测试: Command工作目录设置 ===\n");

        // 设置工作目录并执行命令
        let temp_dir = std::env::temp_dir();
        let output = Command::new("pwd").current_dir(&temp_dir).output()?;

        let pwd_output = String::from_utf8_lossy(&output.stdout).trim().to_string();
        let expected_path = temp_dir.to_string_lossy().to_string();

        println!("当前工作目录: {}", pwd_output);
        println!("期望工作目录: {}", expected_path);

        // 验证工作目录设置
        if pwd_output == expected_path {
            println!("✅ 工作目录测试成功");
        } else {
            println!("❌ 工作目录测试失败");
        }

        Ok(())
    }

    #[test]
    fn test_command_stdin_stdout_stderr() -> io::Result<()> {
        println!("=== 测试: Command标准输入输出重定向 ===\n");

        // 测试标准输入输出重定向
        let mut child = Command::new("cat")
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .spawn()?;

        // 写入数据到子进程的标准输入
        if let Some(mut stdin) = child.stdin.take() {
            stdin.write_all(b"hello from stdin\n")?;
        }

        // 读取子进程的标准输出
        let output = child.wait_with_output()?;

        println!("子进程退出状态: {}", output.status);
        println!(
            "标准输出: {}",
            String::from_utf8_lossy(&output.stdout).trim()
        );
        println!(
            "标准错误: {}",
            String::from_utf8_lossy(&output.stderr).trim()
        );

        // 验证输出
        let stdout_str = String::from_utf8_lossy(&output.stdout);
        let trimmed_stdout = stdout_str.trim();
        if trimmed_stdout == "hello from stdin" {
            println!("✅ 标准输入输出测试成功");
        } else {
            println!("❌ 标准输入输出测试失败");
        }

        Ok(())
    }

    #[test]
    fn test_command_family_comparison() -> io::Result<()> {
        println!("=== 综合测试: Command模拟exec函数家族 ===\n");

        println!("1. 测试execl风格:");
        test_command_execl_style().unwrap();

        println!("\n2. 测试execv风格:");
        test_command_execv_style().unwrap();

        println!("\n3. 测试execlp风格:");
        test_command_execlp_style().unwrap();

        println!("\n4. 测试execvp风格:");
        test_command_execvp_style().unwrap();

        println!("\n5. 测试管道功能:");
        test_command_pipe_echo_grep().unwrap();

        println!("\n6. 测试复杂管道:");
        test_command_advanced_pipe().unwrap();

        println!("\n7. 测试环境变量:");
        test_command_env_variables().unwrap();

        println!("\n8. 测试工作目录:");
        test_command_working_directory().unwrap();

        println!("\n9. 测试标准输入输出:");
        test_command_stdin_stdout_stderr().unwrap();

        println!("\n=== Command模拟exec函数家族测试完成 ===");
        Ok(())
    }

    #[test]
    fn test_exec_family_comparison_pipe() -> io::Result<()> {
        println!("=== 测试: exec函数家族在管道通信中的对比 ===\n");

        println!("1. 测试execl版本:");
        test_fork_pipe_echo_grep().unwrap();

        println!("\n2. 测试execv版本:");
        test_fork_pipe_echo_grep_execv().unwrap();

        println!("\n3. 测试execlp版本:");
        test_fork_pipe_echo_grep_execlp().unwrap();

        println!("\n4. 测试execvp版本:");
        test_fork_pipe_echo_grep_execvp().unwrap();

        println!("\n=== 所有exec函数家族测试完成 ===");
        Ok(())
    }
}
