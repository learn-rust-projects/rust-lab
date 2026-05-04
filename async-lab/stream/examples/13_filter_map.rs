use tokio_stream::StreamExt;

#[tokio::main]
async fn main() {
    let stream = tokio_stream::iter(1..=5);
    let filtered = stream.filter_map(|x| if x % 2 == 0 { Some(x * 10) } else { None });
    let result: Vec<_> = filtered.collect().await;
    println!("FilterMap: {:?}", result);
}