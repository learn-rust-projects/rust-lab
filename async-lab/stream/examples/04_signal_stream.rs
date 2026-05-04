use tokio::{
    signal::unix::{SignalKind, signal},
    time::Duration,
};
use tokio_stream::{StreamExt, wrappers::SignalStream};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let stream = signal(SignalKind::interrupt())?;
    let mut stream = SignalStream::new(stream);

    println!("等待Ctrl+C信号 (3秒超时)...",);

    match tokio::time::timeout(Duration::from_secs(3), stream.next()).await {
        Ok(Some(_)) => println!("收到Ctrl+C信号!"),
        Ok(None) => println!("流已关闭"),
        Err(_) => println!("超时，未收到信号"),
    }
    Ok(())
}