use std::time::Duration;

use futures::stream::{FuturesUnordered, StreamExt};
use tokio::time::sleep;

/// FuturesUnordered动态添加任务
#[tokio::main]
async fn main() {
    println!("=== FuturesUnordered动态添加任务 ===");

    async fn dynamic_task(id: u32) -> String {
        let duration = (id * 10) as u64;
        sleep(Duration::from_millis(duration)).await;
        format!("动态任务{} - 耗时{}ms", id, duration)
    }

    let mut futures = FuturesUnordered::new();
    let mut completed_count = 0;

    for i in 1..=3 {
        futures.push(dynamic_task(i));
    }

    println!("开始执行，初始3个任务...");

    while completed_count < 10 {
        tokio::select! {
            Some(result) = futures.next() => {
                println!("完成: {}", result);
                completed_count += 1;

                if completed_count < 10 {
                    let new_id = completed_count + 3;
                    futures.push(dynamic_task(new_id));
                    println!("添加新任务: {}", new_id);
                }
            }
            _ = sleep(Duration::from_millis(500)) => {
                println!("当前活跃任务数: {}", futures.len());
            }
        }
    }

    println!("动态任务执行完成！");
}
