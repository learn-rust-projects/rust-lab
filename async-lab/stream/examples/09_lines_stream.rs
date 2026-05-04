use tokio::io::{AsyncBufReadExt, BufReader};

#[tokio::main]
async fn main() {
    let cursor = std::io::Cursor::new("第一行\n第二行\n第三行");
    let reader = BufReader::new(cursor);
    let mut lines = reader.lines();

    while let Some(line) = lines.next_line().await.unwrap() {
        println!("读取行: {}", line);
    }
}