//! mpsc同步通道测试示例

use std::{sync::mpsc, thread, time::Duration};

/// 测试基本的mpsc通道通信
fn test_basic_mpsc() {
    println!("=== 基本mpsc通道测试 ===");

    // 创建通道
    let (tx, rx) = mpsc::channel();
    let x = "1230";
    // 发送消息s
    tx.send(x).unwrap();

    // 接收消息
    let received = rx.recv().unwrap();
    println!("收到消息: {}", received);

    println!("✓ 基本通道测试通过");
}

/// 测试多线程间的mpsc通信
fn test_multithread_mpsc() {
    println!("\n=== 多线程mpsc测试 ===");

    let (tx, rx) = mpsc::channel();

    // 创建多个生产者线程
    let tx1 = tx.clone();
    let tx2 = tx.clone();

    let handle1 = thread::spawn(move || {
        for i in 1..=3 {
            tx1.send(format!("消息 {} 来自线程1", i)).unwrap();
            thread::sleep(Duration::from_millis(100));
        }
    });

    let handle2 = thread::spawn(move || {
        for i in 1..=3 {
            tx2.send(format!("消息 {} 来自线程2", i)).unwrap();
            thread::sleep(Duration::from_millis(150));
        }
    });

    // 在主线程中接收所有消息
    let mut messages = Vec::new();
    for _ in 0..6 {
        let msg = rx.recv().unwrap();
        messages.push(msg);
        println!("收到: {}", messages.last().unwrap());
    }

    handle1.join().unwrap();
    handle2.join().unwrap();

    assert_eq!(messages.len(), 6);
    println!("✓ 多线程通信测试通过");
}

/// 测试try_recv非阻塞接收
fn test_try_recv() {
    println!("\n=== try_recv非阻塞接收测试 ===");

    let (tx, rx) = mpsc::channel();

    // 初始时通道为空，try_recv应该返回Err
    match rx.try_recv() {
        Ok(msg) => panic!("不应该收到消息: {}", msg),
        Err(mpsc::TryRecvError::Empty) => println!("通道为空 - 正确"),
        Err(mpsc::TryRecvError::Disconnected) => panic!("通道不应该断开"),
    }

    // 发送消息后应该能收到
    tx.send("测试消息").unwrap();

    match rx.try_recv() {
        Ok(msg) => println!("成功收到消息: {}", msg),
        Err(e) => panic!("应该收到消息，但出错: {:?}", e),
    }

    println!("✓ try_recv测试通过");
}

/// 测试recv_timeout超时接收
fn test_recv_timeout() {
    println!("\n=== recv_timeout超时接收测试 ===");

    let (tx, rx) = mpsc::channel();

    // 测试超时情况
    match rx.recv_timeout(Duration::from_millis(100)) {
        Ok(_) => panic!("不应该在超时前收到消息"),
        Err(mpsc::RecvTimeoutError::Timeout) => println!("超时 - 正确"),
        Err(mpsc::RecvTimeoutError::Disconnected) => panic!("通道不应该断开"),
    }

    // 在另一个线程中发送消息
    let tx_clone = tx.clone();
    thread::spawn(move || {
        thread::sleep(Duration::from_millis(50));
        tx_clone.send("延迟消息").unwrap();
    });

    // 这次应该能收到消息
    match rx.recv_timeout(Duration::from_millis(200)) {
        Ok(msg) => println!("成功收到消息: {}", msg),
        Err(e) => panic!("应该收到消息，但出错: {:?}", e),
    }

    println!("✓ recv_timeout测试通过");
}

/// 测试有界通道
fn test_bounded_channel() {
    println!("\n=== 有界通道测试 ===");

    // 创建容量为2的有界通道
    let (tx, rx) = mpsc::sync_channel(2);

    // 发送两个消息应该成功
    tx.send("消息1").unwrap();
    tx.send("消息2").unwrap();

    // 第三个消息应该阻塞（在另一个线程中测试）
    let tx_clone = tx.clone();
    let handle = thread::spawn(move || {
        println!("尝试发送第三个消息...");
        tx_clone.send("消息3").unwrap();
        println!("第三个消息发送成功");
    });

    // 等待一小段时间让发送线程阻塞
    thread::sleep(Duration::from_millis(100));

    // 接收一个消息，释放空间
    let msg1 = rx.recv().unwrap();
    println!("收到: {}", msg1);

    // 等待发送线程完成
    handle.join().unwrap();

    // 接收剩余消息
    let msg2 = rx.recv().unwrap();
    let msg3 = rx.recv().unwrap();
    println!("收到: {}", msg2);
    println!("收到: {}", msg3);

    println!("✓ 有界通道测试通过");
}

