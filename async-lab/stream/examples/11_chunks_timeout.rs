use futures::stream;
use tokio::time::{Duration, interval};
use tokio_stream::{StreamExt, StreamExt as _, wrappers::IntervalStream};

#[tokio::main]
async fn main() {
    // 模拟“慢速数据流”（每 60ms 来一个数据）
    let base_stream = stream::iter(vec![1, 2, 3, 4, 5, 6]).throttle(Duration::from_millis(60));

    let chunks = base_stream.chunks_timeout(2, Duration::from_millis(100));

    let result: Vec<Vec<_>> = chunks.collect().await;

    println!("ChunksTimeout result: {:?}", result);
}
