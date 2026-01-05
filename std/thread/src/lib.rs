mod martix;
mod metrics;
mod vector;

pub use martix::{Matrix, multiply};
pub use metrics::{AmapMetrics, CmapMetric};
pub use vector::{Vector, dot_product};
