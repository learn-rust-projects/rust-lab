use bytes::{Buf, BufMut, Bytes, BytesMut};
/// ```text
/// 
///    Arc ptrs                   ┌─────────┐
///    ________________________ / │ Bytes 2 │
///   /                           └─────────┘
///  /          ┌───────────┐     |         |
/// |_________/ │  Bytes 1  │     |         |
/// |           └───────────┘     |         |
/// |           |           | ___/ data     | tail
/// |      data |      tail |/              |
/// v           v           v               v
/// ┌─────┬─────┬───────────┬───────────────┬─────┐
/// │ Arc │     │           │               │     │
/// └─────┴─────┴───────────┴───────────────┴─────┘
/// ```
fn main() {
    println!("=== Bytes crate 常用方法测试 ===\n");

    // 测试1: 创建Bytes和BytesMut
    test_creation();

    // 测试2: 读写操作
    test_read_write();

    // 测试3: 切片操作
    test_slices();

    // 测试4: 复制和克隆
    test_copy_clone();

    // 测试5: 缓冲区操作
    test_buffer_operations();

    // 测试6: 字节操作
    test_bytes_operations();

    println!("\n=== 所有测试完成 ===");
}

fn test_creation() {
    println!("--- 测试创建 ---");

    // 从Vec<u8>创建Bytes
    let data = vec![1, 2, 3, 4, 5];
    let bytes = Bytes::from(data);
    println!("从Vec创建Bytes: {:?}", bytes);

    // 从字节数组创建
    let array = [10, 20, 30, 40];
    let bytes_from_slice = Bytes::from(array[..].to_vec());
    println!("从数组切片创建Bytes: {:?}", bytes_from_slice);

    // 创建BytesMut
    let bytes_mut = BytesMut::with_capacity(1024);
    println!(
        "创建容量为1024的BytesMut: capacity={}",
        bytes_mut.capacity()
    );

    // 从字符串创建
    let str_bytes = Bytes::from_static(b"Hello, Bytes!");
    println!(
        "从静态字符串创建Bytes: {:?}",
        std::str::from_utf8(&str_bytes).unwrap()
    );
}

fn test_read_write() {
    println!("\n--- 测试读写操作 ---");

    let mut buf = BytesMut::with_capacity(1024);

    // 写入操作
    buf.put_u8(42);
    buf.put_u16(1024);
    buf.put_slice(&[1, 2, 3, 4]);
    buf.put(&b"Hello"[..]);

    println!("写入后的缓冲区: {:?}", &buf[..]);

    // 读取操作 - 修复：直接从Bytes读取，而不是使用Reader
    let mut bytes_data = buf.freeze();
    println!("原始数据: {:?}", std::str::from_utf8(&bytes_data).unwrap());
    let val1 = bytes_data.get_u8();
    let val2 = bytes_data.get_u16();
    let b3 = bytes_data.slice(0..5);
    println!("读取前5个字节: {:?}", std::str::from_utf8(&b3).unwrap());
    println!(
        "读取后剩余数据: {:?}",
        std::str::from_utf8(&bytes_data).unwrap()
    );
    println!("读取u8: {}, 读取u16: {}", val1, val2);
}

fn test_slices() {
    println!("\n--- 测试切片操作 ---");

    let mut buf = BytesMut::from(&b"hello world"[..]);
    println!("原始数据: {:?}", std::str::from_utf8(&buf).unwrap());

    // 切片操作
    let slice1 = buf.split_to(5); // 分割前5个字节
    println!(
        "分割的前5个字节: {:?}",
        std::str::from_utf8(&slice1).unwrap()
    );
    println!("剩余数据: {:?}", std::str::from_utf8(&buf).unwrap());

    // 重置数据
    buf.unsplit(slice1); // 合并回去
    println!("合并后: {:?}", std::str::from_utf8(&buf).unwrap());

    // 使用slice方法
    let sliced = &buf[1..6];
    println!("切片[1..6]: {:?}", std::str::from_utf8(&sliced).unwrap());
}

fn test_copy_clone() {
    println!("\n--- 测试复制和克隆 ---");

    let original = Bytes::from(&b"Hello, World!"[..]);
    println!("原始Bytes: {:?}", std::str::from_utf8(&original).unwrap());

    // 克隆 - 由于Bytes的引用计数特性，这是O(1)操作
    let cloned = original.clone();
    println!("克隆的Bytes: {:?}", std::str::from_utf8(&cloned).unwrap());

    // 验证它们指向相同的数据
    println!("两个Bytes内容相等: {}", original == cloned);

    // 深拷贝到BytesMut
    let mut deep_copy = BytesMut::with_capacity(original.len());
    deep_copy.put(original.as_ref());
    println!(
        "深拷贝到BytesMut: {:?}",
        std::str::from_utf8(&deep_copy).unwrap()
    );
}

