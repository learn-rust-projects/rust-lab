use std::time::Duration;

use futures::stream::{FuturesUnordered, StreamExt};
use tokio::time::sleep;

/// FuturesUnordered性能对比 - 并发vs顺序
#[tokio::main]
async fn main() {
    println!("=== FuturesUnordered性能对比 ===");

    async fn cpu_intensive_task(id: u32) -> u32 {
        let mut result = 0;
        for i in 0..1000 {
            result += i * id;
        }
        sleep(Duration::from_millis(50)).await;
        result
    }

    // 并发执行
    let start_concurrent = std::time::Instant::now();
    let mut concurrent_futures = FuturesUnordered::new();

    for i in 1..=10 {
        concurrent_futures.push(cpu_intensive_task(i));
    }

    let mut concurrent_results = Vec::new();
    while let Some(result) = concurrent_futures.next().await {
        concurrent_results.push(result);
    }

    let concurrent_duration = start_concurrent.elapsed();

    // 顺序执行
    let start_sequential = std::time::Instant::now();
    let mut sequential_results = Vec::new();

    for i in 1..=10 {
        sequential_results.push(cpu_intensive_task(i).await);
    }

    let sequential_duration = start_sequential.elapsed();

    println!("并发执行耗时: {:?}", concurrent_duration);
    println!("顺序执行耗时: {:?}", sequential_duration);
    println!(
        "性能提升: {:.2}x",
        sequential_duration.as_secs_f64() / concurrent_duration.as_secs_f64()
    );
}
