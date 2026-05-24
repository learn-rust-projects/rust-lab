pub mod config;
pub mod engine;
pub mod kv;
pub mod storage;

/// RocksDB 错误类型
#[derive(Debug)]
pub enum RocksDbError {
    /// 序列化错误
    SerializeError(String),
    /// 反序列化错误
    DeserializeError(String),
    /// 写入错误
    WriteError(String),
    /// 读取错误
    ReadError(String),
    /// 列族不存在
    ColumnFamilyNotFound(String),
    /// 列族不可用
    ColumnFamilyNotAvailable,
    /// 其他 RocksDB 错误
    RocksDb(rocksdb::Error),
}

impl std::fmt::Display for RocksDbError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RocksDbError::SerializeError(msg) => write!(f, "Serialize error: {}", msg),
            RocksDbError::DeserializeError(msg) => write!(f, "Deserialize error: {}", msg),
            RocksDbError::WriteError(msg) => write!(f, "Write error: {}", msg),
            RocksDbError::ReadError(msg) => write!(f, "Read error: {}", msg),
            RocksDbError::ColumnFamilyNotFound(name) => {
                write!(f, "Column family not found: {}", name)
            }
            RocksDbError::ColumnFamilyNotAvailable => write!(f, "Column family not available"),
            RocksDbError::RocksDb(e) => write!(f, "RocksDB error: {}", e),
        }
    }
}

impl std::error::Error for RocksDbError {}

impl From<rocksdb::Error> for RocksDbError {
    fn from(err: rocksdb::Error) -> Self {
        RocksDbError::RocksDb(err)
    }
}
