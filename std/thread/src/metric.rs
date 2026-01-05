use core::fmt;
// metric data structure
// 基本功能 inc/dec/snapshot
use std::sync::Arc;

use anyhow::Result;
use dashmap::DashMap;
#[derive(Debug, Clone)]
pub struct Metric {
    data: Arc<DashMap<String, i64>>,
}
impl Default for Metric {
    fn default() -> Self {
        Self::new()
    }
}
impl Metric {
    pub fn new() -> Self {
        Self {
            data: Arc::new(DashMap::new()),
        }
    }

    pub fn inc(&self, key: impl Into<String>) -> Result<()> {
        self.data
            .entry(key.into())
            .and_modify(|v| *v += 1)
            .or_insert(1);
        Ok(())
    }
    pub fn dec(&self, key: impl Into<String>) -> Result<()> {
        self.data
            .entry(key.into())
            .and_modify(|v| *v -= 1)
            .or_insert(-1);
        Ok(())
    }
}
impl fmt::Display for Metric {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for entry in self.data.iter() {
            writeln!(f, "{}: {}", entry.key(), entry.value())?;
        }
        Ok(())
    }
}
