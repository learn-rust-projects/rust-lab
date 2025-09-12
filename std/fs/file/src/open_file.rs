#![allow(unused)]

#[cfg(test)]
mod tests {
    use std::{
        error::Error,
        fs::{self, File, OpenOptions, read},
        io::{BufRead, BufReader, Read, Seek, SeekFrom, Write},
    };
    #[test]
    fn open_file() -> Result<(), Box<dyn Error>> {
        //[创建文件-1：覆盖写入]
        // 文件不存在就创建  有就清空
        let mut f = File::create("foo.txt")?;
        // [创建/打开文件-2：追加写入]
        // 没有就创建（可选） 有就追加
        let mut f = OpenOptions::new()
            .append(true)
            .create(true)
            .open("foo.txt")?;

        //[文件句柄-1]
        println!("{:?}", f);

        //[写入-1：write]
        // 返回写入的字数数，不保证一次都写入,需要检查数据确保全部写入：手动构造循环
        let _ = f.write(&[56u8; 4])?;
        //[写入-2：write_all]
        // 内部构建写入，保证全部写入
        // 写入&[u8]
        f.write_all(&[56u8; 4])?;
        f.write_all(b"hello world\n")?;
        // 写入&str
        f.write_all("中文".as_bytes())?;
        writeln!(&mut f, "hello world\n").unwrap();

        read_file()?;

        // [删除文件-1]
        fs::remove_file("foo.txt");
        Ok(())
    }

    fn read_file() -> Result<(), Box<dyn Error>> {
        // [打开文件-1:只读模式]
        // 打开文件，读取为字节，找不到文件会报错
        let mut f = File::open("foo.txt")?;
        let mut data = Vec::new();

        // [读取文件-1：读取到vec]
        // 读取整个文件内容到 Vec 中
        f.read_to_end(&mut data)?;
        // 由于 Deref 的存在，Rust 允许把 &Vec<T> 自动解引用成 &[T]。
        let content = String::from_utf8_lossy(&data);
        println!("read_to_end {:?}", content);

        // [读取文件-2：回到文件开头]
        f.seek(SeekFrom::Start(0))?;

        // [读取文件-3：read：File::open + read_to_end]
        // 使用 File::open 和 read_to_end 且导入次数较少且没有中间变量的便捷函数。
        let mut f = fs::read("foo.txt")?;
        let content = String::from_utf8_lossy(&f);
        println!("read {:?}", content);

        // [读取文件-4：fs::read_to_string]
        // 使用 fs::read_to_string 读取文件内容并直接转换为字符串。
        let content = fs::read_to_string("foo.txt")?;
        println!("read_to_string :{} {}", content, content.len());

        // 【读取文件】示例 4
        // 只读模式打开文件，读取为字节
        let mut f = File::open("foo.txt")?;
        let mut buf = [0u8; 4];

        // 固定长度读取，必须填满，如果文件长度不足 4 字节 → 返回
        // Err(ErrorKind::UnexpectedEof)
        f.read_exact(&mut buf)?;
        println!("{:?}", buf); // 输出 [0, 0, 0, 0]

        // [读取行]
        let mut reader = BufReader::new(f);
        let mut line = String::new();
        // [读取多行]
        for line in reader.lines().enumerate() {
            println!("line:{:?}---content:{:?}", line.0, line.1?);
        }
        Ok(())
    }

    // [写完就读的例子]
    #[test]
    fn create_file_then_read() -> Result<(), Box<dyn Error>> {
        use std::env;
        let mut temp_dir = env::temp_dir();
        let temp_file = temp_dir.join("temp_file");
        let mut file: File = OpenOptions::new()
        .read(true)   // 开启读权限
        .write(true)  // 开启写权限
        .create(true) // 如果不存在就创建
        .truncate(true) // 如果存在就清空
        .open(&temp_file)?; // 打开文件
        // [writeln! 直接写入字符串]
        // 引入use std::io::Write;
        // 如果使用 直接 File（非 BufWriter），写操作已经在内核缓冲区，flush
        // 通常不是必须。
        writeln!(&mut file, "hello world\n")?;

        // 使用指针调整到开头才能读
        use std::io::{Seek, SeekFrom};
        file.seek(SeekFrom::Start(0))?;

        // [读取文件-4：fs::read_to_string]
        // 尝试读取
        let mut buf = String::new();
        let res = file.read_to_string(&mut buf);
        println!("read result: {:?}", buf); // Err(Bad file descriptor)

        fs::remove_file(&temp_file);
        Ok(())
    }
}
