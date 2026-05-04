use tokio_stream::StreamExt;

#[tokio::main]
async fn main() {
    let stream = tokio_stream::iter(1..=5);
    let skipped = stream.skip_while(|&x| x < 3);
    let result: Vec<_> = skipped.collect().await;
    println!("SkipWhile: {:?}", result);
}