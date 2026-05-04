use std::time::Duration;

use tokio_stream::{StreamExt, wrappers::IntervalStream};

#[tokio::main]
async fn main() {
    println!("=== Test 1: No items time out ===");
    test_no_timeout().await;

    println!("\n=== Test 2: Second item times out, continue polling ===");
    test_continue_after_timeout().await;

    println!("\n=== Test 3: Stop on first timeout with take_while ===");
    test_stop_on_timeout().await;

    println!("\n=== Test 4: Timeouts do not repeat (only one timeout between values) ===");
    test_no_repeat_timeout().await;
}

async fn test_no_timeout() {
    let int_stream = tokio_stream::iter(vec![1, 2, 3]).timeout(Duration::from_secs(1));

    let result: Vec<_> = int_stream.collect().await;
    println!("Result: {:?}", result);
    assert_eq!(result, vec![Ok(1), Ok(2), Ok(3)]);
}

async fn test_continue_after_timeout() {
    let int_stream = tokio_stream::iter(vec![1, 2, 3])
        .then(|v| async move {
            println!("  Processing item: {}", v);
            if v == 2 {
                tokio::time::sleep(Duration::from_millis(100)).await;
            }
            v
        })
        .timeout(Duration::from_millis(10));

    tokio::pin!(int_stream);

    let r1 = int_stream.try_next().await;
    println!("  Result 1: {:?}", r1);
    assert_eq!(r1, Ok(Some(1)));

    let r2 = int_stream.try_next().await;
    println!("  Result 2 (should timeout): {:?}", r2.is_err());
    assert!(r2.is_err());

    let r3 = int_stream.try_next().await;
    println!("  Result 3: {:?}", r3);
    assert_eq!(r3, Ok(Some(2)));

    let r4 = int_stream.try_next().await;
    println!("  Result 4: {:?}", r4);
    assert_eq!(r4, Ok(Some(3)));

    let r5 = int_stream.try_next().await;
    println!("  Result 5: {:?}", r5);
    assert_eq!(r5, Ok(None));

    println!("  Test 2 passed!");
}

async fn test_stop_on_timeout() {
    let int_stream = tokio_stream::iter(vec![1, 2, 3])
        .then(|v| async move {
            println!("  Processing item: {}", v);
            if v == 2 {
                tokio::time::sleep(Duration::from_millis(100)).await;
            }
            v
        })
        .timeout(Duration::from_millis(10))
        .take_while(Result::is_ok);

    tokio::pin!(int_stream);

    let r1 = int_stream.try_next().await;
    println!("  Result 1: {:?}", r1);
    assert_eq!(r1, Ok(Some(1)));

    let r2 = int_stream.try_next().await;
    println!(
        "  Result 2 (should be None because of take_while): {:?}",
        r2
    );
    assert_eq!(r2, Ok(None));

    println!("  Test 3 passed!");
}

async fn test_no_repeat_timeout() {
    let interval_stream = IntervalStream::new(tokio::time::interval(Duration::from_millis(100)));
    let timeout_stream = interval_stream.timeout(Duration::from_millis(10));
    tokio::pin!(timeout_stream);

    assert!(timeout_stream.try_next().await.is_ok());
    assert!(
        timeout_stream.try_next().await.is_err(),
        "expected one timeout"
    );
    assert!(
        timeout_stream.try_next().await.is_ok(),
        "expected no more timeouts"
    );

    tokio::time::sleep(Duration::from_millis(100)).await;

    assert!(timeout_stream.try_next().await.is_ok());
    assert!(
        timeout_stream.try_next().await.is_err(),
        "expected one timeout"
    );
    assert!(
        timeout_stream.try_next().await.is_ok(),
        "expected no more timeouts"
    );

    println!("Test passed: timeouts do not repeat");
}
