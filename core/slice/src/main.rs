//! 标准库切片[T]方法测试程序
//!
//! 这个程序展示了标准库中切片的各种方法，包括：
//! - 基本操作：len, is_empty, first, last
//! - 索引访问：get, get_mut, get_unchecked
//! - 迭代器：iter, iter_mut, windows, chunks
//! - 搜索和排序：contains, sort, binary_search
//! - 分割操作：split, split_at, split_first
//! - 转换操作：to_vec, to_owned

fn main() {
    println!("=== 标准库切片[T]方法测试程序 ===");

    // 运行所有测试示例
    basic_operations();
    indexing_methods();
    iterator_methods();
    search_and_sort();
    splitting_operations();
    conversion_methods();
    advanced_methods();

    println!("\n🎉 所有切片方法测试完成！");
}

/// 基本操作示例
fn basic_operations() {
    println!("\n=== 基本操作示例 ===");

    let slice = &mut [1, 2, 3, 4, 5];
    println!("切片: {:?}", slice);

    // len - 获取长度
    println!("长度: {}", slice.len());

    // is_empty - 检查是否为空
    println!("是否为空: {}", slice.is_empty());

    // first - 获取第一个元素
    println!("第一个元素: {:?}", slice.first());

    // last - 获取最后一个元素
    println!("最后一个元素: {:?}", slice.last());

    // first - 获取第一个元素
    let first_mut = slice.first_mut().unwrap();
    println!("第一个元素: {:?}", first_mut);

    // last - 获取最后一个元素
    let last_mut = slice.last_mut().unwrap();
    println!("最后一个元素: {:?}", last_mut);

    // 空切片的测试
    let empty_slice: &[i32] = &[];
    println!(
        "空切片长度: {}, 是否为空: {}",
        empty_slice.len(),
        empty_slice.is_empty()
    );
}

/// 索引访问方法示例
fn indexing_methods() {
    println!("\n=== 索引访问方法示例 ===");

    let mut data = [1, 2, 3, 4, 5];
    let slice = &mut data[..];

    println!("原始切片: {:?}", slice);

    // get - 安全获取元素
    println!("索引2的值: {:?}", slice.get(2));
    println!("索引10的值: {:?}", slice.get(10));

    // get_mut - 安全获取可变引用
    if let Some(elem) = slice.get_mut(1) {
        *elem = 20;
        println!("修改索引1后的切片: {:?}", slice);
    }

    // 直接索引访问（可能panic）
    println!("直接索引访问[0]: {}", slice[0]);

    // 使用get_unchecked（不安全，需要unsafe块）
    unsafe {
        let elem = slice.get_unchecked(3);
        println!("get_unchecked(3): {}", elem);
    }
}

/// 迭代器方法示例
fn iterator_methods() {
    println!("\n=== 迭代器方法示例 ===");

    let slice = &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    println!("原始切片: {:?}", slice);

    // iter - 不可变迭代器
    println!("iter遍历:");
    for item in slice.iter() {
        print!("{} ", item);
    }
    println!();

    // windows - 滑动窗口
    println!("windows(3):");
    for window in slice.windows(3) {
        print!("{:?} ", window);
    }
    println!();

    // chunks - 分块
    println!("chunks(4):");
    for chunk in slice.chunks(4) {
        print!("{:?} ", chunk);
    }
    println!();

    // chunks_exact - 精确分块
    // 不满足不会输出了
    println!("chunks_exact(3):");
    for chunk in slice.chunks_exact(4) {
        print!("{:?} ", chunk);
    }
    println!();
}

