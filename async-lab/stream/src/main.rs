//! Stream trait使用案例

use std::{
    pin::Pin,
    task::{Context, Poll},
};

use futures::{Stream, StreamExt};
use tokio::time::{Duration, sleep};

/// 1. 自定义同步Stream实现
struct CounterStream {
    count: u32,
    max: u32,
}

impl CounterStream {
    fn new(max: u32) -> Self {
        Self { count: 0, max }
    }
}

impl Stream for CounterStream {
    type Item = u32;

    fn poll_next(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        if self.count < self.max {
            let current = self.count;
            self.count += 1;
            Poll::Ready(Some(current))
        } else {
            Poll::Ready(None)
        }
    }
}

/// 2. 自定义异步Stream实现
struct AsyncCounterStream {
    count: u32,
    max: u32,
}

impl AsyncCounterStream {
    fn new(max: u32) -> Self {
        Self { count: 0, max }
    }
}

impl Stream for AsyncCounterStream {
    type Item = u32;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        if self.count < self.max {
            let current = self.count;
            self.count += 1;

            // 模拟异步延迟
            let waker = cx.waker().clone();
            tokio::spawn(async move {
                sleep(Duration::from_millis(100)).await;
                waker.wake();
            });

            Poll::Ready(Some(current))
        } else {
            Poll::Ready(None)
        }
    }
}

/// 3. 使用Stream组合器
async fn test_stream_combinators() {
    println!("=== Stream组合器测试 ===");

    // 使用futures::stream::iter创建Stream
    let stream = futures::stream::iter(1..=5);

    // map转换
    let mapped: Vec<_> = stream.map(|x| x * 2).collect().await;
    println!("map转换结果: {:?}", mapped);

    // filter过滤
    let filtered: Vec<_> = futures::stream::iter(1..=10)
        .filter(|&x| async move { x % 2 == 0 })
        .collect()
        .await;
    println!("filter过滤结果: {:?}", filtered);

    // take取前n个
    let taken: Vec<_> = futures::stream::iter(1..=100).take(3).collect().await;
    println!("take取前3个: {:?}", taken);

    // fold累加
    let sum: u32 = futures::stream::iter(1..=5)
        .fold(0, |acc, x| async move { acc + x })
        .await;
    println!("fold累加结果: {}", sum);
}

/// 4. 文件读取Stream示例
struct FileLineStream {
    lines: Vec<String>,
    index: usize,
}

impl FileLineStream {
    fn new(content: &str) -> Self {
        let lines = content.lines().map(|s| s.to_string()).collect();
        Self { lines, index: 0 }
    }
}

impl Stream for FileLineStream {
    type Item = String;

    fn poll_next(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        if self.index < self.lines.len() {
            let line = self.lines[self.index].clone();
            self.index += 1;
            Poll::Ready(Some(line))
        } else {
            Poll::Ready(None)
        }
    }
}

/// 5. 错误处理Stream
struct ResultStream {
    items: Vec<Result<i32, &'static str>>,
    index: usize,
}

impl ResultStream {
    fn new() -> Self {
        Self {
            items: vec![Ok(1), Err("错误1"), Ok(2), Err("错误2"), Ok(3)],
            index: 0,
        }
    }
}

impl Stream for ResultStream {
    type Item = Result<i32, &'static str>;

    fn poll_next(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        if self.index < self.items.len() {
            let item = self.items[self.index].clone();
            self.index += 1;
            Poll::Ready(Some(item))
        } else {
            Poll::Ready(None)
        }
    }
}

/// 6. 无限Stream示例
struct InfiniteStream {
    current: u32,
}

impl InfiniteStream {
    fn new() -> Self {
        Self { current: 0 }
    }
}

impl Stream for InfiniteStream {
    type Item = u32;

    fn poll_next(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let current = self.current;
        self.current += 1;
        Poll::Ready(Some(current))
    }
}

/// 7. 缓冲Stream示例
struct BufferedStream {
    data: Vec<Vec<i32>>,
    current_chunk: usize,
    current_index: usize,
}

impl BufferedStream {
    fn new() -> Self {
        Self {
            data: vec![vec![1, 2, 3], vec![4, 5], vec![6, 7, 8, 9]],
            current_chunk: 0,
            current_index: 0,
        }
    }
}

impl Stream for BufferedStream {
    type Item = i32;

    fn poll_next(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        if self.current_chunk < self.data.len() {
            if self.current_index < self.data[self.current_chunk].len() {
                let value = self.data[self.current_chunk][self.current_index];
                self.current_index += 1;
                Poll::Ready(Some(value))
            } else {
                self.current_chunk += 1;
                self.current_index = 0;
                self.poll_next(_cx)
            }
        } else {
            Poll::Ready(None)
        }
    }
}

