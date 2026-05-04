use tokio_stream::{StreamExt, wrappers::ReceiverStream};
use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    let (tx, rx) = mpsc::channel(10);
    let mut stream = ReceiverStream::new(rx);

    tx.send("消息1").await.unwrap();
    tx.send("消息2").await.unwrap();
    drop(tx);

    while let Some(msg) = stream.next().await {
        println!("Receiver消息: {}", msg);
    }
}