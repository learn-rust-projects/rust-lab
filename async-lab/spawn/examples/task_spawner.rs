//! 使用join_all和task::spawn并发执行多个异步任务的示例
//!
//! 这个示例展示了如何使用tokio的spawn功能来创建并发任务，
//! 并使用join_all来等待所有任务完成。

use std::time::Duration;

use futures::future::join_all;
use tokio::{task, time::sleep};

/// 模拟一个异步任务
async fn my_task(duration: Duration) -> String {
    println!("任务开始执行，预计耗时: {:?}", duration);
    sleep(duration).await;
    let result = format!("任务完成，耗时: {:?}", duration);
    println!("{}", result);
    result
}

/// 使用task::spawn和join_all来并发执行多个任务
async fn task_spawner() {
    println!("=== 开始并发任务执行 ===");

    // 创建多个并发任务
    let tasks = vec![
        task::spawn(my_task(Duration::from_secs(1))),
        task::spawn(my_task(Duration::from_secs(2))),
        task::spawn(my_task(Duration::from_secs(3))),
    ];

    println!("已创建 {} 个并发任务", tasks.len());

    // 使用join_all等待所有任务完成
    // 如果不等待这些任务，函数结束时它们会被丢弃
    let results = join_all(tasks).await;

    println!("\n=== 所有任务完成 ===");

    // 处理任务结果
    for (i, result) in results.into_iter().enumerate() {
        match result {
            Ok(msg) => println!("任务 {} 成功: {}", i + 1, msg),
            Err(e) => println!("任务 {} 失败: {:?}", i + 1, e),
        }
    }
}

/// 对比示例：顺序执行 vs 并发执行
async fn compare_execution_modes() {
    println!("\n=== 顺序执行 vs 并发执行对比 ===");

    // 顺序执行
    let start_sequential = std::time::Instant::now();

    my_task(Duration::from_secs(1)).await;
    my_task(Duration::from_secs(2)).await;
    my_task(Duration::from_secs(3)).await;

    let sequential_duration = start_sequential.elapsed();
    println!("顺序执行总耗时: {:?}", sequential_duration);

    // 并发执行
    let start_concurrent = std::time::Instant::now();

    let tasks = vec![
        task::spawn(my_task(Duration::from_secs(1))),
        task::spawn(my_task(Duration::from_secs(2))),
        task::spawn(my_task(Duration::from_secs(3))),
    ];

    join_all(tasks).await;

    let concurrent_duration = start_concurrent.elapsed();
    println!("并发执行总耗时: {:?}", concurrent_duration);

    println!(
        "性能提升: {:.2}x",
        sequential_duration.as_secs_f64() / concurrent_duration.as_secs_f64()
    );
}

/// 动态任务创建示例
async fn dynamic_task_creation() {
    println!("\n=== 动态任务创建示例 ===");

    let mut tasks = Vec::new();

    // 动态创建任务
    for i in 0..5 {
        let duration = Duration::from_secs(i as u64 + 1);
        let task = task::spawn(my_task(duration));
        tasks.push(task);
        println!("创建任务 {}，耗时: {:?}", i + 1, duration);
    }

    // 等待所有任务完成
    let results = join_all(tasks).await;

    println!("动态创建的所有任务完成");

    // 统计成功和失败的任务
    let success_count = results.iter().filter(|r| r.is_ok()).count();
    let error_count = results.iter().filter(|r| r.is_err()).count();

    println!("成功: {}，失败: {}", success_count, error_count);
}

/// 错误处理示例
async fn error_handling_example() {
    println!("\n=== 错误处理示例 ===");

    async fn task_with_error(id: u32) -> Result<String, &'static str> {
        sleep(Duration::from_secs(1)).await;

        if id % 2 == 0 {
            Ok(format!("任务 {} 成功", id))
        } else {
            Err("任务失败")
        }
    }

    let tasks = vec![
        task::spawn(task_with_error(1)),
        task::spawn(task_with_error(2)),
        task::spawn(task_with_error(3)),
        task::spawn(task_with_error(4)),
    ];

    let results = join_all(tasks).await;

    for (i, result) in results.into_iter().enumerate() {
        match result {
            Ok(inner_result) => match inner_result {
                Ok(msg) => println!("任务 {}: ✅ {}", i + 1, msg),
                Err(e) => println!("任务 {}: ❌ {}", i + 1, e),
            },
            Err(e) => println!("任务 {}: 💥 任务执行失败: {:?}", i + 1, e),
        }
    }
}

