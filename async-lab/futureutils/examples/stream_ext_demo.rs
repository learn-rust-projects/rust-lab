use core::pin::Pin;
use futures::future;
use futures::stream::{self, StreamExt};

struct StreamExtDemo;

impl StreamExtDemo {
    // 1. next - 获取流中的下一个元素
    async fn test_next() {
        println!("\n--- test_next ---");
        let mut stream = stream::iter(1..=3);
        assert_eq!(stream.next().await, Some(1));
        assert_eq!(stream.next().await, Some(2));
        assert_eq!(stream.next().await, Some(3));
        assert_eq!(stream.next().await, None);
        println!("test_next passed!");
    }

    // 2. map - 映射流的元素到不同类型
    async fn test_map() {
        println!("\n--- test_map ---");
        let stream = stream::iter(1..=3);
        let result: Vec<i32> = stream.map(|x| x + 10).collect().await;
        assert_eq!(result, vec![11, 12, 13]);
        println!("test_map passed! result: {:?}", result);
    }

    // 3. filter - 过滤流元素
    async fn test_filter() {
        println!("\n--- test_filter ---");
        let stream = stream::iter(1..=10);
        let result: Vec<i32> = stream.filter(|x| future::ready(x % 2 == 0)).collect().await;
        assert_eq!(result, vec![2, 4, 6, 8, 10]);
        println!("test_filter passed! result: {:?}", result);
    }

    // 4. filter_map - 过滤并映射
    async fn test_filter_map() {
        println!("\n--- test_filter_map ---");
        let stream = stream::iter(1..=5);
        let result: Vec<i32> = stream
            .filter_map(|x| async move { if x % 2 == 0 { Some(x * 2) } else { None } })
            .collect()
            .await;
        assert_eq!(result, vec![4, 8]);
        println!("test_filter passed! result: {:?}", result);
    }

    // 5. then - 异步映射
    async fn test_then() {
        println!("\n--- test_then ---");
        let stream = stream::iter(1..=3);
        let result: Vec<i32> = stream.then(|x| async move { x * 2 }).collect().await;
        assert_eq!(result, vec![2, 4, 6]);
        println!("test_then passed! result: {:?}", result);
    }

    // 6. collect - 收集到集合
    async fn test_collect() {
        println!("\n--- test_collect ---");
        let stream = stream::iter(1..=5);
        let result: Vec<i32> = stream.collect().await;
        assert_eq!(result, vec![1, 2, 3, 4, 5]);
        println!("test_collect passed! result: {:?}", result);
    }

    // 7. fold - 累加计算
    async fn test_fold() {
        println!("\n--- test_fold ---");
        let stream = stream::iter(1..=5);
        let sum = stream.fold(0, |acc, x| async move { acc + x }).await;
        assert_eq!(sum, 15);
        println!("test_fold passed! sum: {}", sum);
    }

    // 8. enumerate - 带索引的迭代
    async fn test_enumerate() {
        println!("\n--- test_enumerate ---");
        let stream = stream::iter(vec!['a', 'b', 'c']);
        let result: Vec<(usize, char)> = stream.enumerate().collect().await;
        assert_eq!(result, vec![(0, 'a'), (1, 'b'), (2, 'c')]);
        println!("test_enumerate passed! result: {:?}", result);
    }

    // 9. take - 取前n个元素
    async fn test_take() {
        println!("\n--- test_take ---");
        let stream = stream::iter(1..=100);
        let result: Vec<i32> = stream.take(5).collect().await;
        assert_eq!(result, vec![1, 2, 3, 4, 5]);
        println!("test_take passed! result: {:?}", result);
    }

    // 10. skip - 跳过前n个元素
    async fn test_skip() {
        println!("\n--- test_skip ---");
        let stream = stream::iter(1..=10);
        let result: Vec<i32> = stream.skip(5).collect().await;
        assert_eq!(result, vec![6, 7, 8, 9, 10]);
        println!("test_skip passed! result: {:?}", result);
    }

    // 11. cycle - 循环重复流
    async fn test_cycle() {
        println!("\n--- test_cycle ---");
        let stream = stream::iter(vec![1, 2, 3]);
        let result: Vec<i32> = stream.cycle().take(9).collect().await;
        assert_eq!(result, vec![1, 2, 3, 1, 2, 3, 1, 2, 3]);
        println!("test_cycle passed! result: {:?}", result);
    }