/// 测试通道断开连接
fn test_channel_disconnect() {
    println!("\n=== 通道断开连接测试 ===");

    let (tx, rx) = mpsc::channel();

    // 发送一些消息
    tx.send("消息1").unwrap();
    tx.send("消息2").unwrap();

    // 丢弃发送端，断开连接
    drop(tx);

    // 应该能收到已发送的消息
    assert_eq!(rx.recv().unwrap(), "消息1");
    assert_eq!(rx.recv().unwrap(), "消息2");

    // 之后接收应该返回断开错误
    match rx.recv() {
        Ok(_) => panic!("不应该再收到消息"),
        Err(_) => println!("通道已断开 - 正确"),
    }

    println!("✓ 通道断开测试通过");
}

/// 测试迭代器模式接收消息
fn test_iterator_receiver() {
    println!("\n=== 迭代器模式测试 ===");

    let (tx, rx) = mpsc::channel();

    // 发送多个消息
    for i in 1..=5 {
        tx.send(format!("消息{}", i)).unwrap();
    }

    // 使用迭代器接收所有消息
    let mut count = 0;
    for msg in rx {
        count += 1;
        println!("迭代器收到: {}", msg);
        if count == 5 {
            break; // 防止无限循环
        }
    }

    assert_eq!(count, 5);
    println!("✓ 迭代器模式测试通过");
}

/// 测试复杂数据类型传输
fn test_complex_data_types() {
    println!("\n=== 复杂数据类型测试 ===");

    #[derive(Debug, PartialEq)]
    struct ComplexData {
        id: u32,
        name: String,
        values: Vec<i32>,
    }

    let (tx, rx) = mpsc::channel();

    let data = ComplexData {
        id: 1,
        name: "测试数据".to_string(),
        values: vec![1, 2, 3, 4, 5],
    };

    // 发送复杂数据
    tx.send(data).unwrap();

    // 接收并验证数据
    let received = rx.recv().unwrap();
    assert_eq!(received.id, 1);
    assert_eq!(received.name, "测试数据");
    assert_eq!(received.values, vec![1, 2, 3, 4, 5]);

    println!("✓ 复杂数据类型测试通过");
}

/// 测试性能场景 - 大量消息传输
fn test_performance_scenario() {
    println!("\n=== 性能场景测试 ===");

    let (tx, rx) = mpsc::channel();
    const MESSAGE_COUNT: usize = 1000;

    let producer = thread::spawn(move || {
        for i in 0..MESSAGE_COUNT {
            tx.send(i).unwrap();
        }
    });

    let consumer = thread::spawn(move || {
        let mut received_count = 0;
        while let Ok(msg) = rx.recv() {
            received_count += 1;
            if received_count == MESSAGE_COUNT {
                break;
            }
        }
        received_count
    });

    producer.join().unwrap();
    let received_count = consumer.join().unwrap();

    assert_eq!(received_count, MESSAGE_COUNT);
    println!("成功传输 {} 条消息", MESSAGE_COUNT);
    println!("✓ 性能场景测试通过");
}

/// 主测试函数
fn main() {
    println!("开始mpsc同步通道测试...\n");

    test_basic_mpsc();
    test_multithread_mpsc();
    test_try_recv();
    test_recv_timeout();
    test_bounded_channel();
    test_channel_disconnect();
    test_iterator_receiver();
    test_complex_data_types();
    test_performance_scenario();

    println!("\n🎉 所有mpsc同步通道测试完成！");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_channel_communication() {
        let (tx, rx) = mpsc::channel();
        tx.send("test message").unwrap();
        assert_eq!(rx.recv().unwrap(), "test message");
    }

    #[test]
    fn test_multiple_producers() {
        let (tx, rx) = mpsc::channel();
        let tx1 = tx.clone();
        let tx2 = tx.clone();

        let handle1 = thread::spawn(move || {
            tx1.send(1).unwrap();
        });

        let handle2 = thread::spawn(move || {
            tx2.send(2).unwrap();
        });

        handle1.join().unwrap();
        handle2.join().unwrap();

        drop(tx); // 丢弃原始发送端

        let mut messages = Vec::new();
        while let Ok(msg) = rx.recv() {
            messages.push(msg);
        }

        messages.sort();
        assert_eq!(messages, vec![1, 2]);
    }

    #[test]
    fn test_bounded_channel_blocks() {
        let (tx, rx) = mpsc::sync_channel(1);

        tx.send("first").unwrap();

        let tx_clone = tx.clone();
        let handle = thread::spawn(move || {
            tx_clone.send("second").unwrap(); // 应该阻塞直到有空间
        });

        // 确保发送线程开始执行
        thread::sleep(Duration::from_millis(50));

        // 接收第一个消息，释放空间
        assert_eq!(rx.recv().unwrap(), "first");

        handle.join().unwrap();
        assert_eq!(rx.recv().unwrap(), "second");
    }

    #[test]
    fn test_try_recv_behavior() {
        let (tx, rx) = mpsc::channel();

        // 初始为空
        assert!(rx.try_recv().is_err());

        tx.send("message").unwrap();
        assert_eq!(rx.try_recv().unwrap(), "message");

        // 再次为空
        assert!(rx.try_recv().is_err());
    }
}
