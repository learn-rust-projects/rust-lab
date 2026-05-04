use tokio_stream::StreamExt;

#[tokio::main]
async fn main() {
    let doubled: Vec<i32> = tokio_stream::iter(vec![1, 2, 3])
        .map(|x| x * 2)
        .collect().await;
    println!("collect (Vec): {:?}", doubled);

    let result: Result<Vec<i32>, &str> = tokio_stream::iter(vec![Ok(1), Ok(2), Ok(3)])
        .collect().await;
    println!("collect (Result): {:?}", result);

    let result: Result<Vec<i32>, &str> = tokio_stream::iter(vec![Ok(1), Err("error"), Ok(3)])
        .collect().await;
    println!("collect (Result带错): {:?}", result);
}