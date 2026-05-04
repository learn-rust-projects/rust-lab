use tokio_stream::StreamExt;

#[tokio::main]
async fn main() {
    let v = vec![1, 2, 3];
    let result = tokio_stream::iter(&v).any(|x| *x > 2).await;
    println!("any (存在 > 2): {}", result);

    let result = tokio_stream::iter(&v).any(|x| *x > 5).await;
    println!("any (存在 > 5): {}", result);
}