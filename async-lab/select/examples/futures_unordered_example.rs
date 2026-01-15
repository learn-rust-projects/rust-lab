//! FuturesUnordered与Fuse和select!宏结合使用的复杂示例
//!
//! 这个示例展示了如何使用FuturesUnordered来管理多个并发任务，
//! 同时使用Fuse来管理单个future的生命周期，并通过select!宏
//! 实现复杂的异步控制流。

use std::time::Duration;

use futures::{
    future::{Fuse, FusedFuture, FutureExt},
    pin_mut, select,
    stream::{FusedStream, FuturesUnordered, Stream, StreamExt},
};
use tokio::time::{interval, sleep};

/// 模拟异步获取新数字的操作
async fn get_new_num() -> u8 {
    // 模拟异步操作，比如从网络或数据库获取数据
    sleep(Duration::from_millis(100)).await;
    println!("获取到新数字: 5");
    5
}

/// 模拟基于新数字执行的操作
async fn run_on_new_num(num: u8) -> u8 {
    // 模拟基于数字执行的操作，比如计算或处理
    println!("开始处理数字: {}", num);
    sleep(Duration::from_millis(50)).await;
    let result = num * 2;
    println!("处理完成，结果: {}", result);
    result
}

/// 主要的异步循环函数
///
/// 这个函数展示了如何结合使用：
/// - FuturesUnordered: 管理多个run_on_new_num任务的并发执行
/// - Fuse: 管理单个get_new_num_fut的生命周期
/// - select!宏: 实现复杂的异步控制流
///
/// # 参数
/// - `interval_timer`: 定时器流，用于定期触发获取新数字的操作
/// - `starting_num`: 初始数字，用于启动第一个run_on_new_num任务
async fn run_loop(
    mut interval_timer: impl Stream<Item = ()> + FusedStream + Unpin,
    starting_num: u8,
) {
    println!("开始异步循环，初始数字: {}", starting_num);

    // 使用FuturesUnordered来管理多个run_on_new_num任务的并发执行
    let mut run_on_new_num_futs = FuturesUnordered::new();

    // 启动第一个任务
    run_on_new_num_futs.push(run_on_new_num(starting_num));
    println!("启动初始任务，数字: {}", starting_num);

    // 使用Fuse来管理get_new_num_fut的生命周期
    let get_new_num_fut = Fuse::terminated();
    pin_mut!(get_new_num_fut);

    let mut iteration = 0;

    loop {
        iteration += 1;
        println!("\n=== 循环迭代 {} ===", iteration);
        println!("当前活跃任务数: {}", run_on_new_num_futs.len());

        select! {
            // 定时器触发时，启动新的get_new_num_fut（如果当前没有正在运行的）
            () = interval_timer.select_next_some() => {
                println!("step1:定时器触发");

                if get_new_num_fut.is_terminated() {
                    println!("启动新的get_new_num任务");
                    get_new_num_fut.set(get_new_num().fuse());
                } else {
                    println!("get_new_num任务仍在运行，跳过");
                }
            },

            // 获取新数字完成后，启动新的run_on_new_num任务
            new_num = get_new_num_fut => {
                println!("step2:获取到新数字: {}", new_num);
                println!("启动新的run_on_new_num任务");
                run_on_new_num_futs.push(run_on_new_num(new_num));
            },

            // 处理run_on_new_num_futs中完成的任务
            res = run_on_new_num_futs.select_next_some() => {
                println!("step3:run_on_new_num任务完成，返回结果: {}", res);
            },

            // 如果所有分支都完成（不应该发生，因为定时器应该持续产生值）
            complete => {
                println!("所有任务意外完成，退出循环");
                break;
            },
        }

        // 限制循环次数，避免无限运行（在实际应用中可能不需要）
        if iteration >= 10 {
            println!("达到最大迭代次数，退出循环");
            break;
        }
    }

    println!("异步循环结束");
}

/// 演示函数：展示run_loop的使用
#[tokio::main]
async fn main() {
    println!("=== FuturesUnordered与Fuse结合示例 ===");
    use futures::StreamExt;
    use tokio_stream::wrappers::IntervalStream;

    // 创建定时器流，每500毫秒触发一次
    let interval_stream = IntervalStream::new(interval(Duration::from_secs(1)))
        .map(|_| {
            println!("--- 定时器触发 ---");
            
        })
        .take(5) // 限制触发次数
        .fuse();

    // 运行异步循环，初始数字为1
    run_loop(interval_stream, 1).await;

    println!("\n🎉 示例执行完成！");
}

#[cfg(test)]
mod tests {
    use futures::stream;

    use super::*;

    #[tokio::test]
    async fn test_get_new_num() {
        let result = get_new_num().await;
        assert_eq!(result, 5);
    }

    #[tokio::test]
    async fn test_run_on_new_num() {
        let result = run_on_new_num(10).await;
        assert_eq!(result, 20); // 10 * 2 = 20
    }

    #[tokio::test]
    async fn test_run_loop_basic() {
        // 使用快速的定时器流进行测试
        let fast_interval = stream::iter(vec![(), (), ()]).fuse();

        // 运行循环，应该正常完成
        run_loop(fast_interval, 1).await;
    }

    #[tokio::test]
    async fn test_futures_unordered_behavior() {
        // 测试FuturesUnordered的基本行为
        let mut futs = FuturesUnordered::new();

        futs.push(run_on_new_num(1));
        futs.push(run_on_new_num(2));
        futs.push(run_on_new_num(3));

        let mut results = Vec::new();
        while let Some(result) = futs.next().await {
            results.push(result);
        }

        // 应该得到所有结果（顺序可能不同）
        assert_eq!(results.len(), 3);
        assert!(results.contains(&2)); // 1*2=2
        assert!(results.contains(&4)); // 2*2=4
        assert!(results.contains(&6)); // 3*2=6
    }

    #[tokio::test]
    async fn test_fuse_management() {
        // 测试Fuse的生命周期管理
        let mut fuse_fut = Fuse::terminated();
        assert!(fuse_fut.is_terminated());

        // 设置新的future
        fuse_fut.set(get_new_num().fuse());
        assert!(!fuse_fut.is_terminated());

        // 等待完成
        let result = fuse_fut.await;
        assert_eq!(result, 5);
        assert!(fuse_fut.is_terminated());
    }
}
