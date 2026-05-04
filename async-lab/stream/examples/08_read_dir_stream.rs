use tokio_stream::{StreamExt, wrappers::ReadDirStream};
use tokio::fs;

#[tokio::main]
async fn main() {
    let mut stream = ReadDirStream::new(fs::read_dir(".").await.unwrap());

    println!("目录项:");
    while let Some(entry) = stream.next().await {
        if let Ok(entry) = entry {
            println!("  {}", entry.file_name().to_string_lossy());
        }
    }
}