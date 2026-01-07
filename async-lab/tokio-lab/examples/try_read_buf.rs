use std::io;

use bytes::BytesMut;
use tokio::{io::Interest, net::TcpStream};

#[tokio::main]
async fn main() -> io::Result<()> {
    // 1. 建立连接
    let stream = TcpStream::connect("127.0.0.1:8080").await?;

    // 2. 准备一个可自动扩容的缓冲区 (来自 bytes crate)
    // 必须引入 features = ["net"] 和依赖 bytes
    let mut buf = BytesMut::with_capacity(1024);

    loop {
        // 3. 等待 Socket 变为“可读”状态
        stream.ready(Interest::READABLE).await?;

        // 4. 尝试非阻塞读取
        // try_read_buf 会自动根据 buf 的剩余容量进行读取
        match stream.try_read_buf(&mut buf) {
            Ok(0) => {
                println!("连接已关闭");
                break;
            }
            Ok(n) => {
                println!("成功读取了 {} 字节", n);
                // 这里可以处理 buf 中的数据
                // 例如：let data = buf.split().freeze();
            }
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                // 虽然 ready 说可读，但可能被其他任务抢占了数据
                // 或者是虚假唤醒，直接继续循环即可
                continue;
            }
            Err(e) => {
                return Err(e);
            }
        }
    }

    Ok(())
}
