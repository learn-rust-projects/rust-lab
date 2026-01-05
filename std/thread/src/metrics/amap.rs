use std::{
    collections::HashMap,
    sync::{
        Arc,
        atomic::{AtomicI64, Ordering},
    },
};

use anyhow::Result;
#[derive(Debug)]
pub struct AmapMetrics {
    pub data: Arc<HashMap<&'static str, AtomicI64>>,
}
impl AmapMetrics {
    pub fn new(metric_names: &[&'static str]) -> Self {
        let data = Arc::new(
            metric_names
                .iter()
                .map(|&name| (name, AtomicI64::new(0)))
                .collect(),
        );
        AmapMetrics { data }
    }
}
impl Clone for AmapMetrics {
    fn clone(&self) -> Self {
        Self {
            data: self.data.clone(),
        }
    }
}
impl AmapMetrics {
    pub fn inc(&self, name: impl AsRef<str>) -> Result<()> {
        self.data
            .get(name.as_ref())
            .ok_or_else(|| anyhow::anyhow!("metric {} not found", name.as_ref()))?
            .fetch_add(1, Ordering::Relaxed);
        Ok(())
    }
    pub fn dec(&self, name: impl AsRef<str>) -> Result<()> {
        self.data
            .get(name.as_ref())
            .ok_or_else(|| anyhow::anyhow!("metric {} not found", name.as_ref()))?
            .fetch_sub(1, Ordering::Relaxed);
        Ok(())
    }
}

impl std::fmt::Display for AmapMetrics {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (name, value) in self.data.iter() {
            writeln!(f, "{}: {}", name, value.load(Ordering::Relaxed))?;
        }
        Ok(())
    }
}
