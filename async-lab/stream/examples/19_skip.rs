use tokio_stream::StreamExt;

#[tokio::main]
async fn main() {
    let stream = tokio_stream::iter(1..=5);
    let skipped = stream.skip(2);
    let result: Vec<_> = skipped.collect().await;
    println!("Skip: {:?}", result);
}