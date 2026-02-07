#![feature(slice_as_array)]
#![feature(slice_split_once)]
#![feature(trim_prefix_suffix)]
#![feature(slice_partition_dedup)]
#![feature(substr_range)]
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
    unsafe_methods();
    strip_prefix_method();
    sort_methods();

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
    // -> Option<&T>
    println!("第一个元素: {:?}", slice.first());

    // last - 获取最后一个元素
    // -> Option<&T>
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
fn strip_prefix_method() {
    println!("\n=== strip_prefix方法示例 ===");

    let slice = &[1, 2, 3, 4, 5];
    println!("原始切片: {:?}", slice);
    // starts_with
    println!("是否以[1, 2]开头: {}", slice.starts_with(&[1, 2]));
    // ends_with
    println!("是否以[4, 5]结尾: {}", slice.ends_with(&[4, 5]));

    // strip_prefix - 移除前缀
    // -> Option<&[T]>
    println!("移除前缀[1, 2]: {:?}", slice.strip_prefix(&[1, 2]));
    println!("移除前缀[1, 3]: {:?}", slice.strip_prefix(&[1, 3]));
    // strip_suffix - 移除后缀
    // -> Option<&[T]>
    println!("移除后缀[4, 5]: {:?}", slice.strip_suffix(&[4, 5]));
    println!("移除后缀[3, 4]: {:?}", slice.strip_suffix(&[3, 4]));
    // trim_prefix - 移除前缀
    // -> &[T]
    // trim_prefix：只针对字符串，去掉前缀，如果前缀不存在，则返回
    // 原字符串本身，不会返回 Option。
    // unstable trim_prefix_suffix feature
    // trim_suffix
    println!("移除前缀[1, 2]: {:?}", slice.trim_prefix(&[1, 2]));
    println!("移除前缀[1, 3]: {:?}", slice.trim_prefix(&[1, 3]));
    //     移除前缀[1, 2]: [3, 4, 5]
    // 移除前缀[1, 3]: [1, 2, 3, 4, 5]
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
}
fn unsafe_methods() {
    println!("\n=== 不安全方法示例 ===");

    let mut data = [1, 2, 3, 4, 5];
    let slice = &mut data[..];

    println!("原始切片: {:?}", slice);

    // get_unchecked - 不安全获取元素
    unsafe {
        let elem = slice.get_unchecked(3);
        println!("get_unchecked(3): {}", elem);
    }
    // get_unchecked_mut - 不安全获取可变引用
    unsafe {
        let elem = slice.get_unchecked_mut(3);
        println!("get_unchecked_mut(3): {}", elem);
    }
    // as_ptr - 获取原始指针
    // 返回指向切片第一个元素的原始只读指针 *const T
    let ptr = slice.as_ptr();
    println!("切片指针: {:?}", ptr);
    // as_mut_ptr - 获取原始可变指针
    // 返回指向切片第一个元素的原始可变指针 *mut T
    let mut_ptr = slice.as_mut_ptr();
    println!("切片可变指针: {:?}", mut_ptr);

    // as_ptr_range - 获取原始指针范围
    // 返回指向切片第一个元素的原始只读指针 *const T
    let ptr_range = slice.as_ptr_range();
    println!("切片指针范围: {:?}", ptr_range);
    // as_mut_ptr_range - 获取原始可变指针范围
    // 返回指向切片第一个元素的原始可变指针 *mut T
    let mut_ptr_range = slice.as_mut_ptr_range();
    println!("切片可变指针范围: {:?}", mut_ptr_range);

    // as_array - 获取数组引用
    // 返回指向切片第一个元素的原始数组引用 &[T; N]
    //#![feature(slice_as_array)]
    let array_ref = slice.as_array::<3>();
    println!("数组引用: {:?}", array_ref);
    // as_mut_array
    // 返回指向切片第一个元素的原始数组引用 &mut [T; N]
    let mut_array_ref = slice.as_mut_array::<3>();
    println!("可变数组引用: {:?}", mut_array_ref);
}
/// 迭代器方法示例
fn iterator_methods() {
    println!("\n=== 迭代器方法示例 ===");

    let slice = &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 9];
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
    // chunks_mut
    // rchunks
    // rchunks_mut
    println!("chunks(4):");
    for chunk in slice.chunks(4) {
        print!("{:?} ", chunk);
    }
    println!();

    // chunks_exact - 精确分块
    // chunks_exact_mut
    // rchunks_exact
    // rchunks_exact_mut
    // 不满足指定长度不会输出了
    println!("chunks_exact(4):");
    for chunk in slice.chunks_exact(4) {
        println!("{:?} ", chunk);
    }
    // as_chunks -> (&[[T; N]], &[T]) 数组切片+切片
    // as_chunks_mut
    // as_chunks_unchecked_mut
    // as_rchunks -> (&[T], &[T; N]) 切片+数组切片
    // as_rchunks_mut
    println!("as_chunks(4):{:?}", slice.as_chunks::<4>());
    println!();

    println!("chunk_by(|a, b| a <= b):");
    //     作用：把切片按照相邻元素关系分块。
    // 迭代器输出：每次返回一个不重叠的子切片。
    // chunk_by_mut
    for chunk in slice.chunk_by(|a, b| a <= b) {
        println!("{:?} ", chunk);
    }

    println!("\n=== 访问元素示例 ===");
    // Returns an array reference
    // 不满足返回None
    let slice = &[1, 2, 3, 4];
    // first_chunk - 获取第一个元素 -> Option<&[T; N]>
    // first_chunk_mut -> Option<&mut [T; N]>
    println!("第一个chunk: {:?}", slice.first_chunk::<3>());
    // split_first_chunk -> Option<(&[T; N], &[T])>
    // split_first_chunk_mut -> Option<(&mut [T; N], &mut [T])>
    // split_last_chunk -> Option<(&[T], &[T; N])>
    // split_last_chunk_mut -> Option<(&mut [T], &mut [T; N])>
    println!("split_first_chunk: {:?}", slice.split_first_chunk::<3>());
}

