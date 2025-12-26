use std::collections::HashMap;

use radix_trie::{Trie, TrieCommon};

fn main() {
    println!("=== Radix Trie 功能演示 ===\n");

    // 1. 创建和插入数据
    println!("1. 创建Trie并插入数据:");
    let mut trie: Trie<&str, i32> = Trie::new();

    // 插入一些键值对
    trie.insert("apple", 1);
    trie.insert("app", 2);
    trie.insert("application", 3);
    trie.insert("banana", 4);
    trie.insert("band", 5);
    trie.insert("bandana", 6);
    trie.insert("cat", 7);
    trie.insert("catalog", 8);

    println!("   插入的键: apple, app, application, banana, band, bandana, cat, catalog");
    println!();

    // 2. 查找功能演示
    println!("2. 查找功能演示:");
    if let Some(value) = trie.get("apple") {
        println!("   查找 'apple': {}", value);
    }

    if let Some(value) = trie.get("app") {
        println!("   查找 'app': {}", value);
    }

    if let Some(value) = trie.get("nonexistent") {
        println!("   查找 'nonexistent': {}", value);
    } else {
        println!("   查找 'nonexistent': 未找到");
    }
    println!();

    // 3. 前缀查找演示
    println!("3. 前缀查找演示:");
    println!("   以 'app' 为前缀的键:");
    for (key, value) in trie.get_raw_descendant("app").unwrap().iter() {
        println!("     {} -> {}", key, value);
    }
    println!();

    // 4. 最长前缀匹配演示
    println!("4. 最长前缀匹配演示:");
    if let Some(sub_trie) = trie.get_ancestor("application") {
        println!(
            "   'application' 的最长前缀匹配: {} -> {}",
            sub_trie.key().unwrap(),
            sub_trie.value().unwrap()
        );
    }

    if let Some(sub_trie) = trie.get_ancestor("applications") {
        println!(
            "   'applications' 的最长前缀匹配: {} -> {}",
            sub_trie.key().unwrap(),
            sub_trie.value().unwrap()
        );
    } else {
        println!("   'applications' 的最长前缀匹配: 未找到完全匹配");
    }
    println!();

    // 5. 删除功能演示
    println!("5. 删除功能演示:");
    if let Some(old_value) = trie.remove("app") {
        println!("   删除 'app', 原值: {}", old_value);
    }

    // 验证删除
    if trie.get("app").is_none() {
        println!("   验证: 'app' 已成功删除");
    }
    println!();

    // 6. 遍历功能演示
    println!("6. 遍历功能演示:");
    println!("   当前Trie中的所有键值对:");
    for (key, value) in trie.iter() {
        println!("     {} -> {}", key, value);
    }
    println!();

    // 7. 前缀迭代器演示
    println!("7. 前缀迭代器演示:");
    println!("   以 'ban' 为前缀的所有键值对:");
    for (key, value) in trie.get_raw_descendant("ban").unwrap().iter() {
        println!("     {} -> {}", key, value);
    }
    println!();

    // 8. 键的存在性检查
    println!("8. 存在性检查演示:");
    println!("   'apple' 存在: {}", trie.keys().any(|k| *k == "apple"));
    println!("   'app' 存在: {}", trie.keys().any(|k| *k == "app"));
    println!(
        "   'application' 存在: {}",
        trie.keys().any(|k| *k == "application")
    );
    println!();

    // 9. 大小和空检查
    println!("9. 大小和空检查:");
    println!("   Trie大小: {}", trie.len());
    println!("   Trie是否为空: {}", trie.is_empty());
    println!();

    // 10. 从HashMap构建Trie
    println!("10. 从HashMap构建Trie:");
    let map: HashMap<&str, i32> = [("dog", 9), ("dogma", 10), ("dogmatic", 11), ("doge", 12)]
        .iter()
        .cloned()
        .collect();

    let map_trie: Trie<&str, i32> = map.into_iter().collect();
    println!("   从HashMap构建的Trie:");
    for (key, value) in map_trie.iter() {
        println!("     {} -> {}", key, value);
    }
    println!();

    // 11. 子Trie操作演示
    println!("11. 子Trie操作演示:");
    if let Some(subtrie) = trie.subtrie("ban") {
        println!("   'ban' 子Trie中的键值对:");
        for (key, value) in subtrie.iter() {
            println!("     {} -> {}", key, value);
        }
    }
    println!();

    // 12. 获取所有键和值
    println!("12. 获取所有键和值:");
    println!("   所有键: {:?}", trie.keys().collect::<Vec<_>>());
    println!("   所有值: {:?}", trie.values().collect::<Vec<_>>());
    println!();

    // 13. 清空Trie演示
    println!("13. 清空Trie演示:");
    let mut temp_trie = trie.clone();
    temp_trie = Trie::new();
    println!("   清空后Trie大小: {}", temp_trie.len());
    println!("   清空后Trie是否为空: {}", temp_trie.is_empty());
    println!();

    // 14. 实际应用场景演示 - 自动补全
    println!("14. 实际应用场景演示 - 自动补全:");
    println!("   输入 'ba' 的自动补全建议:");
    for (key, _) in trie.get_raw_descendant("ba").unwrap().iter() {
        println!("     - {}", key);
    }
    println!();

    // 15. 实际应用场景演示 - 路由匹配
    println!("15. 实际应用场景演示 - 路由匹配:");
    let mut route_trie: Trie<&str, &str> = Trie::new();
    route_trie.insert("/api/users", "用户API");
    route_trie.insert("/api/users/:id", "用户详情API");
    route_trie.insert("/api/products", "产品API");
    route_trie.insert("/admin", "管理后台");

    let test_routes = [
        "/api/users",
        "/api/users/123",
        "/api/products",
        "/admin",
        "/unknown",
    ];

    for route in test_routes {
        if let Some(sub_trie) = route_trie.get_ancestor(route) {
            println!(
                "   路由 '{}' 匹配到: {} -> {}",
                route,
                sub_trie.key().unwrap(),
                sub_trie.value().unwrap()
            );
        } else {
            println!("   路由 '{}' 未匹配到处理器", route);
        }
    }
    println!();

    // 16. 性能优势演示 - 前缀共享
    println!("16. 性能优势演示 - 前缀共享:");
    println!("   Radix Trie 的优势在于共享公共前缀，节省内存");
    println!("   例如: 'application' 和 'apple' 共享 'app' 前缀");
    println!("   例如: 'banana', 'band', 'bandana' 共享 'ban' 前缀");
    println!();

    println!("=== Radix Trie 演示结束 ===");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trie_insert_and_get() {
        let mut trie: Trie<&str, i32> = Trie::new();

        // 测试插入和获取
        trie.insert("test", 42);
        assert_eq!(trie.get("test"), Some(&42));
        assert_eq!(trie.get("nonexistent"), None);
    }

    #[test]
    fn test_trie_contains_key() {
        let mut trie: Trie<&str, i32> = Trie::new();

        trie.insert("key1", 1);
        trie.insert("key2", 2);

        assert!(trie.keys().any(|k| *k == "key1"));
        assert!(trie.keys().any(|k| *k == "key2"));
        assert!(!trie.keys().any(|k| *k == "key3"));
    }

    #[test]
    fn test_trie_remove() {
        let mut trie: Trie<&str, i32> = Trie::new();

        trie.insert("remove_me", 100);
        assert!(trie.keys().any(|k| *k == "remove_me"));

        let removed_value = trie.remove("remove_me");
        assert_eq!(removed_value, Some(100));
        assert!(!trie.keys().any(|k| *k == "remove_me"));
    }

    #[test]
    fn test_trie_prefix_iter() {
        let mut trie: Trie<&str, i32> = Trie::new();

        trie.insert("apple", 1);
        trie.insert("app", 2);
        trie.insert("application", 3);
        trie.insert("banana", 4);

        let prefix_keys: Vec<&str> = trie
            .get_raw_descendant("app")
            .unwrap()
            .keys()
            .map(|k| *k)
            .collect();
        assert_eq!(prefix_keys, vec!["app", "apple", "application"]);
    }

    #[test]
    fn test_trie_get_ancestor() {
        let mut trie: Trie<&str, i32> = Trie::new();

        trie.insert("app", 1);
        trie.insert("apple", 2);

        // 精确匹配
        if let Some(sub_trie) = trie.get_ancestor("app") {
            assert_eq!(*sub_trie.key().unwrap(), "app");
            assert_eq!(sub_trie.value().unwrap(), &1);
        }
        // 最长前缀匹配
        if let Some(sub_trie) = trie.get_ancestor("application") {
            assert_eq!(*sub_trie.key().unwrap(), "app");
            assert_eq!(sub_trie.value().unwrap(), &1);
        }
    }

    #[test]
    fn test_trie_len_and_is_empty() {
        let mut trie: Trie<&str, i32> = Trie::new();

        assert!(trie.is_empty());
        assert_eq!(trie.len(), 0);

        trie.insert("key", 1);
        assert!(!trie.is_empty());
        assert_eq!(trie.len(), 1);

        trie.remove("key");
        assert!(trie.is_empty());
        assert_eq!(trie.len(), 0);
    }

    #[test]
    fn test_trie_iter() {
        let mut trie: Trie<&str, i32> = Trie::new();

        trie.insert("a", 1);
        trie.insert("b", 2);
        trie.insert("c", 3);

        let mut keys: Vec<&str> = trie.iter().map(|(k, _)| *k).collect();
        keys.sort();

        assert_eq!(keys, vec!["a", "b", "c"]);
    }

    #[test]
    fn test_trie_clear() {
        let mut trie: Trie<&str, i32> = Trie::new();

        trie.insert("key1", 1);
        trie.insert("key2", 2);
        assert_eq!(trie.len(), 2);

        // 清空 Trie
        trie = Trie::new();
        assert!(trie.is_empty());
        assert_eq!(trie.len(), 0);
    }

    #[test]
    fn test_trie_from_iterator() {
        let items = vec![("key1", 1), ("key2", 2), ("key3", 3)];
        let trie: Trie<&str, i32> = items.into_iter().collect();

        assert_eq!(trie.len(), 3);
        assert!(trie.keys().any(|k| *k == "key1"));
        assert!(trie.keys().any(|k| *k == "key2"));
        assert!(trie.keys().any(|k| *k == "key3"));
    }

    #[test]
    fn test_trie_subtrie() {
        let mut trie: Trie<&str, i32> = Trie::new();

        trie.insert("apple", 1);
        trie.insert("application", 2);
        trie.insert("banana", 3);

        if let Some(subtrie) = trie.subtrie("app") {
            let keys: Vec<&str> = subtrie.iter().map(|(k, _)| *k).collect();
            assert_eq!(keys, vec!["apple", "application"]);
        }
    }
}
