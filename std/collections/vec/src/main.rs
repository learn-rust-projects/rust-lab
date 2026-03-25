use std::collections::VecDeque;

mod capacity;
mod split;
mod swap;

fn main() {
    // 创建空的 Vec
    let mut vec = Vec::new();
    // 使用 push 添加元素到末尾
    vec.push(1);
    vec.push(2);
    vec.push(3);
    println!("After push: {:?}", vec);
    // 使用 pop 删除并返回最后一个元素
    let last = vec.pop();
    println!("Popped: {:?}", last);
    println!("After pop: {:?}", vec);
    // 在指定索引位置插入元素（索引不能超过 vec 长度，否则 panic）
    vec.insert(0, 0);
    println!("After insert at index 0: {:?}", vec);
    // 删除指定索引位置的元素，并返回被删除的元素（索引越界会 panic）
    vec.remove(2);
    println!("After remove at index 2: {:?}", vec);

    // 获取首元素（返回 Option<&T>）
    if let Some(first) = vec.first() {
        println!("First element: {}", first);
    }

    // 获取尾元素（返回 Option<&T>）
    if let Some(last) = vec.last() {
        println!("Last element: {}", last);
    }

    // 反转 vec 中所有元素的顺序
    vec.reverse();
    println!("After reverse: {:?}", vec);

    // 升序排序
    vec.sort();
    println!("After sort: {:?}", vec);

    vec.push(10);
    vec.push(5);
    // 降序排序（使用自定义比较函数）
    vec.sort_by(|a, b| b.cmp(a));
    println!("After sort descending: {:?}", vec);

    // 使用 map 转换每个元素（类似 JavaScript 的 map）
    let doubled: Vec<i32> = vec.iter().map(|x| x * 2).collect();
    println!("Doubled: {:?}", doubled);

    // 使用 filter 过滤元素（类似 JavaScript 的 filter）
    let filtered: Vec<&i32> = vec.iter().filter(|x| **x > 5).collect();
    println!("Filtered (x > 5): {:?}", filtered);

    // 使用切片语法获取子集合
    let sliced = &vec[1..4];
    println!("Sliced [1..4]: {:?}", sliced);

    // 截断到指定长度，删除多余的元素
    vec.truncate(3);
    println!("After truncate(3): {:?}", vec);

    // 清空 vec，删除所有元素
    vec.clear();
    println!("After clear: {:?}", vec);
    // 检查 vec 是否为空
    println!("Is empty: {}", vec.is_empty());

    // 使用 vec! 宏快速创建 Vec
    let mut vec2 = vec![1, 2, 3, 4, 5];
    println!("Vec from macro: {:?}", vec2);

    // 获取 Vec 的容量（不小于长度）
    let capacity = vec2.capacity();
    println!("Capacity: {}", capacity);

    // 调整 Vec 长度，不足部分用指定值填充
    vec2.resize(8, 0);
    println!("After resize: {:?}", vec2);

    // 使用闭包调整 Vec 长度
    let mut vec3 = vec![1, 2, 3];
    vec3.resize_with(6, || 0);
    println!("After resize_with: {:?}", vec3);

    // 对数组/切片进行操作
    let vec4 = [1, 2, 3, 4, 5];
    // 计算所有元素的和
    let sum: i32 = vec4.iter().sum();
    println!("Sum: {}", sum);

    // 计算所有元素的积
    let product: i32 = vec4.iter().product();
    println!("Product: {}", product);

    let vec5 = [1, 2, 3, 4, 5];
    // 检查是否包含指定元素
    let contains_three = vec5.contains(&3);
    println!("Contains 3: {}", contains_three);

    // 查找第一个满足条件的元素位置
    let position = vec5.iter().position(|x| *x > 3);
    println!("First position where x > 3: {:?}", position);

    // 查找最后一个满足条件的元素位置
    let last_position = vec5.iter().rposition(|x| *x > 3);
    println!("Last position where x > 3: {:?}", last_position);

    // 删除相邻的重复元素（需要先排序）
    let mut vec6 = vec![3, 1, 4, 1, 5, 9, 2, 6];
    vec6.dedup();
    println!("After dedup: {:?}", vec6);

    // 使用自定义函数删除相邻重复元素（根据奇偶性）
    vec6.dedup_by(|a, b| {
        let a_val = *a;
        let b_val = *b;
        a_val % 2 == b_val % 2
    });
    println!("After dedup_by (same parity): {:?}", vec6);

    // 合并两个 Vec（消耗所有权）
    let vec7 = vec![1, 2, 3];
    let vec8 = vec![4, 5, 6];
    let merged: Vec<i32> = vec7.into_iter().chain(vec8).collect();
    println!("After chain: {:?}", merged);

    // 重复 Vec 元素 N 次
    let vec9 = [1, 2, 3];
    let repeated: Vec<i32> = vec9.repeat(3);
    println!("After repeat(3): {:?}", repeated);

    // 左旋转 N 个元素
    let mut vec10 = vec![1, 2, 3, 4, 5];
    vec10.rotate_left(2);
    println!("After rotate_left(2): {:?}", vec10);

    // 右旋转 N 个元素
    let mut vec11 = vec![1, 2, 3, 4, 5];
    vec11.rotate_right(2);
    println!("After rotate_right(2): {:?}", vec11);

    // 不稳定排序（可能改变相同元素的相对顺序，但更快）
    let mut vec12 = vec![5, 3, 1, 4, 2];
    vec12.sort_unstable();
    println!("After sort_unstable: {:?}", vec12);

    // 不稳定排序（自定义比较函数）
    vec12.sort_unstable_by(|a, b| b.cmp(a));
    println!("After sort_unstable descending: {:?}", vec12);

    // 交换两个位置的元素
    let mut vec13 = vec![1, 2, 3, 4, 5];
    vec13.swap(0, 4);
    println!("After swap(0, 4): {:?}", vec13);

    // 转换为只读切片
    let vec14 = vec![1, 2, 3, 4, 5];
    vec14.as_slice();
    println!("As slice: {:?}", vec14.as_slice());

    // 在指定索引处切分成两个 Vec
    let mut vec16 = vec![1, 2, 3, 4, 5];
    let split = vec16.split_off(3);
    println!("Original after split_off: {:?}", vec16);
    println!("Split off: {:?}", split);

    // 在指定索引处分割为两个引用
    let vec17 = [1, 2, 3, 4, 5];
    let (left, right) = vec17.split_at(2);
    println!("Left: {:?}", left);
    println!("Right: {:?}", right);

    // === 新增 Vec 其他操作示例 ===
    let mut v = vec![1, 2];
    assert!(!v.is_empty()); // 检查 v 是否为空

    let _v1 = [11, 22].to_vec(); // append 操作会导致 v1 清空数据，增加可变声明
    v.truncate(1); // 截断到指定长度，多余的元素被删除, v: [11]
    v.retain(|x| *x > 10); // 保留满足条件的元素，即删除不满足条件的元素

    let mut v = vec![11, 22, 33, 44, 55];
    // 删除指定范围的元素，同时获取被删除元素的迭代器, v: [11, 55], m: [22, 33, 44]
    let _m: Vec<_> = v.drain(1..=3).collect();

    // === 更多 Vec 操作示例 ===

    // 追加另一个 Vec 的所有元素（会清空被追加的 Vec）
    let mut vec15 = vec![1, 2, 3];
    vec15.append(&mut vec![4, 5, 6]);
    println!("After append: {:?}", vec15);

    // extend: 将另一个 Vec 的元素添加到当前 Vec,不清空
    let mut v1 = vec![1, 2, 3];
    let v2 = vec![4, 5, 6];
    v1.extend(&v2); // v1 = [1, 2, 3, 4, 5, 6]
    println!("After extend: {:?}", v1);
    println!("After extend v2: {:?}", v2);

    // extend_from_slice: 从切片添加元素到 Vec
    let slice = &[7, 8, 9];
    let mut v3 = vec![1, 2, 3];
    v3.extend_from_slice(slice); // v3 = [1, 2, 3, 7, 8, 9]
    println!("After extend_from_slice: {:?}", v3);

    // 使用 chain 合并两个集合（不消耗原 Vec）
    let v4 = [1, 2, 3];
    let v5 = [4, 5, 6];
    let merged: Vec<_> = v4.iter().chain(v5.iter()).collect(); // merged = [1, 2, 3, 4, 5, 6]
    println!("After chain: {:?}", merged);

    // 使用 get 安全访问元素（返回 Option，避免 panic）
    let v6 = [1, 2, 3];
    if let Some(value) = v6.get(1) {
        println!("Value at index 1: {}", value);
    }

    // 删除元素前先检查索引是否有效
    let mut v7 = vec![1, 2, 3];
    let index = 5;
    if v7.get(index).is_some() {
        v7.remove(index);
    } else {
        println!("Index {} is out of bounds", index);
    }

    // 转化成切片
    let v8 = vec![1, 2, 3];
    let slice: &[i32] = v8.as_slice();
    println!("As slice: {:?}", slice);

    // 转换为固定大小的数组（可能失败）
    let v9 = vec![1, 2, 3, 4];
    let arr: [i32; 4] = v9.try_into().expect("Failed to convert");
    println!("As array: {:?}", arr);
    // === 高级操作 ===
    let mut v1 = vec![1, 2, 3];
    let mut v2 = vec![4, 5, 6];
    // 交换两个 Vec
    std::mem::swap(&mut v1, &mut v2);
    // Vec 转 HashSet 去重
    use std::collections::HashSet;

    let v = vec![1, 2, 3, 3, 4, 4, 5];
    let set: HashSet<_> = v.into_iter().collect();
    let _unique_vec: Vec<_> = set.into_iter().collect();

    // === VecDeque 双端队列 ===
    // VecDeque 是双端队列，支持在两端高效添加/删除元素
    let mut vec18 = VecDeque::new();
    vec18.push_front(1);
    vec18.push_front(0); // 添加到队首
    vec18.push_back(2); // 添加到队尾
    println!("VecDeque: {:?}", vec18);

    // 从队首/队尾删除元素
    let front = vec18.pop_front();
    let back = vec18.pop_back();
    println!("Front: {:?}, Back: {:?}", front, back);
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_vec_push_pop() {
        let mut vec = Vec::new();
        vec.push(1);
        vec.push(2);
        assert_eq!(vec.len(), 2);
        assert_eq!(vec.pop(), Some(2));
        assert_eq!(vec.pop(), Some(1));
        assert_eq!(vec.pop(), None);
    }

    #[test]
    fn test_vec_insert_remove() {
        let mut vec = vec![1, 2, 3];
        vec.insert(1, 5);
        assert_eq!(vec, vec![1, 5, 2, 3]);
        vec.remove(2);
        assert_eq!(vec, vec![1, 5, 3]);
    }

    #[test]
    fn test_vec_first_last() {
        let vec = [1, 2, 3];
        assert_eq!(vec.first(), Some(&1));
        assert_eq!(vec.last(), Some(&3));
    }

    #[test]
    fn test_vec_reverse() {
        let mut vec = vec![1, 2, 3];
        vec.reverse();
        assert_eq!(vec, vec![3, 2, 1]);
    }

    #[test]
    fn test_vec_sort() {
        let mut vec = vec![3, 1, 2];
        vec.sort();
        assert_eq!(vec, vec![1, 2, 3]);
    }

    #[test]
    fn test_vec_sort_by() {
        let mut vec = vec![1, 2, 3];
        vec.sort_by(|a, b| b.cmp(a));
        assert_eq!(vec, vec![3, 2, 1]);
    }

    #[test]
    fn test_vec_map() {
        let vec = vec![1, 2, 3];
        let doubled: Vec<i32> = vec.iter().map(|x| x * 2).collect();
        assert_eq!(doubled, vec![2, 4, 6]);
    }

    #[test]
    fn test_vec_filter() {
        let vec = vec![1, 2, 3, 4, 5];
        let filtered: Vec<&i32> = vec.iter().filter(|x| **x > 3).collect();
        assert_eq!(filtered, vec![&4, &5]);
    }

    #[test]
    fn test_vec_slice() {
        let vec = vec![1, 2, 3, 4, 5];
        assert_eq!(&vec[1..4], &[2, 3, 4]);
    }

    #[test]
    fn test_vec_truncate() {
        let mut vec = vec![1, 2, 3, 4, 5];
        vec.truncate(3);
        assert_eq!(vec, vec![1, 2, 3]);
    }

    #[test]
    fn test_vec_clear() {
        let mut vec = vec![1, 2, 3];
        vec.clear();
        assert!(vec.is_empty());
    }

    #[test]
    fn test_vec_from_macro() {
        let vec = vec![1, 2, 3];
        assert_eq!(vec, Vec::from([1, 2, 3]));
    }

    #[test]
    fn test_vec_resize() {
        let mut vec = vec![1, 2, 3];
        vec.resize(5, 0);
        assert_eq!(vec, vec![1, 2, 3, 0, 0]);
    }

    #[test]
    fn test_vec_contains() {
        let vec = vec![1, 2, 3];
        assert!(vec.contains(&2));
        assert!(!vec.contains(&5));
    }

    #[test]
    fn test_vec_position() {
        let vec = vec![1, 2, 3, 4, 5];
        assert_eq!(vec.iter().position(|x| *x > 3), Some(3));
        assert_eq!(vec.iter().position(|x| *x > 10), None);
    }

    #[test]
    fn test_vec_dedup() {
        let mut vec = vec![1, 1, 2, 2, 3];
        vec.dedup();
        assert_eq!(vec, vec![1, 2, 3]);
    }

    #[test]
    fn test_vec_chain() {
        let vec1 = vec![1, 2, 3];
        let vec2 = vec![4, 5, 6];
        let merged: Vec<i32> = vec1.into_iter().chain(vec2.into_iter()).collect();
        assert_eq!(merged, vec![1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn test_vec_swap() {
        let mut vec = vec![1, 2, 3, 4];
        vec.swap(0, 3);
        assert_eq!(vec, vec![4, 2, 3, 1]);
    }

    #[test]
    fn test_vec_append() {
        let mut vec1 = vec![1, 2, 3];
        let mut vec2 = vec![4, 5, 6];
        vec1.append(&mut vec2);
        assert_eq!(vec1, vec![1, 2, 3, 4, 5, 6]);
        assert!(vec2.is_empty());
    }

    #[test]
    fn test_vec_split_off() {
        let mut vec = vec![1, 2, 3, 4, 5];
        let split = vec.split_off(3);
        assert_eq!(vec, vec![1, 2, 3]);
        assert_eq!(split, vec![4, 5]);
    }

    #[test]
    fn test_vec_split_at() {
        let vec = vec![1, 2, 3, 4, 5];
        let (left, right) = vec.split_at(2);
        assert_eq!(left, &[1, 2]);
        assert_eq!(right, &[3, 4, 5]);
    }

    #[test]
    fn test_vec_retain() {
        let mut vec = vec![1, 2, 3, 4, 5];
        vec.retain(|x| x % 2 == 0);
        assert_eq!(vec, vec![2, 4]);
    }

    #[test]
    fn test_vec_drain() {
        let mut vec = vec![1, 2, 3, 4, 5];
        let drained: Vec<i32> = vec.drain(1..4).collect();
        assert_eq!(drained, vec![2, 3, 4]);
        assert_eq!(vec, vec![1, 5]);
    }

    #[test]
    fn test_vec_clone() {
        let vec1 = vec![1, 2, 3];
        let vec2 = vec1.clone();
        assert_eq!(vec1, vec2);
    }

    #[test]
    fn test_vec_from_iter() {
        let vec: Vec<i32> = (1..=5).collect();
        assert_eq!(vec, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_vec_iter_sum() {
        let vec = vec![1, 2, 3, 4, 5];
        assert_eq!(vec.iter().sum::<i32>(), 15);
    }

    #[test]
    fn test_vec_binary_search() {
        let vec = vec![1, 2, 3, 4, 5];
        assert_eq!(vec.binary_search(&3), Ok(2));
        assert_eq!(vec.binary_search(&6), Err(5));
    }

    #[test]
    fn test_vec_binary_search_by() {
        let vec = vec![1, 2, 3, 4, 5];
        assert_eq!(vec.binary_search_by(|x| x.cmp(&3)), Ok(2));
    }

    #[test]
    fn test_vec_get() {
        let vec = vec![1, 2, 3];
        assert_eq!(vec.get(1), Some(&2));
        assert_eq!(vec.get(5), None);
        assert_eq!(vec.get(1..3), Some(&[2, 3][..]));
    }

    #[test]
    fn test_vec_get_mut() {
        let mut vec = vec![1, 2, 3];
        if let Some(elem) = vec.get_mut(1) {
            *elem *= 10;
        }
        assert_eq!(vec, vec![1, 20, 3]);
    }

    #[test]
    fn test_vec_to_vec() {
        let vec = vec![1, 2, 3];
        let new_vec = vec.to_vec();
        assert_eq!(vec, new_vec);
    }

    #[test]
    fn test_vec_as_slice() {
        let vec = vec![1, 2, 3];
        let slice: &[i32] = vec.as_slice();
        assert_eq!(slice, &[1, 2, 3]);
    }
}
