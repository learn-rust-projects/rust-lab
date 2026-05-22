use std::time::Duration;

use futures::stream::{FuturesUnordered, StreamExt};
use tokio::time::sleep;

/// FuturesUnordered与Stream结合
#[tokio::main]
async fn main() {
    println!("=== FuturesUnordered与Stream结合 ===");

    async fn process_item(item: u32) -> u32 {
        sleep(Duration::from_millis(item as u64 * 10)).await;
        item * 2
    }

    let data_stream = futures::stream::iter(1..=10);
    let mut processing_futures = FuturesUnordered::new();

    let mut processed_results = Vec::new();
    let mut data_stream = Box::pin(data_stream);

    loop {
        tokio::select! {
            Some(item) = data_stream.next() => {
                println!("开始处理项目: {}", item);
                processing_futures.push(process_item(item));
            }
            Some(result) = processing_futures.next() => {
                println!("处理完成: {}", result);
                processed_results.push(result);
            }
            else => {
                if processing_futures.is_empty() {
                    break;
                }
            }
        }
    }

    println!("流处理完成！结果: {:?}", processed_results);
}
