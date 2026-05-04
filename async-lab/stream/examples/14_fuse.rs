use futures::stream::StreamExt;

#[tokio::main]
async fn main() {
    let stream = futures::stream::iter(vec![1, 2, 3]);
    let fused = stream.fuse();
    let result: Vec<_> = fused.collect().await;
    println!("Fuse: {:?}", result);
}