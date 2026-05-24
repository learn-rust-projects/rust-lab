use std::sync::Arc;

use super::engine::{RocksDBEngine, StorageDataWrap};
use super::storage::{engine_save, DB_COLUMN_FAMILY_CLUSTER};
use super::RocksDbError;

/// Key-Value 存储接口
pub struct KvStorage {
    rocksdb_engine_handler: Arc<RocksDBEngine>,
}

impl KvStorage {
    /// 创建新的 KvStorage 实例
    pub fn new(rocksdb_engine_handler: Arc<RocksDBEngine>) -> Self {
        Self {
            rocksdb_engine_handler,
        }
    }

    /// 设置键值对
    ///
    /// # 参数
    /// - `key`: 键
    /// - `value`: 值（实现 Serialize）
    ///
    /// # 返回
    /// - `Ok(())`: 保存成功
    /// - `Err(RocksDbError)`: 保存失败
    pub fn set<T: serde::Serialize>(&self, key: String, value: T) -> Result<(), RocksDbError> {
        engine_save(
            self.rocksdb_engine_handler.clone(),
            DB_COLUMN_FAMILY_CLUSTER,
            key,
            value,
        )
    }

    /// 获取指定 key 的值
    ///
    /// # 参数
    /// - `key`: 要获取的键
    ///
    /// # 返回
    /// - `Ok(Some(value))`: 找到值
    /// - `Ok(None)`: key 不存在
    /// - `Err(RocksDbError)`: 获取失败
    pub fn get(&self, key: &str) -> Result<Option<String>, RocksDbError> {
        let cf = self
            .rocksdb_engine_handler
            .cf_cluster(DB_COLUMN_FAMILY_CLUSTER)?;

        let result: Result<Option<StorageDataWrap>, String> =
            self.rocksdb_engine_handler.read(cf, key);

        match result {
            Ok(Some(data_wrap)) => Ok(Some(String::from_utf8(data_wrap.data).map_err(
                |e| RocksDbError::DeserializeError(e.to_string()),
            )?)),
            Ok(None) => Ok(None),
            Err(e) => Err(RocksDbError::ReadError(e)),
        }
    }

    /// 删除指定 key
    ///
    /// # 参数
    /// - `key`: 要删除的键
    ///
    /// # 返回
    /// - `Ok(())`: 删除成功
    /// - `Err(RocksDbError)`: 删除失败
    pub fn delete(&self, key: &str) -> Result<(), RocksDbError> {
        let cf = self
            .rocksdb_engine_handler
            .cf_cluster(DB_COLUMN_FAMILY_CLUSTER)?;

        self.rocksdb_engine_handler
            .delete(cf, key)
            .map_err(|e| RocksDbError::WriteError(e.to_string()))
    }

    /// 判断 key 是否存在
    ///
    /// # 参数
    /// - `key`: 要检查的键
    ///
    /// # 返回
    /// - `true`: key 可能存在
    /// - `false`: key 不存在
    pub fn exists(&self, key: &str) -> bool {
        let cf = match self
            .rocksdb_engine_handler
            .cf_cluster(DB_COLUMN_FAMILY_CLUSTER)
        {
            Ok(cf) => cf,
            Err(_) => return false,
        };

        self.rocksdb_engine_handler.exists(cf, key)
    }

    /// 列出所有具有指定前缀的 key
    ///
    /// # 参数
    /// - `prefix`: key 前缀
    ///
    /// # 返回
    /// - `Ok(Vec<String>)`: 匹配前缀的所有 key
    /// - `Err(RocksDbError)`: 列出失败
    pub fn list(&self, prefix: &str) -> Result<Vec<String>, RocksDbError> {
        let cf = self
            .rocksdb_engine_handler
            .cf_cluster(DB_COLUMN_FAMILY_CLUSTER)?;

        let results = self.rocksdb_engine_handler.read_prefix(cf, prefix);

        // read_prefix 返回 Vec<HashMap<String, Vec<u8>>>，每个 HashMap 的 key 是数据库中的 key
        let keys: Vec<String> = results
            .into_iter()
            .flat_map(|item| item.into_keys())
            .collect();

        Ok(keys)
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::sync::atomic::{AtomicU32, Ordering};

    use super::*;
    use super::super::config::RocksDbConfig;

    static TEST_COUNTER: AtomicU32 = AtomicU32::new(0);

    /// 测试辅助结构体，自动清理临时目录
    struct TestContext {
        path: String,
        kv: KvStorage,
    }

    impl Drop for TestContext {
        fn drop(&mut self) {
            // 测试结束后清理临时目录
            let _ = fs::remove_dir_all(&self.path);
        }
    }

    fn test_config(path: &str) -> RocksDbConfig {
        serde_yaml::from_str(&format!(
            r#"
path: {}
create_if_missing: true
create_missing_column_families: true
max_open_files: 1000
use_fsync: false
bytes_per_sync: 8388608
block_cache_size_mb: 1024
table_cache_num_shard_bits: 6
max_write_buffer_number: 32
write_buffer_size: 536870912
target_file_size_base: 1073741824
min_write_buffer_number_to_merge: 4
level_zero_stop_writes_trigger: 2000
level_zero_slowdown_writes_trigger: 0
compaction_style: universal
disable_auto_compactions: true
prefix_length: 10
memtable_prefix_bloom_ratio: 0.2
"#,
            path
        ))
        .unwrap()
    }

    fn setup_kv_storage() -> TestContext {
        let id = TEST_COUNTER.fetch_add(1, Ordering::SeqCst);
        let path = format!("/tmp/test_kv_rocksdb_{}", id);

        // 清理之前的测试数据
        let _ = fs::remove_dir_all(&path);

        let config = test_config(&path);
        let engine = Arc::new(RocksDBEngine::new(&config).unwrap());
        let kv = KvStorage::new(engine);

        TestContext { path, kv }
    }

    #[test]
    fn test_kv_storage_set_and_get() {
        let ctx = setup_kv_storage();

        // 设置键值对
        let result = ctx.kv.set("key1".to_string(), "value1".to_string());
        assert!(result.is_ok());

        // 获取值（JSON 序列化后的字符串带引号）
        let result = ctx.kv.get("key1");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Some("\"value1\"".to_string()));
    }

