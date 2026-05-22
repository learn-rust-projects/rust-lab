use std::time::Duration;

use futures::stream::{FuturesUnordered, StreamExt};
use tokio::time::sleep;

/// FuturesUnordered 基本使用示例 - 按完成顺序返回结果
#[tokio::main]
async fn main() {
    println!("=== FuturesUnordered基本使用示例 ===");

    async fn task(id: u32, duration_ms: u64) -> String {
        sleep(Duration::from_millis(duration_ms)).await;
        format!("任务{}完成 - 耗时{}ms", id, duration_ms)
    }

    let mut futures = FuturesUnordered::new();
    futures.push(task(1, 100));
    futures.push(task(2, 50)); // 这个会先完成
    futures.push(task(3, 150));
    futures.push(task(4, 30)); // 这个会最先完成

    println!("开始并发执行任务...");

    let mut results = Vec::new();
    while let Some(result) = futures.next().await {
        println!("收到结果: {}", result);
        results.push(result);
    }

    println!("所有任务完成！结果顺序: {:?}", results);
}
