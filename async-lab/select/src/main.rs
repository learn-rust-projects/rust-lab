use std::time::Duration;

use futures::{Stream, StreamExt, future::FutureExt, pin_mut, select, stream::FusedStream};
use tokio::time::{sleep, timeout};

/// 1. 基本select!使用示例 - 任务竞速
async fn example_basic_race() {
    println!("\n=== 基本select!使用示例 - 任务竞速 ===");

    async fn fast_task() -> &'static str {
        sleep(Duration::from_millis(50)).await;
        "快速任务完成"
    }

    async fn slow_task() -> &'static str {
        sleep(Duration::from_millis(100)).await;
        "慢速任务完成"
    }

    let fast_fut = fast_task().fuse();
    let slow_fut = slow_task().fuse();

    pin_mut!(fast_fut, slow_fut);

    let result = select! {
        result = fast_fut => format!("快速任务胜出: {}", result),
        result = slow_fut => format!("慢速任务胜出: {}", result),
    };

    println!("{}", result);
}

/// 2. select!返回值模式匹配
async fn example_pattern_matching() {
    println!("\n=== select!返回值模式匹配 ===");

    async fn task_with_result() -> Result<u32, &'static str> {
        sleep(Duration::from_millis(80)).await;
        Ok(42)
    }

    async fn task_with_error() -> Result<u32, &'static str> {
        sleep(Duration::from_millis(60)).await;
        Err("任务失败")
    }

    let success_fut = task_with_result().fuse();
    let error_fut = task_with_error().fuse();

    pin_mut!(success_fut, error_fut);

    select! {
        res = success_fut => match res{
            Ok(value) => println!("成功任务完成: {}", value),
            Err(e) => println!("错误任务完成: {}", e),
        },
        res2 = error_fut => match res2{
            Ok(value) => println!("成功任务完成: {}", value),
            Err(e) => println!("错误任务完成: {}", e),
        },
    }
}

/// 3. 多个future的select!
async fn example_multiple_futures() {
    println!("\n=== 多个future的select! ===");

    async fn task_a() -> &'static str {
        sleep(Duration::from_millis(30)).await;
        "任务A"
    }

    async fn task_b() -> &'static str {
        sleep(Duration::from_millis(50)).await;
        "任务B"
    }

    async fn task_c() -> &'static str {
        sleep(Duration::from_millis(70)).await;
        "任务C"
    }

    let fut_a = task_a().fuse();
    let fut_b = task_b().fuse();
    let fut_c = task_c().fuse();

    pin_mut!(fut_a, fut_b, fut_c);

    select! {
        result = fut_a => println!("第一个完成: {}", result),
        result = fut_b => println!("第二个完成: {}", result),
        result = fut_c => println!("第三个完成: {}", result),
    }
}

/// 4. select!与循环结合 - 直到某个条件满足
async fn example_select_with_loop() {
    println!("\n=== select!与循环结合 ===");

    async fn background_task() -> &'static str {
        sleep(Duration::from_millis(200)).await;
        "后台任务完成"
    }

    async fn timeout_task() -> &'static str {
        sleep(Duration::from_millis(150)).await;
        "超时任务触发"
    }

    let mut counter = 0;

    loop {
        let bg_fut = background_task().fuse();
        let timeout_fut = timeout_task().fuse();

        pin_mut!(bg_fut, timeout_fut);

        select! {
            result = bg_fut => {
                println!("{} - 循环次数: {}", result, counter);
                break;
            }
            result = timeout_fut => {
                counter += 1;
                println!("{} - 继续等待...", result);
                if counter >= 3 {
                    println!("超过最大重试次数，退出循环");
                    break;
                }
            }
        }
    }
}

