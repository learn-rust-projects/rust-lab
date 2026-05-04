use tokio_stream::StreamExt;

#[tokio::main]
async fn main() {
    let stream = tokio_stream::iter(1..=10);
    let taken = stream.take(3);
    let result: Vec<_> = taken.collect().await;
    println!("Take: {:?}", result);
}