use tokio::{io::AsyncWriteExt, net::TcpListener, time::Duration};
use tokio_stream::StreamExt;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    println!("监听: {}", addr);

    let mut stream = tokio_stream::wrappers::TcpListenerStream::new(listener);

    let server = tokio::spawn(async move {
        match tokio::time::timeout(Duration::from_millis(200), stream.next()).await {
            Ok(Some(Ok(mut socket))) => {
                let addr = socket.peer_addr().unwrap();
                println!("收到连接: {}", addr);
                socket.write_all(b"Hello").await.unwrap();
            }
            Ok(Some(Err(e))) => println!("错误: {}", e),
            Ok(None) => println!("监听关闭"),
            Err(_) => println!("超时"),
        }
    });

    let client = tokio::spawn(async move {
        tokio::time::sleep(Duration::from_millis(50)).await;
        let mut stream = tokio::net::TcpStream::connect(addr).await.unwrap();
        let mut buf = vec![0; 5];
        tokio::io::AsyncReadExt::read(&mut stream, &mut buf)
            .await
            .unwrap();
        println!("收到响应: {:?}", String::from_utf8_lossy(&buf));
    });

    let _ = tokio::join!(server, client);
}
