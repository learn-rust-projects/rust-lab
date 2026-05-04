use tokio_stream::StreamExt;

#[tokio::main]
async fn main() {
    let stream = tokio_stream::iter(1..=5);
    let mapped = stream.map_while(|x| {
        if x < 4 { Some(x * 10) } else { None }
    });
    let result: Vec<_> = mapped.collect().await;
    println!("MapWhile: {:?}", result);
}