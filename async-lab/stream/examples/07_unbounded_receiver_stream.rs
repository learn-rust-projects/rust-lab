use tokio_stream::{StreamExt, wrappers::UnboundedReceiverStream};
use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    let (tx, rx) = mpsc::unbounded_channel();
    let mut stream = UnboundedReceiverStream::new(rx);

    tx.send("消息A").unwrap();
    tx.send("消息B").unwrap();
    drop(tx);

    while let Some(msg) = stream.next().await {
        println!("UnboundedReceiver: {}", msg);
    }
}