/// 8. 使用tokio_stream工具
async fn test_tokio_stream_utils() {
    println!("\n=== tokio_stream工具测试 ===");

    use tokio_stream::StreamExt;

    // 使用tokio_stream::iter
    let mut stream = tokio_stream::iter(vec![1, 2, 3, 4, 5]);

    while let Some(value) = tokio_stream::StreamExt::next(&mut stream).await {
        println!("从tokio_stream接收: {}", value);
    }

    // 使用interval创建定时Stream
    let mut interval = tokio_stream::wrappers::IntervalStream::new(tokio::time::interval(
        Duration::from_millis(500),
    ));

    println!("开始接收定时事件(3秒)...");
    for _ in 0..6 {
        if let Some(_) = tokio_stream::StreamExt::next(&mut interval).await {
            println!("定时事件触发");
        }
    }
}

/// 9. Stream的并发处理
async fn test_concurrent_stream() {
    println!("\n=== Stream并发处理测试 ===");

    use futures::stream::{FuturesUnordered, StreamExt};

    // 创建多个异步任务
    let tasks = (0..5).map(|i| async move {
        sleep(Duration::from_millis((5 - i) as u64 * 100)).await;
        format!("任务{}完成", i)
    });

    // 使用FuturesUnordered并发执行
    let mut futures: FuturesUnordered<_> = tasks.collect();

    while let Some(result) = futures.next().await {
        println!("{}", result);
    }
}

/// 10. 自定义Stream适配器
struct FilterMapStream<S, F> {
    stream: S,
    predicate: F,
}

impl<S, F, B> Stream for FilterMapStream<S, F>
where
    S: Stream + Unpin,
    F: FnMut(S::Item) -> Option<B> + Unpin,
{
    type Item = B;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let this = self.get_mut(); // 取得 &mut FilterMapStream<S, F>
        loop {
            // 安全地将 stream 包装成 Pin
            let item = { Pin::new(&mut this.stream).poll_next(cx) };

            match item {
                Poll::Ready(Some(value)) => {
                    if let Some(mapped) = (this.predicate)(value) {
                        return Poll::Ready(Some(mapped));
                    }
                    // 如果 predicate 返回 None，继续循环
                }
                Poll::Ready(None) => return Poll::Ready(None),
                Poll::Pending => return Poll::Pending,
            }
        }
    }
}

/// 12. 使用try_next方法计算Result Stream的和
async fn sum_with_try_next(
    mut stream: Pin<&mut dyn Stream<Item = Result<i32, std::io::Error>>>,
) -> Result<i32, std::io::Error> {
    use futures::stream::TryStreamExt; // for `try_next`
    let mut sum = 0;
    while let Some(item) = stream.try_next().await? {
        println!("try_next返回: {:?}", item);
        sum += item;
    }
    Ok(sum)
}
/// 12. 使用try_for_each_concurrent进行并发处理
async fn jump_n_times(num: u8) -> Result<(), std::io::Error> {
    println!("跳跃 {} 次", num);
    // 模拟跳跃操作，num为跳跃次数
    if num == 0 {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "跳跃次数不能为0",
        ));
    }
    tokio::time::sleep(Duration::from_millis(num as u64 * 10)).await;
    Ok(())
}

