use tokio::{sync::broadcast, time::Duration};
use tokio_stream::{StreamExt, wrappers::BroadcastStream};

#[tokio::main]
async fn main() {
    let (tx, rx) = broadcast::channel(10);
    let mut stream = BroadcastStream::new(rx);

    tx.send("广播1").unwrap();
    tx.send("广播2").unwrap();
    drop(tx);

    tokio::select! {
        _ = async {
            while let Some(result) = stream.next().await {
                println!("广播: {}", result.unwrap());
            }
        } => {}
        _ = tokio::time::sleep(Duration::from_millis(100)) => {}
    }
}