/// 任务未被等待时的行为示例
async fn unawaited_tasks_behavior() {
    println!("\n=== 任务未被等待时的行为示例 ===");
    println!("这个示例展示当任务未被等待时会发生什么情况");

    // 创建一个长时间运行的任务（5秒）
    let long_task = task::spawn(async {
        println!("长时间任务开始执行...");
        sleep(Duration::from_secs(5)).await;
        println!("长时间任务完成！");
        "长时间任务结果"
    });

    // 创建一个短时间任务（1秒）
    let short_task = task::spawn(async {
        println!("短时间任务开始执行...");
        sleep(Duration::from_secs(1)).await;
        println!("短时间任务完成！");
        "短时间任务结果"
    });

    println!("已创建两个任务，但只等待短时间任务...");

    // 只等待短时间任务，不等待长时间任务
    match short_task.await {
        Ok(result) => println!("短时间任务结果: {}", result),
        Err(e) => println!("短时间任务失败: {:?}", e),
    }

    println!("短时间任务已完成，但长时间任务仍在运行...");
    println!("函数即将结束，长时间任务将被取消...");

    // 注意：这里我们没有等待long_task，所以当函数结束时，
    // 长时间任务将被丢弃（取消）
    // 程序将继续执行而不等待长时间任务完成
}

/// 手动取消任务示例
async fn manual_task_cancellation() {
    println!("\n=== 手动取消任务示例 ===");
    println!("这个示例展示如何在函数完成前手动取消任务");

    // 创建一个可取消的任务
    let (cancel_tx, mut cancel_rx) = tokio::sync::oneshot::channel();

    let cancellable_task = task::spawn(async move {
        println!("可取消任务开始执行...");

        // 使用select!来同时等待任务完成和取消信号
        tokio::select! {
            _ = sleep(Duration::from_secs(10)) => {
                println!("可取消任务正常完成！");
                "任务正常完成"
            }
            _ = &mut cancel_rx => {
                println!("可取消任务被取消！");
                "任务被取消"
            }
        }
    });

    // 等待一段时间（2秒）
    sleep(Duration::from_secs(2)).await;
    println!("等待2秒后，发送取消信号...");

    // 发送取消信号
    let _ = cancel_tx.send(());

    // 等待任务完成（无论是否被取消）
    match cancellable_task.await {
        Ok(result) => println!("任务最终结果: {}", result),
        Err(e) => println!("任务执行失败: {:?}", e),
    }

    println!("手动取消示例完成");
}

#[tokio::main]
async fn main() {
    println!("开始task_spawner示例...");

    // 运行基本示例
    task_spawner().await;

    // 运行对比示例
    compare_execution_modes().await;

    // 运行动态任务创建示例
    dynamic_task_creation().await;

    // 运行错误处理示例
    error_handling_example().await;

    // 运行任务未被等待时的行为示例
    unawaited_tasks_behavior().await;

    // 运行手动取消任务示例
    manual_task_cancellation().await;

    println!("\n🎉 所有示例执行完成！");
}

#[cfg(test)]
mod tests {
    use tokio::time::timeout;

    use super::*;

    #[tokio::test]
    async fn test_my_task() {
        let result = my_task(Duration::from_millis(10)).await;
        assert!(result.contains("任务完成"));
    }

    #[tokio::test]
    async fn test_task_spawner_basic() {
        // 测试基本功能，设置超时避免无限等待
        let result = timeout(Duration::from_secs(5), task_spawner()).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_join_all_behavior() {
        let tasks = vec![
            task::spawn(my_task(Duration::from_millis(50))),
            task::spawn(my_task(Duration::from_millis(100))),
        ];

        let results = join_all(tasks).await;
        assert_eq!(results.len(), 2);

        for result in results {
            assert!(result.is_ok());
        }
    }

    #[tokio::test]
    async fn test_error_handling() {
        async fn failing_task() -> Result<String, &'static str> {
            Err("测试错误")
        }

        let task = task::spawn(failing_task());
        let results = join_all(vec![task]).await;

        assert_eq!(results.len(), 1);
        if let Ok(inner_result) = &results[0] {
            assert!(inner_result.is_err());
        }
    }

    #[tokio::test]
    async fn test_concurrent_vs_sequential() {
        // 验证并发执行确实比顺序执行快
        let start = std::time::Instant::now();

        let tasks = vec![
            task::spawn(my_task(Duration::from_millis(100))),
            task::spawn(my_task(Duration::from_millis(100))),
        ];

        join_all(tasks).await;
        let concurrent_duration = start.elapsed();

        // 并发执行应该小于200ms（两个100ms任务的顺序执行时间）
        assert!(concurrent_duration < Duration::from_millis(200));
    }

    #[tokio::test]
    async fn test_unawaited_tasks_behavior() {
        // 测试任务未被等待时的行为
        let result = timeout(Duration::from_secs(3), unawaited_tasks_behavior()).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_manual_cancellation() {
        // 测试手动取消功能
        let result = timeout(Duration::from_secs(5), manual_task_cancellation()).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_task_cancellation_behavior() {
        // 验证任务确实会被取消
        let task = task::spawn(async {
            sleep(Duration::from_secs(10)).await;
            "这个任务应该被取消"
        });

        // 立即丢弃任务（模拟函数结束）
        drop(task);

        // 等待一小段时间确保任务被取消
        sleep(Duration::from_millis(100)).await;

        // 任务应该已经被取消，无法获取结果
        // 这个测试主要验证任务取消机制正常工作
    }
}
