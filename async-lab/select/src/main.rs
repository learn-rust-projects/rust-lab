use futures::{StreamExt, select, stream::FusedStream};

// 辅助函数：合并两个Stream
#[allow(dead_code)]
async fn add_two_streams(
    mut s1: impl FusedStream<Item = u32> + Unpin,
    mut s2: impl FusedStream<Item = u32> + Unpin,
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

#[tokio::main]
async fn main() {
    println!("运行测试: cargo test");
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use futures::{StreamExt, future::FutureExt, pin_mut, select};
    use tokio::time::sleep;

    use super::*;

    // 测试1: 竞速下载
    async fn run_basic_race() {
        async fn download_from_server_a() -> Result<String, &'static str> {
            sleep(Duration::from_millis(90)).await;
            Ok("从服务器A下载完成".to_string())
        }

        async fn download_from_server_b() -> Result<String, &'static str> {
            sleep(Duration::from_millis(70)).await;
            Ok("从服务器B下载完成".to_string())
        }

        use tokio::sync::mpsc;
        let (tx1, rx1) = mpsc::channel(10);

        tokio::spawn(async move {
            sleep(Duration::from_millis(150)).await;
            tx1.send("下载超时").await.unwrap_or_else(|e| {
                eprintln!("发送超时信号失败: {:?}", e);
            });
        });

        let server_a_fut = download_from_server_a().fuse();
        let server_b_fut = download_from_server_b().fuse();

        pin_mut!(server_a_fut, server_b_fut, rx1);

        let start = std::time::Instant::now();

        select! {
            result = server_a_fut => match result {
                Ok(msg) => println!("{} - 耗时: {:?}", msg, start.elapsed()),
                Err(e) => println!("服务器A下载错误: {}", e),
            },
            result = server_b_fut => match result {
                Ok(msg) => println!("{} - 耗时: {:?}", msg, start.elapsed()),
                Err(e) => println!("服务器B下载错误: {}", e),
            },
            msg = rx1.recv().fuse() => match msg {
                Some(msg) => println!("下载超时: {}", msg),
                None => println!("超时通道已关闭"),
            },
        }
    }

    // 测试2: select!与Stream结合
    async fn run_select_with_stream() {
        use futures::stream::{self, StreamExt};

        let stream1 = stream::iter(vec![1, 2, 3]).fuse();
        let stream2 = stream::iter(vec!["a", "b", "c"]).fuse();

        pin_mut!(stream1, stream2);

        let mut results = Vec::new();

        for i in 0..8 {
            println!("第{}轮选择:", i);
            select! {
                num = stream1.next() => match num {
                    Some(num) => {
                        println!("收到数字: {}", num);
                        results.push(format!("数字: {}", num));
                    }
                    None => {
                        println!("Stream1已结束");
                        continue;
                    },
                },
                letter = stream2.next() => match letter {
                    Some(letter) => {
                        println!("收到字母: {}", letter);
                        results.push(format!("字母: {}", letter));
                    }
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

    // 测试3: select!默认分支
    async fn run_select_default() {
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

    // 测试4: select!与循环结合
    async fn run_select_with_loop() {
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

    // 测试5: 使用select!合并两个Stream
    async fn run_add_two_streams() {
        use futures::stream;

        let stream1 = stream::iter(vec![1, 3, 5]).fuse();
        let stream2 = stream::iter(vec![2, 4, 6]).fuse();

        let result = add_two_streams(stream1, stream2).await;
        println!("两个Stream合并结果: {}", result);

        let stream3 = stream::iter(vec![10, 20]).fuse();
        let stream4 = stream::iter(vec![30, 40, 50]).fuse();

        let result2 = add_two_streams(stream3, stream4).await;
        println!("不同长度Stream合并结果: {}", result2);

        let empty_stream = stream::iter(vec![] as Vec<u32>).fuse();
        let normal_stream = stream::iter(vec![100, 200]).fuse();

        let result3 = add_two_streams(empty_stream, normal_stream).await;
        println!("空Stream合并结果: {}", result3);
    }

    // 包装成 #[tokio::test] 函数
    #[tokio::test]
    async fn test_basic_race() {
        println!("\n=== 实际应用：竞速下载 ===");
        run_basic_race().await;
    }

    #[tokio::test]
    async fn test_select_with_stream() {
        println!("\n=== select!与Stream结合和complete ===");
        run_select_with_stream().await;
    }

    #[tokio::test]
    async fn test_select_default() {
        println!("\n=== select!default分支 ===");
        run_select_default().await;
    }

    #[tokio::test]
    async fn test_select_with_loop() {
        println!("\n=== select!与循环结合 ===");
        run_select_with_loop().await;
    }

    #[tokio::test]
    async fn test_add_two_streams() {
        println!("\n=== 使用select!合并两个Stream ===");
        run_add_two_streams().await;
    }

    // 运行所有测试
    #[tokio::test]
    async fn all_tests() {
        println!("\n===== 运行所有测试 =====\n");
        run_basic_race().await;
        println!();
        run_select_with_stream().await;
        println!();
        run_select_default().await;
        println!();
        run_select_with_loop().await;
        println!();
        run_add_two_streams().await;
        println!("\n===== 所有测试执行完成 =====");
    }
}
