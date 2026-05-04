use tokio_stream::StreamExt;

#[tokio::main]
async fn main() {
    let v = vec![1, 2, 3];
    let result = tokio_stream::iter(&v).all(|x| *x > 0).await;
    println!("all (全部 > 0): {}", result);

    let result = tokio_stream::iter(&v).all(|x| *x > 2).await;
    println!("all (全部 > 2): {}", result);
}