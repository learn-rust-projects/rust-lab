mod rocksdb;

use rocksdb::config::RocksDbConfig;
use rocksdb::engine::RocksDBEngine;
use std::sync::Arc;

fn main() {
    let config =
        RocksDbConfig::load_from_file("config.example.yml").expect("Failed to load config");
    let rocksdb_engine: Arc<RocksDBEngine> =
        Arc::new(RocksDBEngine::new(&config).expect("Failed to create engine"));

    println!("RocksDB engine created successfully at: {}", config.path);
}
