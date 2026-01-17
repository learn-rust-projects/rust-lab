//! 标准库Vec方法测试程序
//!
//! 这个程序展示了标准库中Vec的各种方法，包括：
//! - 基本操作：push, pop, insert, remove
//! - 容量管理：reserve, shrink_to_fit, truncate
//! - 迭代器：iter, iter_mut, into_iter
//! - 切片操作：split_at, windows, chunks
//! - 搜索和排序：contains, sort, binary_search

fn main() {
    println!("=== 标准库Vec方法测试程序 ===");

    // 运行所有测试示例
    basic_operations();
    capacity_management();
    iterator_methods();
    slice_operations();
    search_and_sort();
    advanced_methods();

    println!("\n🎉 所有Vec方法测试完成！");
}

/// 基本操作示例
fn basic_operations() {
    println!("\n=== 基本操作示例 ===");

    let mut vec = Vec::new();

    // push - 添加元素
    vec.push(1);
    vec.push(2);
    vec.push(3);
    println!("push后: {:?}", vec);

    // pop - 移除最后一个元素
    let popped = vec.pop();
    println!("pop结果: {:?}, 剩余: {:?}", popped, vec);

    // insert - 在指定位置插入元素
    vec.insert(1, 10);
    println!("insert后: {:?}", vec);

    // remove - 移除指定位置元素
    let removed = vec.remove(0);
    println!("remove结果: {}, 剩余: {:?}", removed, vec);

    // clear - 清空向量
    vec.clear();
    println!("clear后: {:?}", vec);
}

/// 容量管理示例
fn capacity_management() {
    println!("\n=== 容量管理示例 ===");

    let mut vec = vec![1, 2, 3, 4, 5];
    println!("初始向量: {:?}", vec);
    println!("长度: {}, 容量: {}", vec.len(), vec.capacity());

    // truncate - 截断到指定长度
    vec.truncate(3);
    println!("truncate(3)后: {:?}", vec);
    println!("长度: {}, 容量: {}", vec.len(), vec.capacity());
    // reserve - 预留容量
    vec.reserve(10);
    println!(
        "reserve(10)后 - 长度: {}, 容量: {}",
        vec.len(),
        vec.capacity()
    );

    // shrink_to_fit - 收缩到合适大小
    vec.shrink_to_fit();
    println!(
        "shrink_to_fit后 - 长度: {}, 容量: {}",
        vec.len(),
        vec.capacity()
    );

    // resize - 调整大小
    vec.resize(5, 0);
    println!("resize(5, 0)后: {:?}", vec);

    // resize_with - 使用闭包调整大小
    vec.resize_with(7, || 100);
    println!("resize_with(7, || 100)后: {:?}", vec);
}

/// 迭代器方法示例
fn iterator_methods() {
    println!("\n=== 迭代器方法示例 ===");

    let mut vec = vec![1, 2, 3, 4, 5];

    // iter - 不可变迭代器
    println!("iter遍历:");
    for item in vec.iter() {
        print!("{} ", item);
    }
    println!();

    // iter_mut - 可变迭代器
    println!("iter_mut修改:");
    for item in vec.iter_mut() {
        *item *= 2;
    }
    println!("修改后: {:?}", vec);

    // into_iter - 消费迭代器
    let sum: i32 = vec.into_iter().sum();
    println!("into_iter求和: {}", sum);
}

/// 切片操作示例
fn slice_operations() {
    println!("\n=== 切片操作示例 ===");

    let vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    println!("原始向量: {:?}", vec);

    // split_at - 分割切片
    let (left, right) = vec.split_at(5);
    println!("split_at(5): left={:?}, right={:?}", left, right);

    // windows - 滑动窗口
    println!("windows(3):");
    for window in vec.windows(3) {
        print!("{:?} ", window);
    }
    println!();

    // chunks - 分块
    println!("chunks(4):");
    for chunk in vec.chunks(4) {
        print!("{:?} ", chunk);
    }
    println!();

    // get - 安全获取元素
    match vec.get(15) {
        Some(val) => println!("索引15的值: {}", val),
        None => println!("索引15超出范围"),
    }
}