    // 12. flatten - 展平嵌套流
    async fn test_flatten() {
        println!("\n--- test_flatten ---");
        // 创建嵌套流: 流中的每个元素本身也是流
        let nested_stream = stream::iter(vec![stream::iter(vec![1, 2]), stream::iter(vec![3, 4])]);
        let result: Vec<i32> = nested_stream.flatten().collect().await;
        assert_eq!(result, vec![1, 2, 3, 4]);
        println!("test_flatten passed! result: {:?}", result);
    }

    // 13. flat_map - 映射并展平
    async fn test_flat_map() {
        println!("\n--- test_flat_map ---");
        let stream = stream::iter(1..=3);
        let result: Vec<i32> = stream
            .flat_map(|x| stream::iter(vec![x; x as usize]))
            .collect()
            .await;
        assert_eq!(result, vec![1, 2, 2, 3, 3, 3]);
        println!("test_flat_map passed! result: {:?}", result);
    }

    // 14. zip - 合并两个流
    async fn test_zip() {
        println!("\n--- test_zip ---");
        let stream1 = stream::iter(1..=3);
        let stream2 = stream::iter(vec!['a', 'b', 'c']);
        let result: Vec<(i32, char)> = stream1.zip(stream2).collect().await;
        assert_eq!(result, vec![(1, 'a'), (2, 'b'), (3, 'c')]);
        println!("test_zip passed! result: {:?}", result);
    }

    // 15. chain - 连接两个流
    async fn test_chain() {
        println!("\n--- test_chain ---");
        let stream1 = stream::iter(1..=3);
        let stream2 = stream::iter(4..=6);
        let result: Vec<i32> = stream1.chain(stream2).collect().await;
        assert_eq!(result, vec![1, 2, 3, 4, 5, 6]);
        println!("test_chain passed! result: {:?}", result);
    }

    // 16. any - 任意元素满足条件
    async fn test_any() {
        println!("\n--- test_any ---");
        let stream = stream::iter(1..=10);
        let result = stream.any(|x| async move { x == 5 }).await;
        assert!(result);
        println!("test_any passed! result: {}", result);
    }

    // 17. all - 所有元素满足条件
    async fn test_all() {
        println!("\n--- test_all ---");
        let stream = stream::iter(1..=10);
        let result = stream.all(|x| async move { x < 20 }).await;
        assert!(result);
        println!("test_all passed! result: {}", result);
    }

    // 18. count - 计数
    async fn test_count() {
        println!("\n--- test_count ---");
        let stream = stream::iter(1..=10);
        let count = stream.count().await;
        assert_eq!(count, 10);
        println!("test_count passed! count: {}", count);
    }

    // 19. concat - 连接所有元素
    // 聚合集合内容: 合并流中的所有元素，返回一个新的流
    //  flatten 展平流结构
    async fn test_concat() {
        println!("\n--- test_concat ---");
        let stream = stream::iter(vec![vec![1, 2], vec![3, 4], vec![5]]);
        let result: Vec<i32> = stream.concat().await;
        assert_eq!(result, vec![1, 2, 3, 4, 5]);
        println!("test_concat passed! result: {:?}", result);
    }

    // 20. fuse - 熔断流
    async fn test_fuse() {
        println!("\n--- test_fuse ---");
        let stream = stream::iter(1..=3);
        let mut fused = stream.fuse();
        assert_eq!(fused.next().await, Some(1));
        assert_eq!(fused.next().await, Some(2));
        assert_eq!(fused.next().await, Some(3));
        assert_eq!(fused.next().await, None);
        assert_eq!(fused.next().await, None); // 不会panic
        println!("test_fuse passed!");
    }

    // 21. peekable - 可窥视的流
    // peek指向内部引用，所以需要pin住流，此时move，peek返回的元素就成了悬垂引用
    async fn test_peekable() {
        println!("\n--- test_peekable ---");
        use futures::stream::StreamExt;
        let stream = stream::iter(1..=5);
        let mut peekable = stream.peekable();
        assert_eq!(Pin::new(&mut peekable).peek().await, Some(&1));
        assert_eq!(peekable.next().await, Some(1));
        assert_eq!(Pin::new(&mut peekable).peek().await, Some(&2));
        println!("test_peekable passed!");
    }

