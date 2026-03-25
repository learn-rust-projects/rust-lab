use std::collections::HashMap;

fn main() {
    // 创建一个新的HashMap
    let mut map = HashMap::new();

    // 插入键值对
    map.insert(String::from("apple"), 10);
    map.insert(String::from("banana"), 20);
    map.insert(String::from("cherry"), 30);
    println!("Initial HashMap: {:?}", map);

    // 获取值
    if let Some(count) = map.get("apple") {
        println!("Number of apples: {}", count);
    }

    // 检查键是否存在
    println!("Contains 'banana': {}", map.contains_key("banana"));
    println!("Contains 'date': {}", map.contains_key("date"));

    // 获取可变引用并修改值
    if let Some(count) = map.get_mut("banana") {
        *count += 5;
    }
    println!("After updating banana: {:?}", map);

    // 移除键值对
    map.remove("cherry");
    println!("After removing cherry: {:?}", map);

    // 获取长度和检查是否为空
    println!("Length of HashMap: {}", map.len());
    println!("Is HashMap empty? {}", map.is_empty());

    // 迭代键值对
    println!("Iterating over key-value pairs:");
    for (key, value) in &map {
        println!("{}: {}", key, value);
    }

    // 获取所有键
    println!("Keys: {:?}", map.keys().collect::<Vec<_>>());

    // 获取所有值
    println!("Values: {:?}", map.values().collect::<Vec<_>>());

    // 使用Entry API插入或更新
    map.entry(String::from("date")).or_insert(40);
    map.entry(String::from("apple"))
        .and_modify(|count| *count += 10);
    println!("After using Entry API: {:?}", map);

    // 扩展HashMap
    let mut additional_items = HashMap::new();
    additional_items.insert(String::from("elderberry"), 50);
    additional_items.insert(String::from("fig"), 60);
    map.extend(additional_items);
    println!("After extending: {:?}", map);

    // 保留特定条件的键值对（保留值大于40的）
    map.retain(|_, value| *value > 40);
    println!("After retaining values > 40: {:?}", map);

    // drain clear +move
    // 清空HashMap
    for (key, value) in map.drain() {
        println!("Drained value: {} for key: {}", value, key);
    }
    println!("After draining: {:?}", map);
    println!("Is HashMap empty? {}", map.is_empty());
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    #[test]
    fn test_hashmap_basic_operations() {
        let mut map = HashMap::new();

        // 测试插入
        map.insert("one", 1);
        map.insert("two", 2);
        assert_eq!(map.len(), 2);

        // 测试获取
        assert_eq!(map.get("one"), Some(&1));
        assert_eq!(map.get("two"), Some(&2));
        assert_eq!(map.get("three"), None);

        // 测试包含键
        assert!(map.contains_key("one"));
        assert!(!map.contains_key("three"));

        // 测试移除
        map.remove("one");
        assert_eq!(map.len(), 1);
        assert!(!map.contains_key("one"));

        // 测试清空
        map.clear();
        assert_eq!(map.len(), 0);
        assert!(map.is_empty());
    }

    #[test]
    fn test_hashmap_mutability() {
        let mut map = HashMap::new();
        map.insert("counter", 0);

        // 测试可变获取和修改
        if let Some(count) = map.get_mut("counter") {
            *count += 1;
        }
        assert_eq!(map.get("counter"), Some(&1));
    }

    #[test]
    fn test_hashmap_entry_api() {
        let mut map = HashMap::new();

        // 测试or_insert
        map.entry("apple").or_insert(10);
        assert_eq!(map.get("apple"), Some(&10));

        // 测试or_insert不覆盖现有值
        map.entry("apple").or_insert(20);
        assert_eq!(map.get("apple"), Some(&10));

        // 测试and_modify
        map.entry("apple").and_modify(|count| *count += 5);
        assert_eq!(map.get("apple"), Some(&15));

        // 测试or_insert_with
        map.entry("banana").or_insert_with(|| 20);
        assert_eq!(map.get("banana"), Some(&20));
    }

    #[test]
    fn test_hashmap_iteration() {
        let mut map = HashMap::new();
        map.insert("a", 1);
        map.insert("b", 2);
        map.insert("c", 3);

        // 测试键迭代
        let mut keys: Vec<_> = map.keys().collect();
        keys.sort();
        assert_eq!(keys, vec![&"a", &"b", &"c"]);

        // 测试值迭代
        let mut values: Vec<_> = map.values().collect();
        values.sort();
        assert_eq!(values, vec![&1, &2, &3]);

        // 测试键值对迭代
        let mut entries: Vec<_> = map.iter().collect();
        entries.sort_by_key(|&(k, _)| k);
        assert_eq!(entries, vec![(&"a", &1), (&"b", &2), (&"c", &3)]);
    }

    #[test]
    fn test_hashmap_extend_and_retain() {
        let mut map = HashMap::new();
        map.insert("a", 1);
        map.insert("b", 2);

        // 测试extend
        let other_map: HashMap<_, _> = [("c", 3), ("d", 4)].iter().cloned().collect();
        map.extend(other_map);
        assert_eq!(map.len(), 4);
        assert_eq!(map.get("c"), Some(&3));

        // 测试retain
        map.retain(|_, value| *value > 2);
        assert_eq!(map.len(), 2);
        assert!(!map.contains_key("a"));
        assert!(!map.contains_key("b"));
        assert!(map.contains_key("c"));
        assert!(map.contains_key("d"));
    }

    #[test]
    fn test_hashmap_drain() {
        let mut map = HashMap::new();
        map.insert("a", 1);
        map.insert("b", 2);
        map.insert("c", 3);

        // 测试drain
        let drained: Vec<_> = map.drain().collect();
        assert_eq!(drained.len(), 3);
        assert!(map.is_empty());
    }

    #[test]
    fn test_hashmap_from_iter() {
        // 从迭代器创建HashMap
        let items = vec![("a", 1), ("b", 2), ("c", 3)];
        let map: HashMap<_, _> = items.into_iter().collect();

        assert_eq!(map.len(), 3);
        assert_eq!(map.get("a"), Some(&1));
        assert_eq!(map.get("b"), Some(&2));
        assert_eq!(map.get("c"), Some(&3));
    }

    #[test]
    fn test_hashmap_clone() {
        let mut map = HashMap::new();
        map.insert("a", 1);
        map.insert("b", 2);

        // 测试克隆
        let cloned_map = map.clone();
        assert_eq!(map, cloned_map);
        assert_eq!(cloned_map.len(), 2);
        assert_eq!(cloned_map.get("a"), Some(&1));
    }

    #[test]
    fn test_and_or_insert_order() {
        let mut map = HashMap::new();
        map.insert("a", 1);
        map.insert("b", 2);

        // 有的话加1，没有插入0
        map.entry("count").and_modify(|v| *v += 1).or_insert(1);
        assert_eq!(map["count"], 1);
        // 没有插入0，返回V
        let v = map.entry("count2").or_insert(0);
        *v += 1;
        assert_eq!(map["count2"], 1);
    }
}