/// 搜索和排序示例
fn search_and_sort() {
    println!("\n=== 搜索和排序示例 ===");

    let mut data = [5, 2, 8, 1, 9, 3, 7, 4, 6, 10];
    let slice = &mut data[..];

    println!("原始切片: {:?}", slice);

    // contains - 检查是否包含元素
    //  x.iter().any(|y| *y == *self)
    println!("包含8: {}", slice.contains(&8));
    println!("包含11: {}", slice.contains(&11));

    // sort - 排序（原地）
    slice.sort();
    println!("排序后: {:?}", slice);

    // binary_search - 二分查找
    match slice.binary_search(&7) {
        Ok(index) => println!("找到7在索引: {}", index),
        Err(index) => println!("未找到7，插入位置: {}", index),
    }
    // binary_search - 二分查找
    match slice.binary_search(&120) {
        Ok(index) => println!("找到120在索引: {}", index),
        Err(index) => println!("未找到120，插入位置: {}", index),
    }
    // binary_search_by - 自定义二分查找
    // binary_search_by_mut

    // 找到第一个大于等于7的元素
    match slice.binary_search_by(|&x| x.cmp(&7)) {
        Ok(index) => println!("找到第一个大于等于7的元素在索引: {}", index),
        Err(index) => println!("未找到大于等于7的元素，插入位置: {}", index),
    }
    // binary_search_by_key - 自定义键值二分查找 等价于 self.binary_search_by(|elem|
    // f(elem).cmp(key))

    // starts_with - 检查前缀
    println!("以[1,2,3]开头: {}", slice.starts_with(&[1, 2, 3]));

    // ends_with - 检查后缀
    println!("以[8,9,10]结尾: {}", slice.ends_with(&[8, 9, 10]));
}

