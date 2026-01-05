// metric data structure
// 基本功能 inc/dec/snapshot
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use anyhow::Result;
#[derive(Debug, Clone)]
pub struct Metric {
    data: Arc<Mutex<HashMap<String, i64>>>,
}
impl Default for Metric {
    fn default() -> Self {
        Self::new()
    }
}
impl Metric {
    pub fn new() -> Self {
        Self {
            data: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn inc(&self, key: impl Into<String>) -> Result<()> {
        self.data
            .lock()
            .map_err(|e| anyhow::anyhow!("{:?}", e))?
            .entry(key.into())
            .and_modify(|v| *v += 1)
            .or_insert(1);
        Ok(())
    }
    pub fn dec(&self, key: impl Into<String>) -> Result<()> {
        self.data
            .lock()
            .map_err(|e| anyhow::anyhow!("{:?}", e))?
            .entry(key.into())
            .and_modify(|v| *v -= 1)
            .or_insert(-1);
        Ok(())
    }
    pub fn snapshot(&self) -> Result<HashMap<String, i64>> {
        Ok(self
            .data
            .lock()
            .map_err(|e| anyhow::anyhow!("{:?}", e))?
            .clone())
    }
}
