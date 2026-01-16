use std::fs;

use async_std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    prelude::*,
    task::spawn,
};
use futures::stream::StreamExt;
#[async_std::main]
async fn main() {
    // Listen for incoming TCP connections on localhost port 7878
    let listener = TcpListener::bind("127.0.0.1:7878").await.unwrap();

    // Block forever, handling each request that arrives at this IP address
    // 将 listener.incoming() 从阻塞式迭代器转换为非阻塞式流
    // 流与迭代器类似，但可以异步使用。
    listener
    // 异步版本的 TcpListener 为 listener.incoming() 实现了 Stream trait
        .incoming()
        // 可以使用 Stream 的 ` for_each_concurrent 方法选择性地并发处理 Stream 中的元素
        .for_each_concurrent(None, |stream| async move {
            let stream = stream.unwrap();
            // 每个连接都在独立的任务中处理，因此可以并发处理多个连接
            // 多工位：async-std 会在后台维持几个工作线程（通常数量等于你的 CPU 核心数）。
            // 负载均衡：新任务会被分配到目前最闲的那个线程上。
            // 真·并行：如果你的电脑是 4 核的，那么可能有 4 个连接是**物理上同时（Parallelism）**在运行的，而不是轮流排队。
            spawn(handle_connection(stream));
        })
        .await;
}

async fn handle_connection(mut stream: impl Read + Write + Unpin) {
    // Read the first 1024 bytes of data from the stream
    let mut buffer = [0; 1024];
    let _ = stream.read(&mut buffer).await.unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";
    // Respond with greetings or a 404,
    // depending on the data in the request
    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else if buffer.starts_with(sleep) {
        async_std::task::sleep(std::time::Duration::from_secs(5)).await;
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };
    let contents = fs::read_to_string(filename).unwrap();

    // Write response back to the stream,
    // and flush the stream to ensure the response is sent back to the client
    let response = format!("{status_line}{contents}");
    stream.write_all(response.as_bytes()).await.unwrap();
    stream.flush().await.unwrap();
}

use std::{cmp::min, pin::Pin};

use futures::{
    io::Error,
    task::{Context, Poll},
};

#[cfg(test)]
mod tests {

    use std::{cmp::min, pin::Pin};

    use futures::{
        io::Error,
        task::{Context, Poll},
    };

    use super::*;

    struct MockTcpStream {
        read_data: Vec<u8>,
        write_data: Vec<u8>,
    }
    impl Read for MockTcpStream {
        fn poll_read(
            self: Pin<&mut Self>,
            _cx: &mut Context<'_>,
            buf: &mut [u8],
        ) -> Poll<Result<usize, Error>> {
            let len = min(buf.len(), self.read_data.len());
            buf[..len].copy_from_slice(&self.read_data[..len]);
            Poll::Ready(Ok(len))
        }
    }
    impl Write for MockTcpStream {
        fn poll_write(
            mut self: Pin<&mut Self>,
            _: &mut Context,
            buf: &[u8],
        ) -> Poll<Result<usize, Error>> {
            self.write_data = Vec::from(buf);
            Poll::Ready(Ok(buf.len()))
        }

        fn poll_flush(self: Pin<&mut Self>, _: &mut Context) -> Poll<Result<(), Error>> {
            Poll::Ready(Ok(()))
        }

        fn poll_close(self: Pin<&mut Self>, _: &mut Context) -> Poll<Result<(), Error>> {
            Poll::Ready(Ok(()))
        }
    }
    impl Unpin for MockTcpStream {}
    use std::fs;

    #[async_std::test]
    async fn test_handle_connection() {
        let input_bytes = b"GET / HTTP/1.1\r\n";
        let mut contents = vec![0u8; 1024];
        contents[..input_bytes.len()].clone_from_slice(input_bytes);
        let mut stream = MockTcpStream {
            read_data: contents,
            write_data: Vec::new(),
        };
        

        handle_connection(&mut stream).await;

        let expected_contents = fs::read_to_string("hello.html").unwrap();
        let expected_response = format!("HTTP/1.1 200 OK\r\n\r\n{}", expected_contents);
        assert!(stream.write_data.starts_with(expected_response.as_bytes()));
    }
}