/// 搜索和排序示例
fn search_and_sort() {
    println!("\n=== 搜索和排序示例 ===");

    let mut data = [5, 2, 8, 1, 9, 3, 7, 4, 6, 10];
    let slice = &mut data[..];

    println!("原始切片: {:?}", slice);

    // contains - 检查是否包含元素
    println!("包含8: {}", slice.contains(&8));
    println!("包含11: {}", slice.contains(&11));

    // sort - 排序（原地）
    slice.sort();
    println!("排序后: {:?}", slice);

    // binary_search - 二分查找
    match slice.binary_search(&7) {
        Ok(index) => println!("找到7在索引: {}", index),
        Err(_) => println!("未找到7"),
    }

    // starts_with - 检查前缀
    println!("以[1,2,3]开头: {}", slice.starts_with(&[1, 2, 3]));

    // ends_with - 检查后缀
    println!("以[8,9,10]结尾: {}", slice.ends_with(&[8, 9, 10]));
}

/// 分割操作示例
fn splitting_operations() {
    println!("\n=== 分割操作示例 ===");

    let slice = &mut [1, 2, 0, 3, 4, 0, 5, 6];
    println!("原始切片: {:?}", slice);

    // split - 按条件分割
    println!("按0分割:");
    for part in slice.split(|&x| x == 0) {
        print!("{:?} ", part);
    }
    println!();

    // split_at - 在指定位置分割
    let (left, right) = slice.split_at(4);
    println!("split_at(4): left={:?}, right={:?}", left, right);

    // split_first - 分割第一个元素
    if let Some((first, rest)) = slice.split_first() {
        println!("split_first: first={}, rest={:?}", first, rest);
    }

    // split_last - 分割最后一个元素
    if let Some((last, rest)) = slice.split_last() {
        println!("split_last: last={}, rest={:?}", last, rest);
    }

    // splitn - 限制分割次数
    println!("splitn(2)按0分割:");
    for part in slice.splitn(2, |&x| x == 0) {
        print!("{:?} ", part);
    }
    // split_first - 分割第一个元素
    if let Some((first, rest)) = slice.split_first() {
        println!("split_first: first={}, rest={:?}", first, rest);
    }
    // split_last - 分割最后一个元素
    if let Some((last, rest)) = slice.split_last() {
        println!("split_last: last={}, rest={:?}", last, rest);
    }
    // split_first_mut - 分割第一个元素（可变）要求切片是可变
    if let Some((first, rest)) = slice.split_first_mut() {
        println!("split_first_mut: first={}, rest={:?}", first, rest);
    }
    // split_last_mut - 分割最后一个元素（可变）要求切片是可变
    if let Some((last, rest)) = slice.split_last_mut() {
        println!("split_last_mut: last={}, rest={:?}", last, rest);
    }
    println!();
}

/// 转换方法示例
fn conversion_methods() {
    println!("\n=== 转换方法示例 ===");

    let slice = &[1, 2, 3, 4, 5];
    println!("原始切片: {:?}", slice);

    // to_vec - 转换为Vec
    let vec = slice.to_vec();
    println!("to_vec结果: {:?}", vec);

    // to_owned - 转换为拥有的数据
    let owned = slice.to_owned();
    println!("to_owned结果: {:?}", owned);

    // 字符串切片的转换示例
    let str_slice = "hello world";
    let string = str_slice.to_string();
    println!("字符串切片: '{}', 转换后: '{}'", str_slice, string);
}

