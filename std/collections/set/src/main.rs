use std::collections::HashSet;

fn main() {
    // 创建一个新的HashSet
    let mut set = HashSet::new();

    // 插入元素
    set.insert(10);
    set.insert(20);
    set.insert(30);
    println!("Initial HashSet: {:?}", set);

    // get
    // 获取元素
    println!("Get 20: {:?}", set.get(&20));
    println!("Get 40: {:?}", set.get(&40));

    // take
    // 从集合中移出元素
    println!("Take 20: {:?}", set.take(&20));
    println!("After taking 20: {:?}", set);

    // 检查元素是否存在
    println!("Contains 20: {}", set.contains(&20));
    println!("Contains 40: {}", set.contains(&40));

    // 获取HashSet的大小
    println!("Size of HashSet: {}", set.len());

    // 检查是否为空
    println!("Is empty? {}", set.is_empty());

    // 移除元素
    println!("Removed 20: {:?}", set.remove(&20));
    println!("After removing 20: {:?}", set);

    // 迭代元素
    println!("Iterating over elements:");
    for &element in &set {
        println!("Element: {}", element);
    }

    // 创建两个集合进行集合运算
    let set1: HashSet<_> = [1, 2, 3, 4, 5].iter().cloned().collect();
    let set2: HashSet<_> = [3, 4, 5, 6, 7].iter().cloned().collect();
    println!("\nSet1: {:?}", set1);
    println!("Set2: {:?}", set2);

    // Union (并集)
    let union: HashSet<_> = set1.union(&set2).cloned().collect();
    println!("Union: {:?}", union);

    // Intersection (交集)
    let intersection: HashSet<_> = set1.intersection(&set2).cloned().collect();
    println!("Intersection: {:?}", intersection);

    // Difference (差集)
    let difference: HashSet<_> = set1.difference(&set2).cloned().collect();
    println!("Difference (set1 - set2): {:?}", difference);

    // Symmetric Difference (对称差集)
    let symmetric_difference: HashSet<_> = set1.symmetric_difference(&set2).cloned().collect();
    println!("Symmetric Difference: {:?}", symmetric_difference);

    // Subset (子集) 和 Superset (超集)
    let set3: HashSet<_> = [3, 4].iter().cloned().collect();
    println!("\nSet3: {:?}", set3);
    println!("Is set3 subset of set1? {}", set3.is_subset(&set1));
    println!("Is set1 superset of set3? {}", set1.is_superset(&set3));

    // is_disjoint
    // 检查两个集合是否不相交
    println!("Are set1 and set2 disjoint? {}", set1.is_disjoint(&set2));
    println!(
        "Are set1 and empty set disjoint? {}",
        set1.is_disjoint(&HashSet::new())
    );
    // 创建HashSet的其他方法
    let set_from_iter: HashSet<_> = (1..=5).collect();
    println!("\nHashSet from iterator: {:?}", set_from_iter);

    // 克隆HashSet
    let cloned_set = set_from_iter.clone();
    println!("Cloned HashSet: {:?}", cloned_set);

    // 替换HashSet中的元素
    let mut replace_set = HashSet::new();
    replace_set.insert(10);
    replace_set.insert(20);
    println!("\nOriginal replace_set: {:?}", replace_set);

    // insert返回false如果元素已存在
    let inserted = replace_set.insert(10);
    println!("Insert 10 again: {}", inserted);
    println!("After inserting 10 again: {:?}", replace_set);

    // 清空HashSet
    replace_set.clear();
    println!("After clearing: {:?}", replace_set);
    println!("Is empty after clear? {}", replace_set.is_empty());

    // 演示retain方法
    let mut retain_set: HashSet<_> = [1, 2, 3, 4, 5, 6].iter().cloned().collect();
    println!("\nOriginal retain_set: {:?}", retain_set);
    retain_set.retain(|&x| x % 2 == 0);
    println!("After retaining even numbers: {:?}", retain_set);

    // 演示drain方法 clear+move出元素
    let mut drain_set: HashSet<_> = [10, 20, 30].iter().cloned().collect();
    println!("\nOriginal drain_set: {:?}", drain_set);
    let drained: Vec<_> = drain_set.drain().collect();
    println!("Drained elements: {:?}", drained);
    println!("drain_set after drain: {:?}", drain_set);
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    #[test]
    fn test_hashset_basic_operations() {
        let mut set = HashSet::new();

        // 测试插入
        assert_eq!(set.insert(10), true);
        assert_eq!(set.insert(20), true);
        assert_eq!(set.len(), 2);

        // 测试重复插入
        assert_eq!(set.insert(10), false);
        assert_eq!(set.len(), 2);

        // 测试包含
        assert!(set.contains(&10));
        assert!(!set.contains(&30));

        // 测试移除
        assert_eq!(set.remove(&10), true);
        assert!(!set.contains(&10));
        assert_eq!(set.len(), 1);

        // 测试移除不存在的元素
        assert_eq!(set.remove(&30), false);

        // 测试清空
        set.clear();
        assert!(set.is_empty());
        assert_eq!(set.len(), 0);
    }

    #[test]
    fn test_hashset_set_operations() {
        let set1: HashSet<_> = [1, 2, 3, 4, 5].iter().cloned().collect();
        let set2: HashSet<_> = [3, 4, 5, 6, 7].iter().cloned().collect();

        // 测试并集
        let union: HashSet<_> = set1.union(&set2).cloned().collect();
        assert_eq!(union.len(), 7);
        assert!(union.contains(&1));
        assert!(union.contains(&2));
        assert!(union.contains(&3));
        assert!(union.contains(&4));
        assert!(union.contains(&5));
        assert!(union.contains(&6));
        assert!(union.contains(&7));

        // 测试交集
        let intersection: HashSet<_> = set1.intersection(&set2).cloned().collect();
        assert_eq!(intersection.len(), 3);
        assert!(intersection.contains(&3));
        assert!(intersection.contains(&4));
        assert!(intersection.contains(&5));

        // 测试差集
        let difference: HashSet<_> = set1.difference(&set2).cloned().collect();
        assert_eq!(difference.len(), 2);
        assert!(difference.contains(&1));
        assert!(difference.contains(&2));
        assert!(!difference.contains(&3));

        // 测试对称差集
        let symmetric_difference: HashSet<_> = set1.symmetric_difference(&set2).cloned().collect();
        assert_eq!(symmetric_difference.len(), 4);
        assert!(symmetric_difference.contains(&1));
        assert!(symmetric_difference.contains(&2));
        assert!(symmetric_difference.contains(&6));
        assert!(symmetric_difference.contains(&7));
    }

    #[test]
    fn test_hashset_subset_superset() {
        let set1: HashSet<_> = [1, 2, 3, 4, 5].iter().cloned().collect();
        let set2: HashSet<_> = [3, 4, 5].iter().cloned().collect();
        let set3: HashSet<_> = [5, 6, 7].iter().cloned().collect();

        // 测试子集
        assert!(set2.is_subset(&set1));
        assert!(!set3.is_subset(&set1));

        // 测试超集
        assert!(set1.is_superset(&set2));
        assert!(!set1.is_superset(&set3));

        // 测试相等性
        let set4: HashSet<_> = [1, 2, 3, 4, 5].iter().cloned().collect();
        assert!(set1.is_subset(&set4));
        assert!(set1.is_superset(&set4));
    }

    #[test]
    fn test_hashset_iteration() {
        let set: HashSet<_> = [10, 20, 30, 40, 50].iter().cloned().collect();
        let mut elements: Vec<_> = set.iter().cloned().collect();
        elements.sort();

        assert_eq!(elements, vec![10, 20, 30, 40, 50]);
    }

    #[test]
    fn test_hashset_from_iterator() {
        // 从迭代器创建HashSet
        let set: HashSet<_> = (1..=5).collect();

        assert_eq!(set.len(), 5);
        for i in 1..=5 {
            assert!(set.contains(&i));
        }

        // 从Vec创建HashSet
        let vec = vec![10, 20, 30, 20, 10];
        let set_from_vec: HashSet<_> = vec.into_iter().collect();

        assert_eq!(set_from_vec.len(), 3);
        assert!(set_from_vec.contains(&10));
        assert!(set_from_vec.contains(&20));
        assert!(set_from_vec.contains(&30));
    }

    #[test]
    fn test_hashset_retain() {
        let mut set: HashSet<_> = [1, 2, 3, 4, 5, 6, 7, 8].iter().cloned().collect();

        // 保留偶数
        set.retain(|&x| x % 2 == 0);

        assert_eq!(set.len(), 4);
        assert!(set.contains(&2));
        assert!(set.contains(&4));
        assert!(set.contains(&6));
        assert!(set.contains(&8));
        assert!(!set.contains(&1));
        assert!(!set.contains(&3));
    }

    #[test]
    fn test_hashset_drain() {
        let mut set: HashSet<_> = [1, 2, 3, 4, 5].iter().cloned().collect();

        // 清空集合并获取所有元素
        let drained: Vec<_> = set.drain().collect();

        assert_eq!(drained.len(), 5);
        assert!(set.is_empty());

        // 验证所有元素都被正确移除
        for i in 1..=5 {
            assert!(!set.contains(&i));
        }
    }

    #[test]
    fn test_hashset_clone() {
        let set1: HashSet<_> = [10, 20, 30].iter().cloned().collect();
        let set2 = set1.clone();

        // 验证两个集合相等
        assert_eq!(set1, set2);
        assert!(set1.is_subset(&set2));
        assert!(set2.is_subset(&set1));

        // 修改其中一个集合，另一个不受影响
        let mut set3 = set1.clone();
        set3.insert(40);

        assert_ne!(set1, set3);
        assert!(!set1.contains(&40));
        assert!(set3.contains(&40));
    }

    #[test]
    fn test_hashset_special_methods() {
        let set1: HashSet<_> = [1, 2, 3].iter().cloned().collect();
        let set2: HashSet<_> = [3, 4, 5].iter().cloned().collect();

        // 测试union的迭代器特性
        let union_count = set1.union(&set2).count();
        assert_eq!(union_count, 5);

        // 测试intersection的迭代器特性
        let intersection_count = set1.intersection(&set2).count();
        assert_eq!(intersection_count, 1);

        // 测试difference的迭代器特性
        let difference_count = set1.difference(&set2).count();
        assert_eq!(difference_count, 2);

        // 测试symmetric_difference的迭代器特性
        let symmetric_difference_count = set1.symmetric_difference(&set2).count();
        assert_eq!(symmetric_difference_count, 4);

        // 测试元素类型为字符串的HashSet
        let mut string_set = HashSet::new();
        string_set.insert(String::from("apple"));
        string_set.insert(String::from("banana"));
        string_set.insert(String::from("cherry"));

        assert!(string_set.contains("apple"));
        assert_eq!(string_set.len(), 3);

        // 测试take方法
        let taken = string_set.take("banana");
        assert_eq!(taken, Some(String::from("banana")));
        assert!(!string_set.contains("banana"));

        // 测试take不存在的元素
        let taken_none = string_set.take("date");
        assert_eq!(taken_none, None);
    }
}