fn sort_methods() {
    println!("\n=== 排序方法示例 ===");
    let slice = &mut [5, 2, 8, 1, 9, 3, 7, 4, 6, 10, 12, 3, 2, 1];
    println!("原始切片: {:?}", slice);
    // sort_unstable - 不稳定排序（性能好）
    slice.sort_unstable();
    // sort_unstable_by - 不稳定排序自定义比较函数
    slice.sort_unstable_by(|a, b| b.cmp(a));
    // sort_unstable_by_key - 不稳定排序自定义键函数
    slice.sort_unstable_by_key(|&x| x % 2);
    println!("不稳定排序后: {:?}", slice);

    // select_nth_unstable
    // select_nth_unstable - 选择第n个元素（不稳定）
    println!(
        "select_nth_unstable(4)前: {:?}",
        slice.select_nth_unstable(4)
    );
    println!("select_nth_unstable(4): {:?}", slice);
    println!(
        "select_nth_unstable_by_key(4)前: {:?}",
        slice.select_nth_unstable_by_key(2, |&x| x % 2)
    );
    // partition_dedup
    // partition_dedup - 去重分区
    // #![feature(slice_partition_dedup)]
    // dedup 就是去掉所有重复元素的唯一集合
    // 顺序保持原有顺序
    // 后面重复元素
    // 顺序不保证
    println!("partition_dedup前: {:?}", slice);
    println!("partition_dedup: {:?}", slice.partition_dedup());
    println!("partition_dedup后: {:?}", slice);
    println!(
        "partition_dedup_by(|a, b| a == b): {:?}",
        // 只有相邻元素才被认为是重复
        slice.partition_dedup_by(|a, b| a.eq(&b))
    );

    println!(
        "partition_dedup_by_key(|&x| x % 2): {:?}",
        slice.partition_dedup_by_key(|i| *i % 2)
    );
}
/// 分割操作示例
fn splitting_operations() {
    println!("\n=== 分割操作示例 ===");

    let slice = &mut [1, 2, 0, 3, 4, 0, 5, 6, 9];
    println!("原始切片: {:?}", slice);
    // NOTE：包含分隔符
    // split_at - 在指定位置分割
    // at包含，作为右边部分
    // 位置不对，直接panic
    // 安全方法：split_at_checked
    let (left, right) = slice.split_at(4);
    println!("split_at(4): left={:?}, right={:?}", left, right);

    let len = slice.len() / 2;
    // split_at_mut_checked - 在指定位置分割（可变）
    // 位置不对，返回None
    // 下面方式直接panic
    let (left, right) = slice.split_at_mut(len);
    println!("split_at_mut(len/2): left={:?}, right={:?}", left, right);

    // split_once - 按条件分割一次
    // split_once_mut
    // rsplit_once
    // rsplit_once_mut
    println!("split_once(0): {:?}", slice.split_once(|&x| x == 0));

    // split - 按条件分割
    println!("按0分割:");

    for part in slice.split(|&x| x == 0) {
        // [1, 2] [3, 4] [5, 6]
        print!("{:?} ", part);
    }
    println!();
    println!("splitn(2)按0分割:");
    // limited to returning at most `n` items
    // n表示分割成几部分
    // splitn - 限制分割次数
    //
    // rsplit
    // rsplit_mut
    // rsplitn
    // rsplitn_mut
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

    // split_inclusive
    // split_inclusive - 按条件分割（包含分隔符）
    // 匹配元素会包含在前一个子切片末尾（inclusive split）
    // split_inclusive_mut
    println!("split_inclusive(|a, b| a <= b):");
    for chunk in slice.split_inclusive(|&x| x == 0) {
        println!("{:?} ", chunk);
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
    // 反转后: [1, 9, 8, 7, 6, 5, 4, 3, 2, 10]
    // rotate_left - 左旋转
    slice.rotate_left(3);
    slice.rotate_right(3);
    println!("左旋转3位后: {:?}", slice);
    // 左旋转3位后: [7, 6, 5, 4, 3, 2, 10, 1, 9, 8]

    // fill - 填充值
    let mut fill_data = [25; 5];
    // 使用clone值填充自己
    fill_data.fill(42);
    fill_data.fill_with(|| 2);
    println!("fill填充后: {:?}", fill_data);

    // copy_from_slice - 复制切片
    let mut target = [0; 5];
    target.copy_from_slice(&slice[..5]);
    println!("copy_from_slice结果: {:?}", target);

    // clone_from_slice - 克隆切片（需要Clone trait）
    let mut clone_target = [0; 5];
    clone_target.clone_from_slice(&slice[..5]);
    println!("clone_from_slice结果: {:?}", clone_target);

    // split_off - 分割切片（返回分割点后的部分）
    let mut slice: &[_] = &['a', 'b', 'c', 'd'];
    // OneSidedRange - 单端范围
    let tail = slice.split_off(1..);
    println!("split_off: {:?}", slice);
    // split_off_mut
    let mut slice_mut: &mut [_] = &mut ['a', 'b', 'c', 'd'];
    let tail = slice_mut.split_off_mut(1..);
    println!("split_off_mut: {:?}", slice_mut);
    // split_off_first - 分割第一个元素（返回分割点后的部分）
    // split_off_last - 分割最后一个元素（返回分割点后的部分）
    let first = slice.split_off_first();
    println!("split_off_first: {:?}", first);

    // split_off_first_mut
    let first_mut = slice_mut.split_off_first_mut();
    println!("split_off_first_mut: {:?}", first_mut);

    // get_disjoint_unchecked_mut
    // get_disjoint_unchecked_mut - 获取不相交的可变切片（未检查索引）
    let x = &mut [1, 2, 4];
    // 一次性返回 多个可变引用，不允许混合类型
    // [usize; N] → 返回 [&mut T; N]

    // [Range<usize>; N] → 返回 [&mut [T]; N]

    // [RangeInclusive<usize>; N] → 返回 [&mut [T]; N]
    unsafe {
        let [a, b] = x.get_disjoint_unchecked_mut([0, 2]);
        *a *= 10;
        *b *= 100;
    }
    assert_eq!(x, &[10, 2, 400]);

    unsafe {
        let [a, b] = x.get_disjoint_unchecked_mut([0..1, 2..3]);
        a[0] = 8;
        b[0] = 88;
        b[1] = 888;
    }
    assert_eq!(x, &[8, 88, 888]);

    unsafe {
        let [a, b] = x.get_disjoint_unchecked_mut([1..=1, 0..=0]);
        a[0] = 11;
        a[1] = 111;
        b[0] = 1;
    }
    assert_eq!(x, &[1, 11, 111]);

    // get_disjoint_mut - 检查索引是否不相交（未检查索引）

    let [a, b] = x.get_disjoint_mut([0..1, 2..3]).unwrap();
    a[0] = 8;
    b[0] = 88;
    b[1] = 888;

    assert_eq!(x, &[8, 88, 888]);

    // element_offset - 获取元素在切片中的偏移量（未检查索引）
    // #![feature(substr_range)]
    let nums: &[u32] = &[1, 7, 1, 1];

    let num = &nums[2];

    assert_eq!(num, &1);

    assert_eq!(nums.element_offset(num), Some(2));

    // substr_range
    // 只要有分隔符，就会产生分隔，导致空切片问题
    let nums = &[0, 5, 10, 0, 0, 5];
    let mut iter = nums
        .split(|t| *t == 0)
        .map(|n| nums.subslice_range(n).unwrap());
    assert_eq!(iter.next(), Some(0..0));
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
