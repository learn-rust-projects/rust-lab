use tokio::sync::watch;
use tokio_stream::{StreamExt, wrappers::WatchStream};

#[tokio::main]
async fn main() {
    let (tx, rx) = watch::channel("初始值");
    let mut stream = WatchStream::new(rx);

    tx.send("更新1").unwrap();
    tx.send("更新2").unwrap();
    drop(tx);

    while let Some(value) = stream.next().await {
        println!("Watch值: {}", value);
    }
}
