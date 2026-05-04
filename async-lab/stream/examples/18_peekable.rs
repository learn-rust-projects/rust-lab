use tokio_stream::StreamExt;

#[tokio::main]
async fn main() {
    let mut stream = tokio_stream::iter(vec![1, 2, 3]).peekable();

    println!("Peek 1: {}", stream.peek().await.unwrap());
    println!("Peek 2: {}", stream.peek().await.unwrap());
    println!("Next: {}", stream.next().await.unwrap());
}