use std::time::Duration;

use futures::stream::{FuturesUnordered, StreamExt};
use futures::FutureExt;
use tokio::time::sleep;

/// FuturesUnordered与错误处理
#[tokio::main]
async fn main() {
    println!("=== FuturesUnordered与错误处理 ===");

    async fn successful_task(id: u32) -> Result<String, &'static str> {
        sleep(Duration::from_millis(id as u64 * 20)).await;
        Ok(format!("成功任务{}", id))
    }

    async fn failing_task(id: u32) -> Result<String, &'static str> {
        sleep(Duration::from_millis(id as u64 * 10)).await;
        Err("任务失败")
    }

    let mut futures = FuturesUnordered::new();
    futures.push(successful_task(1).boxed());
    futures.push(failing_task(2).boxed()); // 这个会先完成并返回错误
    futures.push(successful_task(3).boxed());

    let mut success_count = 0;
    let mut error_count = 0;

    while let Some(result) = futures.next().await {
        match result {
            Ok(msg) => {
                println!("成功: {}", msg);
                success_count += 1;
            }
            Err(e) => {
                println!("错误: {}", e);
                error_count += 1;
            }
        }
    }

    println!("统计: 成功{}个, 失败{}个", success_count, error_count);
}
