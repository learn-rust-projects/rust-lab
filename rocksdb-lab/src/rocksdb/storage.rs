use std::sync::Arc;

use serde::Serialize;

use super::engine::{RocksDBEngine, StorageDataWrap};
use super::RocksDbError;

/// 默认列族名称（集群列族）- RocksDB 中空字符串表示默认列族
pub const DB_COLUMN_FAMILY_CLUSTER: &str = "";

/// 保存数据到 RocksDB
///
/// # 类型参数
/// - `T`: 要保存的数据类型，必须实现 `Serialize`
///
/// # 参数
/// - `rocksdb_engine_handler`: RocksDB 引擎句柄（Arc 包装）
/// - `rocksdb_cluster`: 集群标识，用于确定 ColumnFamily
/// - `key_name`: 存储的 key 名称
/// - `value`: 要存储的值
///
/// # 返回
/// - `Ok(())`: 保存成功
/// - `Err(RocksDbError)`: 保存失败
pub fn engine_save<T>(
    rocksdb_engine_handler: Arc<RocksDBEngine>,
    rocksdb_cluster: &str,
    key_name: String,
    value: T,
) -> Result<(), RocksDbError>
where
    T: Serialize,
{
    // 检查集群标识是否匹配默认列族
    let cf = if rocksdb_cluster.to_string() == DB_COLUMN_FAMILY_CLUSTER.to_string() {
        rocksdb_engine_handler.cf_cluster(DB_COLUMN_FAMILY_CLUSTER)?
    } else {
        return Err(RocksDbError::ColumnFamilyNotAvailable);
    };

    // 序列化数据
    let content = serde_json::to_vec(&value)
        .map_err(|e| RocksDbError::SerializeError(e.to_string()))?;

    // 包装数据（包含创建时间）
    let data = StorageDataWrap::new(content);

    // 写入 RocksDB
    rocksdb_engine_handler
        .write(cf, &key_name, &data)
        .map_err(RocksDbError::WriteError)
}