    // 22. chunks - 分块收集
    async fn test_chunks() {
        println!("\n--- test_chunks ---");
        let stream = stream::iter(1..=10);
        let result: Vec<Vec<i32>> = stream.chunks(3).collect().await;
        assert_eq!(
            result,
            vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9], vec![10]]
        );
        println!("test_chunks passed! result: {:?}", result);
    }

    // 23. scan - 带状态的扫描
    async fn test_scan() {
        println!("\n--- test_scan ---");
        let stream = stream::iter(1..=5);
        let result: Vec<i32> = stream
            .scan(0, |state, x| {
                future::ready({
                    *state += x;
                    if *state < 10 { Some(x) } else { None }
                })
            })
            .collect()
            .await;
        assert_eq!(result, vec![1, 2, 3]);
        println!("test_scan passed! result: {:?}", result);
    }

    // 24. skip_while - 跳过满足条件的元素
    async fn test_skip_while() {
        println!("\n--- test_skip_while ---");
        let stream = stream::iter(1..=10);
        let result: Vec<i32> = stream.skip_while(|x| future::ready(*x < 5)).collect().await;
        assert_eq!(result, vec![5, 6, 7, 8, 9, 10]);
        println!("test_skip_while passed! result: {:?}", result);
    }

    // 25. take_while - 获取满足条件的元素
    async fn test_take_while() {
        println!("\n--- test_take_while ---");
        let stream = stream::iter(1..=10);
        let result: Vec<i32> = stream.take_while(|x| future::ready(*x < 5)).collect().await;
        assert_eq!(result, vec![1, 2, 3, 4]);
        println!("test_take_while passed! result: {:?}", result);
    }

    // 26. inspect - 检查元素
    async fn test_inspect() {
        println!("\n--- test_inspect ---");
        let mut sum = 0;
        let stream = stream::iter(1..=5);
        let result: Vec<i32> = stream
            .inspect(|x| {
                sum += x;
            })
            .collect()
            .await;
        assert_eq!(result, vec![1, 2, 3, 4, 5]);
        assert_eq!(sum, 15);
        println!("test_inspect passed! sum: {}", sum);
    }

    // 27. into_future - 转换为Future
    async fn test_into_future() {
        println!("\n--- test_into_future ---");
        let stream = stream::iter(1..=3);
        let (item, rest) = stream.into_future().await;
        assert_eq!(item, Some(1));
        let (item, _) = rest.into_future().await;
        assert_eq!(item, Some(2));
        println!("test_into_future passed!");
    }

    // 28. unzip - 拆分流
    async fn test_unzip() {
        println!("\n--- test_unzip ---");
        let stream = stream::iter(vec![(1, 'a'), (2, 'b'), (3, 'c')]);
        let (nums, chars): (Vec<i32>, Vec<char>) = stream.unzip().await;
        assert_eq!(nums, vec![1, 2, 3]);
        assert_eq!(chars, vec!['a', 'b', 'c']);
        println!("test_unzip passed!");
    }
}

#[tokio::main]
async fn main() {
    println!("=== StreamExt Trait Demo ===");

    StreamExtDemo::test_next().await;
    StreamExtDemo::test_map().await;
    StreamExtDemo::test_filter().await;
    StreamExtDemo::test_filter_map().await;
    StreamExtDemo::test_then().await;
    StreamExtDemo::test_collect().await;
    StreamExtDemo::test_fold().await;
    StreamExtDemo::test_enumerate().await;
    StreamExtDemo::test_take().await;
    StreamExtDemo::test_skip().await;
    StreamExtDemo::test_cycle().await;
    StreamExtDemo::test_flatten().await;
    StreamExtDemo::test_flat_map().await;
    StreamExtDemo::test_zip().await;
    StreamExtDemo::test_chain().await;
    StreamExtDemo::test_any().await;
    StreamExtDemo::test_all().await;
    StreamExtDemo::test_count().await;
    StreamExtDemo::test_concat().await;
    StreamExtDemo::test_fuse().await;
    StreamExtDemo::test_peekable().await;
    StreamExtDemo::test_chunks().await;
    StreamExtDemo::test_scan().await;
    StreamExtDemo::test_skip_while().await;
    StreamExtDemo::test_take_while().await;
    StreamExtDemo::test_inspect().await;
    StreamExtDemo::test_into_future().await;
    StreamExtDemo::test_unzip().await;

    println!("\n=== All tests passed! ===");
}
