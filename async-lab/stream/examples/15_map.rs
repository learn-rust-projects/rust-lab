use tokio_stream::StreamExt;

#[tokio::main]
async fn main() {
    let stream = tokio_stream::iter(1..=3);
    let mapped = stream.map(|x| x * 10);
    let result: Vec<_> = mapped.collect().await;
    println!("Map: {:?}", result);
}