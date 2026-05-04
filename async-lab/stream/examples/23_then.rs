use tokio_stream::StreamExt;

#[tokio::main]
async fn main() {
    let stream = tokio_stream::iter(1..=3);
    let processed = stream.then(|x| async move { x * 10 });
    let result: Vec<_> = processed.collect().await;
    println!("Then: {:?}", result);
}