async fn report_n_jumps(num: u8) -> Result<(), std::io::Error> {
    println!("报告跳跃次数: {}", num);
    // 模拟报告跳跃次数
    if num > 100 {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "跳跃次数过多",
        ));
    }
    tokio::time::sleep(Duration::from_millis(num as u64 * 5)).await;
    Ok(())
}
async fn jump_around(
    mut stream: Pin<&mut dyn Stream<Item = Result<u8, std::io::Error>>>,
) -> Result<(), std::io::Error> {
    use futures::stream::TryStreamExt; // for `try_for_each_concurrent`
    const MAX_CONCURRENT_JUMPERS: usize = 100;

    stream
        .try_for_each_concurrent(MAX_CONCURRENT_JUMPERS, |num| async move {
            jump_n_times(num).await?;
            report_n_jumps(num).await?;
            Ok(())
        })
        .await?;

    Ok(())
}
/// 主测试函数
#[tokio::main]
async fn main() {
    println!("开始Stream trait使用案例测试...\n");

    // 1. 测试自定义同步Stream
    println!("=== 自定义同步Stream测试 ===");
    let mut counter = CounterStream::new(5);
    while let Some(value) = futures::stream::StreamExt::next(&mut counter).await {
        println!("同步计数器: {}", value);
    }

    // 2. 测试自定义异步Stream
    println!("\n=== 自定义异步Stream测试 ===");
    let mut async_counter = AsyncCounterStream::new(3);
    while let Some(value) = futures::stream::StreamExt::next(&mut async_counter).await {
        println!("异步计数器: {}", value);
        sleep(Duration::from_millis(50)).await;
    }

    // 3. 测试Stream组合器
    test_stream_combinators().await;

    // 4. 测试文件读取Stream
    println!("\n=== 文件读取Stream测试 ===");
    let content = "第一行\n第二行\n第三行\n第四行";
    let mut file_stream = FileLineStream::new(content);
    while let Some(line) = futures::stream::StreamExt::next(&mut file_stream).await {
        println!("读取行: {}", line);
    }

    // 5. 测试错误处理Stream
    println!("\n=== 错误处理Stream测试 ===");
    let mut result_stream = ResultStream::new();
    while let Some(result) = futures::stream::StreamExt::next(&mut result_stream).await {
        match result {
            Ok(value) => println!("成功值: {}", value),
            Err(e) => println!("错误: {}", e),
        }
    }

    // 6. 测试无限Stream（有限使用）
    println!("\n=== 无限Stream测试（取前5个）===");
    let mut infinite = InfiniteStream::new();
    for _ in 0..5 {
        if let Some(value) = futures::stream::StreamExt::next(&mut infinite).await {
            println!("无限流值: {}", value);
        }
    }

    // 7. 测试缓冲Stream
    println!("\n=== 缓冲Stream测试 ===");
    let mut buffered = BufferedStream::new();
    while let Some(value) = futures::stream::StreamExt::next(&mut buffered).await {
        println!("缓冲值: {}", value);
    }

    // 8. 测试tokio_stream工具
    test_tokio_stream_utils().await;

    // 9. 测试并发Stream
    test_concurrent_stream().await;

    // 10. 测试自定义Stream适配器
    println!("\n=== 自定义Stream适配器测试 ===");
    let base_stream = futures::stream::iter(1..=10);
    let mut filter_map = FilterMapStream {
        stream: base_stream,
        predicate: |x| if x % 2 == 0 { Some(x * 10) } else { None },
    };

    while let Some(value) = futures::stream::StreamExt::next(&mut filter_map).await {
        println!("过滤映射值: {}", value);
    }
    // 11. 测试try_next方法计算Result Stream的和
    println!("\n=== try_next方法测试 ===");
    let only_errors: Vec<Result<i32, std::io::Error>> = vec![
        Ok(1),
        Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "文件未找到",
        )),
        Err(std::io::Error::new(
            std::io::ErrorKind::PermissionDenied,
            "权限拒绝",
        )),
    ];
    let mut error_only_stream = Box::pin(futures::stream::iter(only_errors));
    let pinned_error_only = Pin::new(&mut error_only_stream);
    let error_only_result = sum_with_try_next(pinned_error_only).await;
    assert!(error_only_result.is_err());
    println!("try_next方法测试结果: {:?}", error_only_result);

    // 12. 测试try_for_each_concurrent并发处理
    println!("\n=== try_for_each_concurrent并发处理测试 ===");
    let jump_stream: Vec<Result<u8, std::io::Error>> =
        vec![Ok(1), Ok(2), Ok(3), Ok(5), Ok(8), Ok(100), Ok(101)];
    let mut jump_stream_pinned = Box::pin(futures::stream::iter(jump_stream));
    let pinned_jump_stream = Pin::new(&mut jump_stream_pinned);
    let jump_result = jump_around(pinned_jump_stream).await;
    println!("try_for_each_concurrent测试结果: {:?}", jump_result);
    println!("\n🎉 所有Stream trait使用案例测试完成！");
}

#[cfg(test)]
mod tests {
    use futures::stream::StreamExt;

    use super::*;

    #[tokio::test]
    async fn test_counter_stream() {
        let mut stream = CounterStream::new(3);
        let mut results = Vec::new();

        while let Some(value) = stream.next().await {
            results.push(value);
        }

        assert_eq!(results, vec![0, 1, 2]);
    }

    #[tokio::test]
    async fn test_file_line_stream() {
        let content = "line1\nline2\nline3";
        let mut stream = FileLineStream::new(content);
        let mut lines = Vec::new();

        while let Some(line) = stream.next().await {
            lines.push(line);
        }

        assert_eq!(lines, vec!["line1", "line2", "line3"]);
    }

    #[tokio::test]
    async fn test_result_stream() {
        let mut stream = ResultStream::new();
        let mut results = Vec::new();

        while let Some(result) = stream.next().await {
            results.push(result);
        }

        assert_eq!(results.len(), 5);
        assert!(results[0].is_ok());
        assert!(results[1].is_err());
    }

    #[tokio::test]
    async fn test_stream_combinators() {
        let stream = futures::stream::iter(1..=5);
        let result: Vec<_> = stream.map(|x| x * 2).collect().await;
        assert_eq!(result, vec![2, 4, 6, 8, 10]);
    }
}
