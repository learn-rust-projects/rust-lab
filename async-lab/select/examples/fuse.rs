use std::time::Duration;

use futures::{
    future::{Fuse, FusedFuture, FutureExt},
    pin_mut, select,
    stream::{FusedStream, Stream, StreamExt},
};
use tokio::time::{interval, sleep};
use tokio_stream::wrappers::IntervalStream;

/// 使用Fuse和select!宏的复杂异步循环示例
/// 这个示例展示了如何管理多个future的状态和生命周期
async fn get_new_num() -> u8 {
    // 模拟异步获取新数字的操作
    sleep(Duration::from_millis(100)).await;
    5
}

async fn run_on_new_num(num: u8) {
    // 模拟基于新数字执行的操作
    println!("执行操作，使用数字: {}", num);
    sleep(Duration::from_millis(50)).await;
}

/// 主要的异步循环函数
/// 使用Fuse来管理future的生命周期，避免重复执行已完成的future
async fn run_loop(
    mut interval_timer: impl Stream<Item = ()> + FusedStream + Unpin,
    starting_num: u8,
) {
    // 初始化future，使用Fuse包装
    let run_on_new_num_fut = run_on_new_num(starting_num).fuse();
    let get_new_num_fut = Fuse::terminated(); // 初始化为已终止状态

    pin_mut!(run_on_new_num_fut, get_new_num_fut);

    println!("开始异步循环，初始数字: {}", starting_num);
    let mut iteration = 0;

    loop {
        iteration += 1;
        println!("=== 循环迭代 {} ===", iteration);

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

            // 获取新数字完成后，启动新的run_on_new_num_fut
            new_num = get_new_num_fut => {
                println!("step2:获取到新数字: {}", new_num);
                println!("启动新的run_on_new_num任务");
                run_on_new_num_fut.set(run_on_new_num(new_num).fuse());
            },

            // 执行run_on_new_num_fut
            () = run_on_new_num_fut => {
                println!("step3:run_on_new_num任务完成");
            },

            // 如果所有分支都完成（不应该发生，因为定时器应该持续产生值）
            complete => panic!("`interval_timer`意外完成"),
        }

        // 限制循环次数，避免无限运行
        if iteration >= 10 {
            println!("达到最大迭代次数，退出循环");
            break;
        }
    }
}

/// 演示run_loop函数的使用
pub async fn example_fuse_loop() {
    println!("\n=== Fuse和select!循环示例 ===");
    use futures::StreamExt;
    // 创建定时器流，每秒触发一次
    let interval_stream = IntervalStream::new(interval(Duration::from_secs(1)))
        .map(|_| ())
        .take(5) // 限制为5次触发，避免无限运行
        .fuse();

    // 运行循环，初始数字为0
    run_loop(interval_stream, 0).await;

    println!("Fuse循环示例完成");
}
#[tokio::main]
async fn main() {
    example_fuse_loop().await;
}
/// 测试函数：验证Fuse循环的基本功能
#[cfg(test)]
mod tests {
    use futures::stream;
    use tokio::time::Duration;

    use super::*;

    #[tokio::test]
    async fn test_fuse_loop_basic() {
        // 创建快速的定时器流用于测试
        let fast_interval = stream::iter(vec![(), (), ()]).fuse();

        // 运行循环，应该正常完成而不panic
        run_loop(fast_interval, 10).await;
    }

    #[tokio::test]
    async fn test_fuse_loop_with_get_new_num() {
        // 测试get_new_num函数
        let result = get_new_num().await;
        assert_eq!(result, 5);
    }

    #[tokio::test]
    async fn test_fuse_loop_run_on_new_num() {
        // 测试run_on_new_num函数
        // 这个函数主要是副作用，我们验证它不会panic
        run_on_new_num(42).await;
    }

    #[tokio::test]
    async fn test_fuse_set_method() {
        // 测试Fuse的set方法
        let mut fuse_fut = Fuse::terminated();
        pin_mut!(fuse_fut);
        // 设置新的future
        fuse_fut.set(async { 42 }.fuse());
        assert!(!fuse_fut.is_terminated());
    }

    #[tokio::test]
    async fn test_fuse_loop_complete_branch() {
        // 测试complete分支（应该panic）
        use futures::stream;

        let empty_stream = stream::empty::<()>().fuse();

        // 这个测试应该panic，因为流是空的，会触发complete分支
        let result = std::panic::catch_unwind(|| {
            tokio::runtime::Runtime::new().unwrap().block_on(async {
                run_loop(empty_stream, 0).await;
            })
        });

        assert!(result.is_err());
    }
}