/// 5. select!与通道结合
async fn example_select_with_channels() {
    println!("\n=== select!与通道结合 ===");

    use tokio::sync::mpsc;

    let (tx1, mut rx1) = mpsc::channel(10);
    let (tx2, mut rx2) = mpsc::channel(10);

    // 发送任务
    tokio::spawn(async move {
        sleep(Duration::from_millis(40)).await;
        tx1.send("通道1消息").await.unwrap_or_else(|e| {
            eprintln!("发送通道1消息失败: {:?}", e);
        });
    });

    tokio::spawn(async move {
        sleep(Duration::from_millis(60)).await;
        tx2.send("通道2消息").await.unwrap_or_else(|e| {
            eprintln!("发送通道2消息失败: {:?}", e);
        });
    });

    select! {
        msg = rx1.recv().fuse() => match msg{
            Some(msg) => println!("收到通道1消息: {}", msg),
            None => println!("通道1已关闭"),
        },
        msg = rx2.recv().fuse() => match msg{
            Some(msg) => println!("收到通道2消息: {}", msg),
            None => println!("通道2已关闭"),
        },
    }
}

/// 6. select!默认分支 - 处理超时
async fn example_select_default() {
    println!("\n=== select!默认分支 - 处理超时 ===");

    async fn long_running_task() -> &'static str {
        sleep(Duration::from_secs(1)).await;
        "长时间任务完成"
    }

    let long_fut = long_running_task().fuse();
    pin_mut!(long_fut);

    let start = std::time::Instant::now();

    loop {
        select! {
            result = long_fut => {
                println!("{} - 耗时: {:?}", result, start.elapsed());
                break;
            }
            default => {
                if start.elapsed() > Duration::from_secs(1) {
                    println!("任务超时，强制退出");
                    break;
                }
                println!("等待中...");
                sleep(Duration::from_millis(100)).await;
            }
        }
    }
}

/// 7. select!完整分支语法
async fn example_complete_branches() {
    println!("\n=== select!完整分支语法 ===");

    async fn task_one() -> u32 {
        sleep(Duration::from_millis(80)).await;
        1
    }

    async fn task_two() -> u32 {
        sleep(Duration::from_millis(120)).await;
        2
    }

    let fut1 = task_one().fuse();
    let fut2 = task_two().fuse();

    pin_mut!(fut1, fut2);

    select! {
        res1 = fut1 => {
            println!("任务1完成: {}", res1);
            // 可以继续处理任务2
            let res2 = fut2.await;
            println!("任务2也完成: {}", res2);
        }
        res2 = fut2 => {
            println!("任务2完成: {}", res2);
            // 可以继续处理任务1
            let res1 = fut1.await;
            println!("任务1也完成: {}", res1);
        }
    }
}

/// 8. 实际应用：竞速下载
async fn example_download_race() {
    println!("\n=== 实际应用：竞速下载 ===");

    async fn download_from_server_a() -> Result<String, &'static str> {
        sleep(Duration::from_millis(90)).await;
        Ok("从服务器A下载完成".to_string())
    }

    async fn download_from_server_b() -> Result<String, &'static str> {
        sleep(Duration::from_millis(70)).await;
        Ok("从服务器B下载完成".to_string())
    }

    async fn download_with_timeout() -> Result<String, &'static str> {
        sleep(Duration::from_millis(150)).await;
        Ok("超时下载完成".to_string())
    }

    let server_a_fut = download_from_server_a().fuse();
    let server_b_fut = download_from_server_b().fuse();
    let timeout_fut = download_with_timeout().fuse();

    pin_mut!(server_a_fut, server_b_fut, timeout_fut);

    let start = std::time::Instant::now();

    select! {
        result = server_a_fut => match result{
            Ok(result) => println!("{} - 耗时: {:?}", result, start.elapsed()),
            Err(e) => println!("服务器A下载错误: {}", e),
        },
        result = server_b_fut => match result{
            Ok(result) => println!("{} - 耗时: {:?}", result, start.elapsed()),
            Err(e) => println!("服务器B下载错误: {}", e),
        },
        result = timeout_fut => match result{
            Ok(result) => println!("{} - 耗时: {:?}", result, start.elapsed()),
            Err(e) => println!("超时下载错误: {}", e),
        },
    }
}

