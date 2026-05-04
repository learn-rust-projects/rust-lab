use std::time::Duration;

use tokio_stream::{StreamExt, wrappers::IntervalStream};

#[tokio::main]
async fn main() {
    let interval = tokio::time::interval(Duration::from_millis(70));
    let mut stream = IntervalStream::new(interval)
        .timeout_repeating(tokio::time::interval(Duration::from_millis(30)));

    for _ in 0..3 {
        tokio::select! {
            Some(v) = stream.next() => {
                println!("收到值: {}", v.is_ok());
            }
            _ = tokio::time::sleep(Duration::from_millis(20)) => {
                println!("超时");
            }
        }
    }
}
