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

    // 清空HashMap
    map.clear();
    println!("After clearing: {:?}", map);
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
    fn test_hashmap_entry_all_methods() {
        let mut map = HashMap::new();

        // 测试or_insert
        let apple_count = map.entry("apple").or_insert(10);
        assert_eq!(*apple_count, 10);
        *apple_count += 5; // 可以直接修改引用的值
        assert_eq!(map.get("apple"), Some(&15));

        // 测试or_insert_with
        let banana_count = map.entry("banana").or_insert_with(|| 20);
        assert_eq!(*banana_count, 20);

        // 测试or_insert_with_key
        let cherry_count = map.entry("cherry").or_insert_with_key(|key| {
            // assert_eq!(key, "cherry");
            key.len() as i32 * 10
        });
        assert_eq!(*cherry_count, 60); // "cherry"长度为6，6*10=60

        // 测试and_modify - 键存在的情况
        let date_count = map
            .entry("apple")
            .and_modify(|count| *count += 10)
            .or_insert(30);
        assert_eq!(*date_count, 25); // 15 + 10 = 25

        // 测试and_modify - 键不存在的情况
        let elderberry_count = map
            .entry("elderberry")
            .and_modify(|count| *count += 10)
            .or_insert(40);
        assert_eq!(*elderberry_count, 40);

        // 测试or_default
        // 需要为值类型实现Default trait
        let mut string_map = HashMap::new();
        let default_value = string_map.entry("default_key").or_default();
        assert_eq!(*default_value, "");

        // 测试or_default与已有值
        string_map.insert("existing_key", "hello");
        let existing_value = string_map.entry("existing_key").or_default();
        assert_eq!(*existing_value, "hello");

        // 测试insert_entry
        let mut insert_map = HashMap::new();
        insert_map.insert("key1", "value1");

        // 插入新键
        let previous_value = insert_map.entry("key2").insert_entry("value2");
        // assert_eq!(previous_value, None);

        // 插入已存在的键
        let previous_value = insert_map.entry("key1").insert_entry("new_value1");
        // assert_eq!(previous_value, Some("value1"));
        assert_eq!(insert_map.get("key1"), Some(&"new_value1"));

        // 测试链式调用
        map.entry("fig")
            .and_modify(|count| *count += 5)
            .or_insert_with(|| 30);
        assert_eq!(map.get("fig"), Some(&30));

        // 测试多次链式调用
        map.entry("fig")
            .and_modify(|count| *count += 10)
            .and_modify(|count| *count *= 2);
        assert_eq!(map.get("fig"), Some(&80)); // 30 + 10 = 40, 40 * 2 = 80

        // 测试Entry的into_mut方法
        let mut grape_entry = map.entry("grape");
        let grape_count = grape_entry.or_insert(50);
        *grape_count += 10;
        assert_eq!(map.get("grape"), Some(&60));

        // 测试Entry的key方法
        let orange_entry = map.entry("orange");
        assert_eq!(orange_entry.key(), &"orange");
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
}
