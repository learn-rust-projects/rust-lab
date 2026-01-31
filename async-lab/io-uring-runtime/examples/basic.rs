use std::io::Write;

use io_uring_runtime::{AsyncFile, Reactor};

fn main() {
    let reactor = Reactor::new().unwrap();

    // 创建一个临时文件用于测试
    let mut temp = tempfile::NamedTempFile::new().unwrap();
    temp.write_all(b"Hello from io_uring!").unwrap();
    let path = temp.path().to_owned();

    println!("Reading from file: {:?}", path);

    reactor.block_on(async {
        let file = AsyncFile::open(&path).expect("Failed to open file");
        let mut buf = vec![0u8; 32];

        match file.read(&mut buf).await {
            Ok(n) => {
                let content = String::from_utf8_lossy(&buf[..n]);
                println!("Read {} bytes: {}", n, content);
            }
            Err(e) => eprintln!("Error reading file: {}", e),
        }
    });
}