    #[test]
    fn test_kv_storage_get_nonexistent_key() {
        let ctx = setup_kv_storage();

        // 获取不存在的 key
        let result = ctx.kv.get("nonexistent");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), None);
    }

    #[test]
    fn test_kv_storage_delete() {
        let ctx = setup_kv_storage();

        // 设置键值对
        ctx.kv.set("key1".to_string(), "value1".to_string()).unwrap();

        // 验证存在
        assert!(ctx.kv.exists("key1"));

        // 删除
        let result = ctx.kv.delete("key1");
        assert!(result.is_ok());

        // 验证不存在
        assert!(!ctx.kv.exists("key1"));
    }

    #[test]
    fn test_kv_storage_exists() {
        let ctx = setup_kv_storage();

        // 设置键值对
        ctx.kv.set("key1".to_string(), "value1".to_string()).unwrap();

        // 验证存在
        assert!(ctx.kv.exists("key1"));

        // 验证不存在的 key
        assert!(!ctx.kv.exists("nonexistent_key"));
    }

    #[test]
    fn test_kv_storage_list() {
        let ctx = setup_kv_storage();

        // 设置多个具有相同前缀的键值对
        ctx.kv.set("user:1".to_string(), "Alice".to_string()).unwrap();
        ctx.kv.set("user:2".to_string(), "Bob".to_string()).unwrap();
        ctx.kv.set("user:3".to_string(), "Charlie".to_string()).unwrap();
        ctx.kv.set("order:1".to_string(), "Order1".to_string()).unwrap();

        // 列出 user: 前缀的 key
        let result = ctx.kv.list("user:");
        assert!(result.is_ok());
        let keys = result.unwrap();
        assert_eq!(keys.len(), 3);
        assert!(keys.contains(&"user:1".to_string()));
        assert!(keys.contains(&"user:2".to_string()));
        assert!(keys.contains(&"user:3".to_string()));
    }

    #[test]
    fn test_kv_storage_list_empty_prefix() {
        let ctx = setup_kv_storage();

        // 列出空前缀（应该返回所有 key）
        ctx.kv.set("a".to_string(), "1".to_string()).unwrap();
        ctx.kv.set("b".to_string(), "2".to_string()).unwrap();

        let result = ctx.kv.list("");
        assert!(result.is_ok());
        assert!(result.unwrap().len() >= 2);
    }

    #[test]
    fn test_kv_storage_update_value() {
        let ctx = setup_kv_storage();

        // 设置初始值
        ctx.kv.set("key1".to_string(), "value1".to_string()).unwrap();

        // 更新值
        let result = ctx.kv.set("key1".to_string(), "value2".to_string());
        assert!(result.is_ok());

        // 验证新值（JSON 序列化后的字符串带引号）
        let result = ctx.kv.get("key1");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Some("\"value2\"".to_string()));
    }

    #[test]
    fn test_kv_storage_set_with_complex_value() {
        #[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq)]
        struct User {
            name: String,
            age: u32,
        }

        let ctx = setup_kv_storage();

        let user = User {
            name: "Alice".to_string(),
            age: 30,
        };

        // 设置复杂类型值
        let result = ctx.kv.set("user:alice".to_string(), user);
        assert!(result.is_ok());

        // 获取值并验证
        let result = ctx.kv.get("user:alice");
        assert!(result.is_ok());
        let value = result.unwrap().unwrap();

        // 验证 JSON 格式
        assert!(value.contains("Alice"));
        assert!(value.contains("30"));
    }
}