/// 搜索和排序示例
fn search_and_sort() {
    println!("\n=== 搜索和排序示例 ===");

    let mut vec = vec![5, 2, 8, 1, 9, 3, 7, 4, 6, 10];
    println!("原始向量: {:?}", vec);

    // contains - 检查是否包含元素
    println!("包含8: {}", vec.contains(&8));
    println!("包含11: {}", vec.contains(&11));

    // sort - 排序
    vec.sort();
    println!("排序后: {:?}", vec);

    // binary_search - 二分查找
    match vec.binary_search(&7) {
        Ok(index) => println!("找到7在索引: {}", index),
        Err(_) => println!("未找到7"),
    }

    // retain - 保留满足条件的元素
    vec.retain(|&x| x % 2 == 0);
    println!("保留偶数: {:?}", vec);
}

/// 高级方法示例
fn advanced_methods() {
    println!("\n=== 高级方法示例 ===");

    let mut vec1 = vec![1, 2, 3];
    let mut vec2 = vec![4, 5, 6];

    println!("vec1: {:?}", vec1);
    println!("vec2: {:?}", vec2);

    // append - 移动所有元素到另一个向量
    vec1.append(&mut vec2);
    println!("append后 - vec1: {:?}, vec2: {:?}", vec1, vec2);

    // extend - 扩展向量
    vec1.extend([7, 8, 9].iter());
    println!("extend后: {:?}", vec1);

    // drain - 移除范围并返回迭代器
    let drained: Vec<i32> = vec1.drain(2..5).collect();
    println!(
        "drain(2..5)后 - 移除的元素: {:?}, 剩余: {:?}",
        drained, vec1
    );

    // splice - 替换范围
    let replacement = vec![10, 11, 12];
    let spliced: Vec<i32> = vec1.splice(1..3, replacement).collect();
    println!("splice后 - 替换的元素: {:?}, 结果: {:?}", spliced, vec1);

    // dedup - 去重
    let mut vec_with_duplicates = vec![1, 2, 2, 3, 3, 3, 4, 4, 4, 4];
    vec_with_duplicates.dedup();
    println!("dedup去重后: {:?}", vec_with_duplicates);
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_truncate() {
        let mut vec = vec![1, 2, 3, 4, 5];
        vec.truncate(3);
        assert_eq!(vec, [1, 2, 3]);
        assert_eq!(vec.len(), 3);
    }

    #[test]
    fn test_reserve_and_shrink() {
        let mut vec = Vec::with_capacity(10);
        vec.push(1);
        vec.push(2);

        assert!(vec.capacity() >= 10);
        vec.shrink_to_fit();
        assert_eq!(vec.capacity(), 2);
    }

    #[test]
    fn test_push_pop() {
        let mut vec = Vec::new();
        vec.push(1);
        vec.push(2);

        assert_eq!(vec.pop(), Some(2));
        assert_eq!(vec.pop(), Some(1));
        assert_eq!(vec.pop(), None);
    }

    #[test]
    fn test_insert_remove() {
        let mut vec = vec![1, 3];
        vec.insert(1, 2);
        assert_eq!(vec, [1, 2, 3]);

        let removed = vec.remove(1);
        assert_eq!(removed, 2);
        assert_eq!(vec, [1, 3]);
    }

    #[test]
    fn test_iterators() {
        let mut vec = vec![1, 2, 3];

        // 测试iter_mut
        for item in vec.iter_mut() {
            *item *= 2;
        }
        assert_eq!(vec, [2, 4, 6]);

        // 测试into_iter
        let sum: i32 = vec.into_iter().sum();
        assert_eq!(sum, 12);
    }

    #[test]
    fn test_search_and_sort() {
        let mut vec = vec![3, 1, 4, 1, 5, 9, 2, 6];
        vec.sort();

        assert_eq!(vec, [1, 1, 2, 3, 4, 5, 6, 9]);
        assert_eq!(vec.binary_search(&5), Ok(5));
        assert!(vec.contains(&3));
    }

    #[test]
    fn test_slice_operations() {
        let vec = [1, 2, 3, 4, 5, 6];

        let (left, right) = vec.split_at(3);
        assert_eq!(left, [1, 2, 3]);
        assert_eq!(right, [4, 5, 6]);

        assert_eq!(vec.get(2), Some(&3));
        assert_eq!(vec.get(10), None);
    }

    #[test]
    fn test_advanced_methods() {
        let mut vec1 = vec![1, 2, 3];
        let mut vec2 = vec![4, 5, 6];

        vec1.append(&mut vec2);
        assert_eq!(vec1, [1, 2, 3, 4, 5, 6]);
        assert!(vec2.is_empty());

        let drained: Vec<i32> = vec1.drain(2..4).collect();
        assert_eq!(drained, [3, 4]);
        assert_eq!(vec1, [1, 2, 5, 6]);
    }
}
