use tokio::time::Duration;
use tokio_stream::{StreamExt, wrappers::IntervalStream};

#[tokio::main]
async fn main() {
    let mut interval = IntervalStream::new(tokio::time::interval(Duration::from_millis(200)));

    println!("IntervalStream: 定时器事件");
    for i in 0..5 {
        if let Some(_) = interval.next().await {
            println!("事件 #{}", i);
        }
    }
}