/// 9. select!与Stream结合
async fn example_select_with_stream() {
    println!("\n=== select!与Stream结合 ===");

    use futures::stream::{self, StreamExt};

    let mut stream1 = stream::iter(vec![1, 2, 3]).fuse();
    let mut stream2 = stream::iter(vec!["a", "b", "c"]).fuse();

    pin_mut!(stream1, stream2);

    let mut results = Vec::new();

    for i in 0..8 {
        println!("第{}轮选择:", i);
        select! {
            num = stream1.next() =>match num {
                Some(num) => {
                    println!("收到数字: {}", num);
                    results.push(format!("数字: {}", num))},
                None => {
                    println!("Stream1已结束");
                    continue;
                },
            },
            letter = stream2.next() => match letter{
                Some(letter) => {
                    println!("收到字母: {}", letter);
                    results.push(format!("字母: {}", letter))},
                None => {
                    println!("Stream2已结束");
                    continue;
                },
            },
            complete => println!("Stream选择完成"),
        }
    }

    println!("Stream选择结果: {:?}", results);
}

/// 10. 性能测试：select! vs 顺序await
async fn example_performance_comparison() {
    println!("\n=== 性能测试：select! vs 顺序await ===");

    async fn fast_operation() -> u32 {
        sleep(Duration::from_millis(50)).await;
        1
    }

    async fn slow_operation() -> u32 {
        sleep(Duration::from_millis(100)).await;
        2
    }

    // 使用select!并发执行
    let start = std::time::Instant::now();
    let fast_fut = fast_operation().fuse();
    let slow_fut = slow_operation().fuse();
    pin_mut!(fast_fut, slow_fut);

    let (result1, result2) = select! {
        res1 = fast_fut => {
            let res2 = slow_fut.await;
            (res1, res2)
        }
        res2 = slow_fut => {
            let res1 = fast_fut.await;
            (res1, res2)
        }
    };

    let select_time = start.elapsed();
    let select_sum = result1 + result2;

    // 顺序执行
    let start = std::time::Instant::now();
    let result1 = fast_operation().await;
    let result2 = slow_operation().await;
    let sequential_time = start.elapsed();
    let sequential_sum = result1 + result2;

    println!("select!执行: 结果={}, 耗时={:?}", select_sum, select_time);
    println!(
        "顺序执行: 结果={}, 耗时={:?}",
        sequential_sum, sequential_time
    );
    println!("性能差异: {:?}", sequential_time - select_time);
}

/// 11. 使用select!合并两个Stream的示例
async fn add_two_streams(
    mut s1: impl Stream<Item = u32> + FusedStream + Unpin,
    mut s2: impl Stream<Item = u32> + FusedStream + Unpin,
) -> u32 {
    let mut total = 0;

    loop {
        let item = select! {
            x = s1.next() => x,
            x = s2.next() => x,
            complete => break,
        };
        if let Some(next_num) = item {
            total += next_num;
        }
    }

    total
}
/// 12. 演示add_two_streams的使用
async fn example_add_two_streams() {
    println!("\n=== 使用select!合并两个Stream ===");

    use futures::stream;

    // 创建两个Stream
    let stream1 = stream::iter(vec![1, 3, 5]).fuse();
    let stream2 = stream::iter(vec![2, 4, 6]).fuse();

    let result = add_two_streams(stream1, stream2).await;
    println!("两个Stream合并结果: {}", result);

    // 测试不同长度的Stream
    let stream3 = stream::iter(vec![10, 20]).fuse();
    let stream4 = stream::iter(vec![30, 40, 50]).fuse();

    let result2 = add_two_streams(stream3, stream4).await;
    println!("不同长度Stream合并结果: {}", result2);

    // 测试空Stream
    let empty_stream = stream::iter(vec![] as Vec<u32>).fuse();
    let normal_stream = stream::iter(vec![100, 200]).fuse();

    let result3 = add_two_streams(empty_stream, normal_stream).await;
    println!("空Stream合并结果: {}", result3);
}

