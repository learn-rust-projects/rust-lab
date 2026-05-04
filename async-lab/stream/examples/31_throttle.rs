use tokio_stream::StreamExt;
use std::time::Duration;

#[tokio::main]
async fn main() {
    let stream = tokio_stream::iter(vec!["a", "b", "c"]).throttle(Duration::from_millis(100));

    for item in stream.collect::<Vec<_>>().await {
        println!("Throttle: {:?}", item);
    }
}