use tokio_stream::StreamExt;

#[tokio::main]
async fn main() {
    let s1 = tokio_stream::iter(vec![1, 2]);
    let s2 = tokio_stream::iter(vec![3, 4]);

    let merged = s1.merge(s2);
    let result: Vec<_> = merged.collect().await;
    println!("Merge: {:?}", result);
    // 交替合并两个流，元素交叉返回（谁先 poll 到谁先返回）
}