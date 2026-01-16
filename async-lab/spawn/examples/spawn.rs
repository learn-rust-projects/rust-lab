use async_std::{
    net::{TcpListener, TcpStream},
    task,
};
use futures::AsyncWriteExt;

async fn process_request(stream: &mut TcpStream) -> Result<(), std::io::Error> {
    println!("处理新连接...");
    stream.write_all(b"HTTP/1.1 200 OK\r\n\r\n").await?;
    stream.write_all(b"Hello World").await?;
    stream.flush().await?;
    stream.close().await?;
    Ok(())
}
#[async_std::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    loop {
        println!("等待新连接...");
        // Accept a new connection
        let (mut stream, _) = listener.accept().await.unwrap();
        // Now process this request without blocking the main loop
        task::spawn(async move { process_request(&mut stream).await });
    }
}