/// 高级方法示例
fn advanced_methods() {
    println!("\n=== 高级方法示例 ===");

    let mut data = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let slice = &mut data[..];
    println!("原始切片: {:?}", slice);

    // swap - 交换两个元素
    slice.swap(0, 9);
    println!("交换索引0和9后: {:?}", slice);

    // reverse - 反转切片
    slice.reverse();
    println!("反转后: {:?}", slice);

    // rotate_left - 左旋转
    slice.rotate_left(3);
    println!("左旋转3位后: {:?}", slice);

    // fill - 填充值
    let mut fill_data = [25; 5];
    fill_data.fill(42);
    println!("fill填充后: {:?}", fill_data);

    // copy_from_slice - 复制切片
    let mut target = [0; 5];
    target.copy_from_slice(&slice[..5]);
    println!("copy_from_slice结果: {:?}", target);

    // clone_from_slice - 克隆切片（需要Clone trait）
    let mut clone_target = [0; 5];
    clone_target.clone_from_slice(&slice[..5]);
    println!("clone_from_slice结果: {:?}", clone_target);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_operations() {
        let slice = &[1, 2, 3, 4, 5];

        assert_eq!(slice.len(), 5);
        assert!(!slice.is_empty());
        assert_eq!(slice.first(), Some(&1));
        assert_eq!(slice.last(), Some(&5));

        let empty: &[i32] = &[];
        assert!(empty.is_empty());
        assert_eq!(empty.first(), None);
    }

    #[test]
    fn test_indexing_methods() {
        let mut data = [1, 2, 3];
        let slice = &mut data[..];

        assert_eq!(slice.get(1), Some(&2));
        assert_eq!(slice.get(5), None);

        if let Some(elem) = slice.get_mut(1) {
            *elem = 20;
        }
        assert_eq!(slice[1], 20);
    }

    #[test]
    fn test_iterator_methods() {
        let slice = &[1, 2, 3, 4, 5];

        let sum: i32 = slice.iter().sum();
        assert_eq!(sum, 15);

        let windows: Vec<&[i32]> = slice.windows(2).collect();
        assert_eq!(windows, vec![&[1, 2], &[2, 3], &[3, 4], &[4, 5]]);

        let chunks: Vec<&[i32]> = slice.chunks(2).collect();
        // assert_eq!(chunks, vec![&[1, 2], &[3, 4], &[5]]);
    }

    #[test]
    fn test_search_and_sort() {
        let mut data = [5, 3, 1, 4, 2];
        let slice = &mut data[..];

        assert!(slice.contains(&3));
        assert!(!slice.contains(&6));

        slice.sort();
        assert_eq!(slice, [1, 2, 3, 4, 5]);

        assert_eq!(slice.binary_search(&3), Ok(2));
        assert!(slice.starts_with(&[1, 2]));
    }

    #[test]
    fn test_splitting_operations() {
        let slice = &[1, 0, 2, 0, 3];

        let parts: Vec<&[i32]> = slice.split(|&x| x == 0).collect();
        assert_eq!(parts, vec![&[1], &[2], &[3]]);

        let (left, right) = slice.split_at(2);
        assert_eq!(left, &[1, 0]);
        assert_eq!(right, &[2, 0, 3]);

        if let Some((first, rest)) = slice.split_first() {
            assert_eq!(*first, 1);
            assert_eq!(rest, &[0, 2, 0, 3]);
        }
    }

    #[test]
    fn test_conversion_methods() {
        let slice = &[1, 2, 3];

        let vec = slice.to_vec();
        assert_eq!(vec, vec![1, 2, 3]);

        let owned = slice.to_owned();
        assert_eq!(owned, vec![1, 2, 3].as_slice());
    }

    #[test]
    fn test_advanced_methods() {
        let mut data = [1, 2, 3, 4, 5];
        let slice = &mut data[..];

        slice.swap(0, 4);
        assert_eq!(slice, &[5, 2, 3, 4, 1]);

        slice.reverse();
        assert_eq!(slice, &[1, 4, 3, 2, 5]);

        let mut fill_data = [0; 3];
        fill_data.fill(10);
        assert_eq!(fill_data, [10, 10, 10]);
    }

    #[test]
    fn test_string_slice_methods() {
        let str_slice = "hello world";

        assert_eq!(str_slice.len(), 11);
        assert_eq!(str_slice.contains("world"), true);

        let parts: Vec<&str> = str_slice.split(' ').collect();
        assert_eq!(parts, vec!["hello", "world"]);

        assert_eq!(str_slice.starts_with("hello"), true);
        assert_eq!(str_slice.ends_with("world"), true);
    }
}