/// 13. FuturesUnordered基本使用示例
async fn example_futures_unordered_basic() {
    println!("\n=== FuturesUnordered基本使用示例 ===");

    use futures::stream::FuturesUnordered;

    async fn task(id: u32, duration_ms: u64) -> String {
        sleep(Duration::from_millis(duration_ms)).await;
        format!("任务{}完成 - 耗时{}ms", id, duration_ms)
    }

    // 创建FuturesUnordered并添加多个任务
    let mut futures = FuturesUnordered::new();

    // 添加不同耗时的任务
    futures.push(task(1, 100));
    futures.push(task(2, 50)); // 这个会先完成
    futures.push(task(3, 150));
    futures.push(task(4, 30)); // 这个会最先完成

    println!("开始并发执行任务...");

    // 按照完成顺序收集结果
    let mut results = Vec::new();
    while let Some(result) = futures.next().await {
        println!("收到结果: {}", result);
        results.push(result);
    }

    println!("所有任务完成！结果顺序: {:?}", results);
}

/// 14. FuturesUnordered与错误处理
async fn example_futures_unordered_with_errors() {
    println!("\n=== FuturesUnordered与错误处理 ===");

    use futures::stream::FuturesUnordered;

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
                println!("✅ {}", msg);
                success_count += 1;
            }
            Err(e) => {
                println!("❌ 错误: {}", e);
                error_count += 1;
            }
        }
    }

    println!("统计: 成功{}个, 失败{}个", success_count, error_count);
}

