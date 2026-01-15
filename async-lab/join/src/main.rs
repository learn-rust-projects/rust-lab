use std::time::Duration;

use futures::{join, try_join};
use tokio::time::sleep;

/// 模拟获取书籍的异步操作
async fn get_book() -> String {
    sleep(Duration::from_millis(100)).await;
    "《Rust编程之道》".to_string()
}

/// 模拟获取音乐的异步操作
async fn get_music() -> String {
    sleep(Duration::from_millis(150)).await;
    "《Rust异步编程》专辑".to_string()
}

/// 模拟获取书籍（返回Result）
async fn get_book_result() -> Result<String, String> {
    sleep(Duration::from_millis(80)).await;
    Ok("《Rust权威指南》".to_string())
}

/// 模拟获取音乐（返回Result，不同错误类型）
async fn get_music_result() -> Result<String, &'static str> {
    println!("尝试获取音乐...");
    sleep(Duration::from_millis(120)).await;
    println!("成功获取音乐");
    Ok("《Tokio实战》专辑".to_string())
}

/// 模拟可能失败的获取操作
async fn get_book_with_error() -> Result<String, String> {
    println!("尝试获取书籍...");
    sleep(Duration::from_millis(50)).await;
    println!("获取书籍失败");
    Err("网络错误：无法获取书籍".to_string())
}

/// 模拟可能失败的音乐获取
async fn get_music_with_error() -> Result<String, String> {
    sleep(Duration::from_millis(70)).await;
    Err("服务器错误：无法获取音乐".to_string())
}

/// 1. 基本join!使用示例
async fn example_basic_join() {
    println!("\n=== 基本join!使用示例 ===");

    // 错误的方式：顺序执行
    async fn get_book_and_music_sequential() -> (String, String) {
        let book = get_book().await;
        let music = get_music().await;
        (book, music)
    }

    // 正确的方式：使用join!并发执行
    async fn get_book_and_music_concurrent() -> (String, String) {
        let book_fut = get_book();
        let music_fut = get_music();
        join!(book_fut, music_fut)
    }

    // 测试两种方式的性能差异
    let start = std::time::Instant::now();
    let (book_seq, music_seq) = get_book_and_music_sequential().await;
    let sequential_time = start.elapsed();

    let start = std::time::Instant::now();
    let (book_conc, music_conc) = get_book_and_music_concurrent().await;
    let concurrent_time = start.elapsed();

    println!(
        "顺序执行结果: ({}, {}), 耗时: {:?}",
        book_seq, music_seq, sequential_time
    );
    println!(
        "并发执行结果: ({}, {}), 耗时: {:?}",
        book_conc, music_conc, concurrent_time
    );
    println!(
        "并发执行比顺序执行快: {:?}",
        sequential_time - concurrent_time
    );
}

/// 2. try_join!使用示例
async fn example_try_join() {
    println!("\n=== try_join!使用示例 ===");
    use futures::future::TryFutureExt;
    // 使用try_join!处理Result类型的future
    match try_join!(
        get_book_result(),
        get_music_result().map_err(|e| e.to_string())
    ) {
        Ok((book, music)) => {
            println!("成功获取: {}, {}", book, music);
        }
        Err(e) => {
            println!("获取失败: {}", e);
        }
    }
}

/// 3. try_join!错误处理示例
async fn example_try_join_error() {
    println!("\n=== try_join!错误处理示例 ===");
    // 使用futures::future::TryFutureExt整合所有错误类型
    use futures::future::TryFutureExt;
    // 当其中一个future失败时，try_join!会立即返回错误
    let get_music_result = get_music_result().map_err(|e| e.to_string());
    match try_join!(get_book_with_error(), get_music_result) {
        Ok((book, music)) => {
            println!("成功获取: {}, {}", book, music);
        }
        Err(e) => {
            println!("其中一个操作失败: {}", e);
        }
    }

    // 两个都失败的情况
    match try_join!(get_book_with_error(), get_music_with_error()) {
        Ok((book, music)) => {
            println!("成功获取: {}, {}", book, music);
        }
        Err(e) => {
            println!("两个操作都失败: {}", e);
        }
    }
}

/// 4. 多个future的join!
async fn example_multiple_join() {
    println!("\n=== 多个future的join!示例 ===");

    async fn get_user_data() -> String {
        sleep(Duration::from_millis(60)).await;
        "用户信息".to_string()
    }

    async fn get_product_data() -> String {
        sleep(Duration::from_millis(90)).await;
        "产品信息".to_string()
    }

    async fn get_order_data() -> String {
        sleep(Duration::from_millis(40)).await;
        "订单信息".to_string()
    }

    let (user, product, order) = join!(get_user_data(), get_product_data(), get_order_data());

    println!("并发获取结果: {}, {}, {}", user, product, order);
}

/// 5. 嵌套join!使用
async fn example_nested_join() {
    println!("\n=== 嵌套join!使用示例 ===");

    async fn get_user_profile() -> String {
        sleep(Duration::from_millis(50)).await;
        "用户档案".to_string()
    }

    async fn get_user_settings() -> String {
        sleep(Duration::from_millis(30)).await;
        "用户设置".to_string()
    }

    async fn get_complete_user_data() -> (String, String) {
        let (profile, settings) = join!(get_user_profile(), get_user_settings());
        (profile, settings)
    }

    async fn get_system_info() -> String {
        sleep(Duration::from_millis(70)).await;
        "系统信息".to_string()
    }

    let ((profile, settings), system_info) = join!(get_complete_user_data(), get_system_info());
    println!(
        "嵌套join结果: 档案={}, 设置={}, 系统={}",
        profile, settings, system_info
    );
}

