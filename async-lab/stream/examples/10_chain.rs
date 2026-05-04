use tokio_stream::StreamExt;

#[tokio::main]
async fn main() {
    let s1 = tokio_stream::iter(vec![1, 2]);
    let s2 = tokio_stream::iter(vec![3, 4]);

    let chain = s1.chain(s2);
    let result: Vec<_> = chain.collect().await;
    println!("Chain: {:?}", result);
}
