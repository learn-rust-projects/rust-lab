use std::fs;
use std::path::Path;

use rocksdb::{DBCompactionStyle, Options, SliceTransform};
use serde::Deserialize;

/// RocksDB 配置结构体
#[derive(Debug, Clone, Deserialize)]
pub struct RocksDbConfig {
    /// 数据库路径
    pub path: String,

    /// 是否在数据库不存在时创建
    pub create_if_missing: bool,

    /// 是否在列族不存在时创建
    pub create_missing_column_families: bool,

    /// 最大打开文件数
    pub max_open_files: i32,

    /// 是否禁用 fsync
    pub use_fsync: bool,

    /// 每写入指定字节后执行一次同步
    pub bytes_per_sync: u64,

    /// 为点查询优化的块缓存大小 (MB)
    pub block_cache_size_mb: u64,

    /// 表缓存分片数 (2^n)
    pub table_cache_num_shard_bits: i32,

    /// 最大写缓冲区数量
    pub max_write_buffer_number: i32,

    /// 写缓冲区大小 (bytes)
    pub write_buffer_size: usize,

    /// 目标文件大小基础值 (bytes)
    pub target_file_size_base: u64,

    /// 最小写缓冲区数量以触发合并
    pub min_write_buffer_number_to_merge: i32,

    /// L0 层停止写入触发器
    pub level_zero_stop_writes_trigger: i32,

    /// L0 层减速写入触发器
    pub level_zero_slowdown_writes_trigger: i32,

    /// 压缩样式: "universal", "level", "fifo"
    pub compaction_style: String,

    /// 是否禁用自动压缩
    pub disable_auto_compactions: bool,

    /// 前缀提取器 - 固定前缀长度 (bytes)
    pub prefix_length: usize,

    /// Memtable 前缀 Bloom 过滤比例
    pub memtable_prefix_bloom_ratio: f64,
}

impl RocksDbConfig {
    /// 转换为 RocksDB Options
    pub fn to_options(&self) -> Options {
        let mut opts = Options::default();

        opts.create_if_missing(self.create_if_missing);
        opts.create_missing_column_families(self.create_missing_column_families);
        opts.set_max_open_files(self.max_open_files);
        opts.set_use_fsync(self.use_fsync);
        opts.set_bytes_per_sync(self.bytes_per_sync);
        opts.optimize_for_point_lookup(self.block_cache_size_mb);
        opts.set_table_cache_num_shard_bits(self.table_cache_num_shard_bits);
        opts.set_max_write_buffer_number(self.max_write_buffer_number);
        opts.set_write_buffer_size(self.write_buffer_size);
        opts.set_target_file_size_base(self.target_file_size_base);
        opts.set_min_write_buffer_number_to_merge(self.min_write_buffer_number_to_merge);
        opts.set_level_zero_stop_writes_trigger(self.level_zero_stop_writes_trigger);
        opts.set_level_zero_slowdown_writes_trigger(self.level_zero_slowdown_writes_trigger);

        let compaction_style = match self.compaction_style.as_str() {
            "universal" => DBCompactionStyle::Universal,
            "level" => DBCompactionStyle::Level,
            "fifo" => DBCompactionStyle::Fifo,
            _ => DBCompactionStyle::Universal,
        };
        opts.set_compaction_style(compaction_style);

        opts.set_disable_auto_compactions(self.disable_auto_compactions);

        let transform = SliceTransform::create_fixed_prefix(self.prefix_length);
        opts.set_prefix_extractor(transform);

        opts.set_memtable_prefix_bloom_ratio(self.memtable_prefix_bloom_ratio);

        opts
    }

    /// 从 YAML 文件加载配置
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self, String> {
        let content =
            fs::read_to_string(path).map_err(|e| format!("Failed to read config file: {}", e))?;
        serde_yaml::from_str(&content).map_err(|e| format!("Failed to parse config: {}", e))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
    fn test_config_deserialization() {
        let config = test_config();
        assert_eq!(config.path, "/tmp/testdb");
        assert_eq!(config.max_open_files, 1000);
    }
}
