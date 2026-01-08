fn main() {
    // 模拟一段包含 \r (回车) 的数据
    let data = b"Hello World\r\nNext Line";
    let mut iter = data.iter();

    // 执行你的逻辑
    // by_ref() 确保我们不夺走 iter 的所有权，这样后面还能继续用 iter
    // &&b 是因为 iter() 产生 &u8，take_while 产生 &&u8
    let line = iter
        .by_ref()
        .take_while(|&&b| b != b'\r')
        .copied()
        .collect::<Vec<_>>();

    // 验证结果
    println!(
        "提取出的内容 (String): {:?}",
        String::from_utf8_lossy(&line)
    );
    println!("提取出的内容 (Bytes): {:?}", line);

    // 观察原迭代器现在指向哪里
    let next = iter.next();
    println!("迭代器剩下的第一个元素: {:?}", next.map(|&b| b as char));
}