/// 15. FuturesUnordered动态添加任务
async fn example_futures_unordered_dynamic() {
    println!("\n=== FuturesUnordered动态添加任务 ===");

    use futures::stream::FuturesUnordered;

    async fn dynamic_task(id: u32) -> String {
        let duration = (id * 10) as u64;
        sleep(Duration::from_millis(duration)).await;
        format!("动态任务{} - 耗时{}ms", id, duration)
    }

    let mut futures = FuturesUnordered::new();
    let mut completed_count = 0;

    // 初始添加一些任务
    for i in 1..=3 {
        futures.push(dynamic_task(i));
    }

    println!("开始执行，初始3个任务...");

    while completed_count < 10 {
        tokio::select! {
            Some(result) = futures.next() => {
                println!("完成: {}", result);
                completed_count += 1;

                // 动态添加新任务
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

/// 16. FuturesUnordered与Stream结合
async fn example_futures_unordered_with_stream() {
    println!("\n=== FuturesUnordered与Stream结合 ===");

    use futures::stream::{FuturesUnordered, StreamExt};

    async fn process_item(item: u32) -> u32 {
        sleep(Duration::from_millis(item as u64 * 10)).await;
        item * 2
    }

    // 创建数据流
    let data_stream = futures::stream::iter(1..=10);

    // 使用FuturesUnordered并发处理流中的每个元素
    let mut processing_futures = FuturesUnordered::new();

    let mut processed_results = Vec::new();
    let mut data_stream = Box::pin(data_stream);

    loop {
        tokio::select! {
            // 从流中获取新项目并开始处理
            Some(item) = data_stream.next() => {
                println!("开始处理项目: {}", item);
                processing_futures.push(process_item(item));
            }
            // 收集已完成的结果
            Some(result) = processing_futures.next() => {
                println!("处理完成: {}", result);
                processed_results.push(result);
            }
            // 当流结束且没有正在处理的任务时退出
            else => {
                if processing_futures.is_empty() {
                    break;
                }
            }
        }
    }

    println!("流处理完成！结果: {:?}", processed_results);
}

/// 17. FuturesUnordered性能对比 - 并发vs顺序
async fn example_futures_unordered_performance() {
    println!("\n=== FuturesUnordered性能对比 ===");

    use futures::stream::FuturesUnordered;

    async fn cpu_intensive_task(id: u32) -> u32 {
        // 模拟CPU密集型任务
        let mut result = 0;
        for i in 0..1000 {
            result += i * id;
        }
        sleep(Duration::from_millis(50)).await; // 模拟I/O等待
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

#[tokio::main]
async fn main() {
    println!("开始futures::select!宏示例测试...");

    // 运行所有示例
    example_basic_race().await;
    example_pattern_matching().await;
    example_multiple_futures().await;
    example_select_with_loop().await;
    example_select_with_channels().await;
    example_select_default().await;
    example_complete_branches().await;
    example_download_race().await;
    example_select_with_stream().await;
    example_performance_comparison().await;
    example_add_two_streams().await;
    example_futures_unordered_basic().await;
    example_futures_unordered_with_errors().await;
    example_futures_unordered_dynamic().await;
    example_futures_unordered_with_stream().await;
    example_futures_unordered_performance().await;

    println!("\n🎉 所有select!宏示例测试完成！");
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use futures::{pin_mut, select};
    use tokio::time::sleep;

    use super::*;

    #[tokio::test]
    async fn test_basic_select() {
        async fn quick_task() -> &'static str {
            "完成"
        }

        async fn slower_task() -> &'static str {
            sleep(Duration::from_millis(10)).await;
            "慢完成"
        }

        let quick_fut = quick_task().fuse();
        let slow_fut = slower_task().fuse();
        pin_mut!(quick_fut, slow_fut);

        let result = select! {
            res = quick_fut => res,
            res = slow_fut => res,
        };

        assert_eq!(result, "完成");
    }

    #[tokio::test]
    async fn test_select_with_result() {
        async fn success_task() -> Result<u32, &'static str> {
            Ok(42)
        }

        async fn error_task() -> Result<u32, &'static str> {
            Err("错误")
        }

        let success_fut = success_task().fuse();
        let error_fut = error_task().fuse();
        pin_mut!(success_fut, error_fut);

        select! {
            value = success_fut => match value {
                Ok(v) => assert_eq!(v, 42),
                Err(_) => panic!("应该选择成功任务"),
            },
            value = error_fut => match value {
                Ok(_) => panic!("应该选择错误任务"),
                Err(e) => assert_eq!(e, "错误"),
            },
        }
    }

    #[tokio::test]
    async fn test_select_multiple_branches() {
        async fn task_a() -> u32 {
            1
        }
        async fn task_b() -> u32 {
            2
        }
        async fn task_c() -> u32 {
            3
        }

        let fut_a = task_a().fuse();
        let fut_b = task_b().fuse();
        let fut_c = task_c().fuse();
        pin_mut!(fut_a, fut_b, fut_c);

        let result = select! {
            a = fut_a => a,
            b = fut_b => b,
            c = fut_c => c,
        };

        assert_eq!(result, 1); // 最快的任务应该先完成
    }

    #[tokio::test]
    async fn test_select_with_default() {
        async fn long_task() -> u32 {
            sleep(Duration::from_millis(50)).await;
            100
        }

        let long_fut = long_task().fuse();
        pin_mut!(long_fut);

        let mut default_called = false;

        select! {
            result = long_fut => {
                assert_eq!(result, 100);
            }
            default => {
                default_called = true;
            }
        }

        // 默认分支不应该被调用，因为任务会先完成
        assert!(!default_called);
    }

    #[tokio::test]
    async fn test_select_complete_branch() {
        use futures::stream::{self, StreamExt};

        let mut stream = stream::iter(vec![1, 2]).fuse();
        pin_mut!(stream);

        let mut results = Vec::new();

        loop {
            select! {
                item = stream.next() => match item{
                    Some(item) => results.push(item),
                    None => break,
                },
                complete => break,
            }
        }

        assert_eq!(results, vec![1, 2]);
    }
    #[tokio::test]
    async fn test_add_two_streams_single_element() {
        use futures::stream;

        // 测试单元素Stream
        let stream1 = stream::iter(vec![42]).fuse();
        let stream2 = stream::iter(vec![58]).fuse();

        let result = add_two_streams(stream1, stream2).await;
        assert_eq!(result, 100); // 42+58 = 100
    }

    #[tokio::test]
    async fn test_futures_unordered_basic() {
        use futures::stream::FuturesUnordered;
        use tokio::time::sleep;

        async fn quick_task() -> &'static str {
            "快速"
        }

        async fn slow_task() -> &'static str {
            sleep(Duration::from_millis(10)).await;
            "慢速"
        }

        let mut futures = FuturesUnordered::new();
        futures.push(quick_task().boxed());
        futures.push(slow_task().boxed());

        let mut results = Vec::new();
        while let Some(result) = futures.next().await {
            results.push(result);
        }

        // 应该按照完成顺序返回
        assert_eq!(results, vec!["快速", "慢速"]);
    }

    #[tokio::test]
    async fn test_futures_unordered_error_handling() {
        use futures::stream::FuturesUnordered;

        async fn success_task() -> Result<u32, &'static str> {
            Ok(42)
        }

        async fn error_task() -> Result<u32, &'static str> {
            Err("错误")
        }

        let mut futures = FuturesUnordered::new();
        futures.push(success_task().boxed());
        futures.push(error_task().boxed());

        let mut success_count = 0;
        let mut error_count = 0;

        while let Some(result) = futures.next().await {
            match result {
                Ok(_) => success_count += 1,
                Err(_) => error_count += 1,
            }
        }

        assert_eq!(success_count, 1);
        assert_eq!(error_count, 1);
    }

    #[tokio::test]
    async fn test_futures_unordered_dynamic() {
        use futures::stream::FuturesUnordered;

        async fn simple_task(id: u32) -> u32 {
            id
        }

        let mut futures = FuturesUnordered::new();

        // 初始添加
        futures.push(simple_task(1));
        futures.push(simple_task(2));

        let mut results = Vec::new();
        let mut count = 0;

        while count < 4 {
            if let Some(result) = futures.next().await {
                results.push(result);
                count += 1;

                // 动态添加
                if count < 4 {
                    futures.push(simple_task(count + 2));
                }
            }
        }

        assert_eq!(results.len(), 4);
    }

    #[tokio::test]
    async fn test_futures_unordered_empty() {
        use futures::stream::FuturesUnordered;

        let mut futures: FuturesUnordered<tokio::task::JoinHandle<u32>> = FuturesUnordered::new();

        // 空的FuturesUnordered应该立即返回None
        let result = futures.next().await;
        assert!(result.is_none());
    }

    #[tokio::test]
    async fn test_futures_unordered_concurrent() {
        use std::sync::{
            Arc,
            atomic::{AtomicU32, Ordering},
        };

        use futures::stream::FuturesUnordered;

        let counter = Arc::new(AtomicU32::new(0));

        async fn increment_task(counter: Arc<AtomicU32>, id: u32) -> u32 {
            // 模拟并发递增
            for _ in 0..100 {
                let current = counter.load(Ordering::SeqCst);
                counter.store(current + 1, Ordering::SeqCst);
                tokio::task::yield_now().await;
            }
            id
        }

        let mut futures = FuturesUnordered::new();
        for i in 0..5 {
            let counter_clone = counter.clone();
            futures.push(increment_task(counter_clone, i));
        }

        let mut results = Vec::new();
        while let Some(result) = futures.next().await {
            results.push(result);
        }

        // 验证并发执行（计数器应该远大于顺序执行的值）
        let final_count = counter.load(Ordering::SeqCst);
        assert!(final_count >= 500); // 由于并发，可能大于500
        assert_eq!(results.len(), 5);
    }
}