/// 6. 带超时控制的join!
async fn example_join_with_timeout() {
    println!("\n=== 带超时控制的join!示例 ===");

    use tokio::time::timeout;

    async fn slow_operation() -> String {
        sleep(Duration::from_secs(2)).await;
        "慢操作完成".to_string()
    }

    async fn fast_operation() -> String {
        sleep(Duration::from_millis(100)).await;
        "快操作完成".to_string()
    }

    // 使用timeout包装join!
    match timeout(Duration::from_secs(1), async {
        join!(slow_operation(), fast_operation())
    })
    .await
    {
        Ok((slow_result, fast_result)) => {
            println!("操作完成: {}, {}", slow_result, fast_result);
        }
        Err(_) => {
            println!("操作超时");
        }
    }
}

/// 7. 实际应用场景：并发API调用
async fn example_concurrent_api_calls() {
    println!("\n=== 并发API调用示例 ===");

    async fn call_user_api() -> Result<String, String> {
        sleep(Duration::from_millis(80)).await;
        Ok("用户API响应".to_string())
    }

    async fn call_product_api() -> Result<String, String> {
        sleep(Duration::from_millis(120)).await;
        Ok("产品API响应".to_string())
    }

    async fn call_order_api() -> Result<String, String> {
        sleep(Duration::from_millis(60)).await;
        Ok("订单API响应".to_string())
    }

    match try_join!(call_user_api(), call_product_api(), call_order_api()) {
        Ok((user_resp, product_resp, order_resp)) => {
            println!("并发API调用成功:");
            println!("  用户API: {}", user_resp);
            println!("  产品API: {}", product_resp);
            println!("  订单API: {}", order_resp);
        }
        Err(e) => {
            println!("API调用失败: {}", e);
        }
    }
}

/// 8. 性能对比测试
async fn example_performance_comparison() {
    println!("\n=== 性能对比测试 ===");

    async fn task_1() -> u32 {
        sleep(Duration::from_millis(100)).await;
        1
    }

    async fn task_2() -> u32 {
        sleep(Duration::from_millis(100)).await;
        2
    }

    async fn task_3() -> u32 {
        sleep(Duration::from_millis(100)).await;
        3
    }

    // 顺序执行
    let start = std::time::Instant::now();
    let result1 = task_1().await;
    let result2 = task_2().await;
    let result3 = task_3().await;
    let sequential_time = start.elapsed();
    let sequential_sum = result1 + result2 + result3;

    // 并发执行
    let start = std::time::Instant::now();
    let (result1, result2, result3) = join!(task_1(), task_2(), task_3());
    let concurrent_time = start.elapsed();
    let concurrent_sum = result1 + result2 + result3;

    println!(
        "顺序执行: 结果={}, 耗时={:?}",
        sequential_sum, sequential_time
    );
    println!(
        "并发执行: 结果={}, 耗时={:?}",
        concurrent_sum, concurrent_time
    );
    println!(
        "性能提升: {:.1}%",
        (sequential_time.as_millis() as f64 - concurrent_time.as_millis() as f64)
            / sequential_time.as_millis() as f64
            * 100.0
    );
}

#[tokio::main]
async fn main() {
    println!("开始futures::join!和futures::try_join!宏示例测试...");

    // 运行所有示例
    example_basic_join().await;
    example_try_join().await;
    example_try_join_error().await;
    example_multiple_join().await;
    example_nested_join().await;
    example_join_with_timeout().await;
    example_concurrent_api_calls().await;
    example_performance_comparison().await;

    println!("\n🎉 所有join!宏示例测试完成！");
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use tokio::time::sleep;

    use super::*;

    #[tokio::test]
    async fn test_basic_join() {
        async fn quick_task() -> &'static str {
            "完成"
        }

        let (result1, result2) = join!(quick_task(), quick_task());
        assert_eq!(result1, "完成");
        assert_eq!(result2, "完成");
    }

    #[tokio::test]
    async fn test_try_join_success() {
        async fn successful_task() -> Result<u32, &'static str> {
            Ok(42)
        }

        let result = try_join!(successful_task(), successful_task());
        assert_eq!(result, Ok((42, 42)));
    }

    #[tokio::test]
    async fn test_try_join_error() {
        async fn failing_task() -> Result<u32, &'static str> {
            Err("测试错误")
        }

        async fn successful_task() -> Result<u32, &'static str> {
            Ok(42)
        }

        let result = try_join!(failing_task(), successful_task());
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_join_performance() {
        async fn delayed_task() -> u32 {
            sleep(Duration::from_millis(50)).await;
            1
        }

        let start = std::time::Instant::now();
        let (a, b) = join!(delayed_task(), delayed_task());
        let duration = start.elapsed();

        assert_eq!(a + b, 2);
        // 并发执行应该远小于100ms（两个50ms任务的顺序执行时间）
        assert!(duration.as_millis() < 70);
    }

    #[tokio::test]
    async fn test_multiple_join() {
        async fn task_1() -> u32 {
            1
        }
        async fn task_2() -> u32 {
            2
        }
        async fn task_3() -> u32 {
            3
        }

        let (a, b, c) = join!(task_1(), task_2(), task_3());
        assert_eq!((a, b, c), (1, 2, 3));
    }
}
