use std::collections::HashMap;

use rocksdb::{ColumnFamily, DB};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

use super::config::RocksDbConfig;
use super::RocksDbError;

/// 默认列族名称
const DEFAULT_COLUMN_FAMILY: &str = "default";

/// 存储数据包装结构
#[derive(Serialize, Deserialize, Debug)]
pub struct StorageDataWrap {
    /// 数据内容
    pub data: Vec<u8>,
    /// 创建时间（Unix 时间戳，秒）
    pub create_time: u64,
}

impl StorageDataWrap {
    /// 创建新的存储数据包装
    pub fn new(data: Vec<u8>) -> Self {
        Self {
            data,
            create_time: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }
    }
}

/// RocksDB 数据库引擎
pub struct RocksDBEngine {
    db: DB,
}

impl RocksDBEngine {
    /// 创建新的 RocksDB 引擎实例
    pub fn new(config: &RocksDbConfig) -> Result<Self, rocksdb::Error> {
        let opts = config.to_options();
        // 打开数据库并包含默认列族
        let db = DB::open_cf(&opts, &config.path, ["default"])?;
        Ok(Self { db })
    }

    /// 获取底层 DB 实例的只读引用
    pub fn db(&self) -> &DB {
        &self.db
    }

    /// 获取指定集群对应的 ColumnFamily
    ///
    /// 如果集群名称有效，返回对应的 ColumnFamily 句柄。
    /// 如果集群不存在或不可用，返回错误。
    ///
    /// # 参数
    /// - `rocksdb_cluster`: 集群标识字符串，空字符串表示默认列族
    ///
    /// # 返回
    /// - `Ok(&ColumnFamily)`: 成功获取列族句柄
    /// - `Err(RocksDbError)`: 获取失败
    pub fn cf_cluster(&self, rocksdb_cluster: &str) -> Result<&ColumnFamily, RocksDbError> {
        // 空字符串或 "default" 都使用默认列族
        let cf_name = if rocksdb_cluster.is_empty() || rocksdb_cluster == "default" {
            DEFAULT_COLUMN_FAMILY
        } else {
            rocksdb_cluster
        };

        self.db
            .cf_handle(cf_name)
            .ok_or_else(|| {
                if rocksdb_cluster.is_empty() || rocksdb_cluster == "default" {
                    RocksDbError::ColumnFamilyNotFound(cf_name.to_string())
                } else {
                    RocksDbError::ColumnFamilyNotAvailable
                }
            })
    }

    /// 写入数据到指定 ColumnFamily
    pub fn write<T: Serialize + std::fmt::Debug>(
        &self,
        cf: &ColumnFamily,
        key: &str,
        value: &T,
    ) -> Result<(), String> {
        match serde_json::to_string(&value) {
            Ok(serialized) => self
                .db
                .put_cf(cf, key, serialized.into_bytes())
                .map_err(|err| format!("Failed to put to ColumnFamily: {:?}", err)),
            Err(err) => Err(format!(
                "Failed to serialize to String. T: {:?}, err: {:?}",
                value, err
            )),
        }
    }

    /// 从指定 ColumnFamily 读取数据
    pub fn read<T: DeserializeOwned>(
        &self,
        cf: &ColumnFamily,
        key: &str,
    ) -> Result<Option<T>, String> {
        match self.db.get_cf(cf, key) {
            Ok(opt) => match opt {
                Some(found) => match String::from_utf8(found) {
                    Ok(s) => match serde_json::from_str::<T>(&s) {
                        Ok(t) => Ok(Some(t)),
                        Err(err) => Err(format!("Failed to deserialize: {:?}", err)),
                    },
                    Err(err) => Err(format!("Failed to convert to String: {:?}", err)),
                },
                None => Ok(None),
            },
            Err(err) => Err(format!("Failed to get from ColumnFamily: {:?}", err)),
        }
    }

    /// 从指定 ColumnFamily 删除数据
    pub fn delete(&self, cf: &ColumnFamily, key: &str) -> Result<(), rocksdb::Error> {
        self.db.delete_cf(cf, key)
    }

    /// 判断 key 是否存在于指定 ColumnFamily
    pub fn exists(&self, cf: &ColumnFamily, key: &str) -> bool {
        self.db.key_may_exist_cf(cf, key)
    }

    /// 按前缀搜索数据
    ///
    /// 使用 RocksDB 的 raw_iterator_cf 获取指定 ColumnFamily 的迭代器，
    /// 通过 seek 到 search_key 快速定位到第一个匹配的 key，
    /// 然后遍历后续所有具有相同前缀的 key-value 对。
    ///
    /// # 参数
    /// - `cf`: ColumnFamily 句柄
    /// - `search_key`: 要搜索的前缀 key
    ///
    /// # 返回
    /// 返回包含所有匹配前缀的 key-value 对的向量，每个元素是一个 HashMap，
    /// key 为 UTF-8 字符串，value 为原始字节数组。
    ///
    /// # 示例
    /// ```
    /// let results = engine.read_prefix(cf, "user:");
    /// for item in results {
    ///     for (key, value) in item {
    ///         println!("{}: {:?}", key, value);
    ///     }
    /// }
    /// ```
    pub fn read_prefix(
        &self,
        cf: &ColumnFamily,
        search_key: &str,
    ) -> Vec<HashMap<String, Vec<u8>>> {
        // 获取 ColumnFamily 的原始迭代器
        let mut iter = self.db.raw_iterator_cf(cf);

        // seek 到第一个 >= search_key 的位置
        iter.seek(search_key);

        let mut result = Vec::new();

        // 遍历迭代器，直到遇到不匹配前缀的 key 或迭代结束
        while iter.valid() {
            // 获取当前的 key 和 value
            let key = iter.key();
            let value = iter.value();

            // 如果 key 或 value 为空，退出循环
            if key.is_none() || value.is_none() {
                break;
            }

            // 将 key 转换为 UTF-8 字符串
            let result_key = match String::from_utf8(key.unwrap().to_vec()) {
                Ok(s) => s,
                // 跳过无法解析为 UTF-8 的 key
                Err(_) => {
                    iter.next();
                    continue;
                }
            };

            // 如果 key 不以 search_key 开头，说明已遍历完所有匹配前缀的 key
            if !result_key.starts_with(search_key) {
                break;
            }

            // 构建 HashMap 并添加到结果集
            let mut raw = HashMap::new();
            raw.insert(result_key, value.unwrap().to_vec());
            result.push(raw);

            // 移动到下一个 key
            iter.next();
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::super::config::RocksDbConfig;
    use super::RocksDBEngine;

    fn test_config() -> RocksDbConfig {
        serde_yaml::from_str(
            r#"
path: /tmp/testdb
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
        )
        .unwrap()
    }

    #[test]
    fn test_rocksdb_engine_creation() {
        let config = test_config();
        let result = RocksDBEngine::new(&config);
        assert!(result.is_ok());
    }
}
