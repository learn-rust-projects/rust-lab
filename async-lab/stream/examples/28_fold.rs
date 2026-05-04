use tokio_stream::StreamExt;

#[tokio::main]
async fn main() {
    let sum = tokio_stream::iter(1..=5).fold(0, |acc, x| acc + x).await;
    println!("fold (求和): {}", sum);
}