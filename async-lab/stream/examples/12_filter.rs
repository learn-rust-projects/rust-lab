use tokio_stream::StreamExt;

#[tokio::main]
async fn main() {
    let stream = tokio_stream::iter(1..=5);
    let filtered = stream.filter(|&x| x % 2 == 0);
    let result: Vec<_> = filtered.collect().await;
    println!("Filter: {:?}", result);
}