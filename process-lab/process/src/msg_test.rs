#[cfg(test)]
mod tests {
    use libc::{
        self,
        IPC_CREAT,
        IPC_RMID, // IPC 控制标志
        O_RDONLY, // 文件操作标志 (虽然通常不用O_RDONLY来定义权限)
        c_long,
        ipc_perm,
        key_t,
        msgctl,
        msgget,
        msgrcv,
        msgsnd, // 核心函数和类型
    };

    // 修正: MSG_R 和 MSG_W 不在 libc 根目录。
    // 我们直接使用八进制数字常量定义权限，这是System V IPC的常见做法。
    // 0o666 = 0o600 (User R/W) | 0o060 (Group R/W) | 0o006 (Other R/W)
    const IPC_PERMS: i32 = 0o666;

    use std::{
        ffi::c_void,
        io::{self, Write},
        mem, process, thread, time,
    };

    // --- 1. 定义消息结构体 ---
    #[repr(C)]
    struct Message {
        mtype: c_long,
        mtext: [u8; 128],
    }

    // 消息队列的唯一标识键和类型
    const QUEUE_KEY: key_t = 1234;
    const MESSAGE_TYPE: c_long = 1;

    // 辅助函数：打印并立即刷新
    fn log_info(message: &str) {
        let _ = writeln!(io::stdout(), "{}", message);
        let _ = io::stdout().flush();
    }

    #[test]
    fn message_queue_ipc_test() {
        // --- 1. 创建消息队列 ---
        log_info("\n[Parent] 尝试创建消息队列...");
        // 使用 msgget 创建或获取消息队列
        let msgid = unsafe {
            // IPC_CREAT: 如果不存在则创建 | IPC_PERMS: 读写权限 0o666
            msgget(QUEUE_KEY, IPC_CREAT | IPC_PERMS)
        };

        if msgid == -1 {
            panic!("[Error] msgget failed: {}", io::Error::last_os_error());
        }
        log_info(&format!("[Parent] 消息队列ID: {}", msgid));

        // --- 2. Fork 进程 ---
        log_info("[Parent] Forking 子进程...");
        let pid = unsafe { libc::fork() };

        match pid {
            -1 => {
                // Fork 失败，清理并退出
                unsafe { msgctl(msgid, IPC_RMID, std::ptr::null_mut()) };
                panic!("[Error] Fork failed: {}", io::Error::last_os_error());
            }
            0 => {
                // --- 子进程逻辑 (接收者) ---
                child_process(msgid);
                process::exit(0);
            }
            _ => {
                // --- 父进程逻辑 (发送者/协调者) ---

                // 等待子进程启动 (非严格必要，但确保子进程已开始阻塞等待)
                thread::sleep(time::Duration::from_millis(100));
                parent_process(msgid);

                // 等待子进程结束
                log_info(&format!("[Parent] 等待子进程 {} 结束...", pid));
                let mut status = 0;
                unsafe { libc::waitpid(pid, &mut status, 0) };

                // --- 4. 清理消息队列 ---
                log_info("[Parent] 清理消息队列...");
                if unsafe { msgctl(msgid, IPC_RMID, std::ptr::null_mut()) } == -1 {
                    panic!(
                        "[Error] msgctl (IPC_RMID) failed: {}",
                        io::Error::last_os_error()
                    );
                }
                log_info("[Parent] 测试成功完成。");
            }
        }
    }

    // --- 子进程：接收消息 ---
    fn child_process(msgid: i32) {
        let mut recv_msg: Message = unsafe { mem::zeroed() };
        // 消息大小是总大小减去 c_long (mtype) 的大小
        let msg_size = mem::size_of::<Message>() - mem::size_of::<c_long>();

        log_info("[Child] 阻塞等待接收消息...");

        let received_bytes = unsafe {
            // 0: 接收所有类型的消息 | 0: 阻塞
            // 解释：
            // - 第一个参数 msgid 是消息队列的 ID，用于标识要操作的队列。
            // - 第二个参数 &mut recv_msg as *mut _ as *mut c_void 是接收消息的缓冲区，
            //   它需要是一个指向 c_void 的指针，因为 msgrcv 是一个 C 函数，
            //   它可以接收任意类型的指针。
            // - 第三个参数 msg_size 是要接收的消息数据的大小（不包括 mtype）。
            // - 第四个参数 0 是接收的消息类型，0 表示接收所有类型的消息。
            // - 第五个参数 0 表示阻塞等待，即如果队列中没有消息，子进程会一直阻塞在这里，
            //   直到有消息到达。
            msgrcv(
                msgid,
                &mut recv_msg as *mut _ as *mut c_void,
                msg_size,
                0,
                0, // flags: 0 表示阻塞等待
            )
        };

        if received_bytes == -1 {
            log_info(&format!(
                "[Child] msgrcv failed: {}",
                io::Error::last_os_error()
            ));
            return;
        }

        // 验证和处理接收到的字符串
        let received_str = String::from_utf8_lossy(&recv_msg.mtext[..received_bytes as usize]);
        log_info(&format!(
            "[Child] 成功接收：类型={}，内容='{}'",
            recv_msg.mtype,
            received_str.trim_end_matches('\0')
        ));

        // 断言验证
        assert_eq!(recv_msg.mtype, MESSAGE_TYPE, "接收到的消息类型错误");
        assert_eq!(
            received_str.trim_end_matches('\0'),
            "Hello from Parent",
            "接收到的消息内容错误"
        );
    }

    // --- 父进程：发送消息 ---
    fn parent_process(msgid: i32) {
        let payload = "Hello from Parent";
        // // 常规 Rust 方式（更安全但更繁琐）：
        // let mut send_msg = Message {
        //     mtype: 0,
        //     mtext: [0; 128], // 必须手动初始化数组的每个元素
        // };
        // // 这种写法是安全的，但当结构体非常大或字段很多时，使用 mem::zeroed()
        // // 可以节省代码量。
        let mut send_msg: Message = unsafe { mem::zeroed() };

        // 设置消息类型
        send_msg.mtype = MESSAGE_TYPE;

        // 拷贝消息内容
        let payload_bytes = payload.as_bytes();
        send_msg.mtext[..payload_bytes.len()].copy_from_slice(payload_bytes);

        let msg_size = mem::size_of::<Message>() - mem::size_of::<c_long>();

        log_info("[Parent] 正在发送消息...");

        if unsafe {
            // 解释：
            // - 第一个参数 msgid 是消息队列的 ID，用于标识要
            // - 第二个参数 &send_msg as *const _ as *const c_void 是要发送的消息指针，
            //   它需要是一个指向 c_void 的指针，因为 msgsnd 是一个 C 函数，
            //   它可以发送任意类型的指针。
            // - 第三个参数 msg_size 是要发送的消息数据的大小（不包括 mtype）。
            // - 第四个参数 0 表示阻塞直到发送成功，即父进程会一直阻塞在这里，
            //   直到消息成功发送到队列。
            // 消息类型是内嵌在消息结构体内部的，而不是作为单独的参数传递给
            // $\text{msgsnd}$。
            msgsnd(
                msgid,
                &send_msg as *const _ as *const c_void,
                msg_size,
                0, // flags: 0 表示阻塞直到发送成功
            )
        } == -1
        {
            panic!("[Parent] msgsnd failed: {}", io::Error::last_os_error());
        }

        log_info("[Parent] 消息发送完成 (父进程继续执行)。");
    }
}