fn test_buffer_operations() {
    println!("\n--- 测试缓冲区操作 ---");

    let mut buf = BytesMut::with_capacity(32);

    // 使用put方法写入
    buf.put_u32(0xDEADBEEF);
    buf.extend_from_slice(&[1, 2, 3, 4, 5]);
    println!("初始容量: {}, 初始长度: {}", buf.capacity(), buf.len());
    let x = buf.split();
    println!("split 初始容量: {}, 初始长度: {}", buf.capacity(), buf.len());
    println!("写入后的缓冲区: {:?}", buf);

    // 读取数据 - 修复：直接从Bytes读取
       // 使用put方法写入
    buf.put_u32(0xDEADBEEF);
    buf.extend_from_slice(&[1, 2, 3, 4, 5]);
    let mut bytes_data = buf.freeze();
    let num = bytes_data.get_u32();
    println!("读取u32: 0x{:X}", num);

    // 剩余字节
    let remaining = bytes_data.to_vec();
    println!("剩余字节: {:?}", remaining);

    // 测试reserve方法
    let mut buf2 = BytesMut::with_capacity(10);
    println!("初始容量: {}, 初始长度: {}", buf2.capacity(), buf2.len());
    buf2.reserve(100);
    println!(
        "预留100字节后 - 容量: {}, 长度: {}",
        buf2.capacity(),
        buf2.len()
    );

    // 测试clear
    buf2.clear();
    println!("清空后长度: {}", buf2.len());
}

fn test_bytes_operations() {
    let mut buf = BytesMut::with_capacity(1000);
    buf.extend_from_slice(b"Hello, World!");
    // 需要buf
    buf.put(&b"Rust"[..]);
    buf.put_i32(12313);
    buf.put_u8(0x42);
    println!("{:?}", buf);
    let mut putted = buf.split();
    println!("{:?}", putted);
    let split_off = putted.split_off(putted.len() - 4);
    println!("split_off:{:?}", split_off);
    println!("{:?}", putted);
    let split_to: BytesMut = putted.split_to(6);
    println!("split_to:{:?}", split_to);
    println!("{:?}", putted);
    // 变成只读的数据
    let mut bytes = putted.freeze();
    println!("{:?}", bytes);
    // 返回新bytes，切成更小的单元，但是每个单元数据仍然是不可变的
    let split_to = bytes.split_to(7);
    println!("{:?}", split_to);
    println!("{:?}", bytes);
}

#[cfg(test)]
mod tests {
    use bytes::{Buf, BufMut, Bytes, BytesMut};

    use super::*;

    #[test]
    fn test_bytes_creation() {
        let data = vec![1, 2, 3, 4];
        let bytes = Bytes::from(data);
        assert_eq!(bytes.len(), 4);
        assert_eq!(bytes[0], 1);
    }

    #[test]
    fn test_bytes_mut_operations() {
        let mut buf = BytesMut::with_capacity(10);
        buf.put_u8(42);
        buf.put_u16(1024);

        // 修复：正确读取数据
        let mut bytes_data = buf.freeze();
        assert_eq!(bytes_data.len(), 3);

        let val1 = bytes_data.get_u8();
        assert_eq!(val1, 42);

        let val2 = bytes_data.get_u16();
        assert_eq!(val2, 1024);
    }

    #[test]
    fn test_slice_operations() {
        let mut buf = BytesMut::from(&b"hello world"[..]);
        let slice1 = buf.split_to(5);

        assert_eq!(&slice1[..], b"hello");
        assert_eq!(&buf[..], b" world");
    }

    #[test]
    fn test_bytes_equality() {
        let b1 = Bytes::from_static(b"test");
        let b2 = Bytes::from_static(b"test");
        assert_eq!(b1, b2);
    }

    #[test]
    fn test_buffer_extension() {
        let mut buf = BytesMut::new();
        buf.extend_from_slice(b"hello");
        buf.extend_from_slice(b" ");
        buf.extend_from_slice(b"world");

        assert_eq!(&buf[..], b"hello world");
    }
}
