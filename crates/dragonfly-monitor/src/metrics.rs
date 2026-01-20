//! System metrics data types

use serde::{Deserialize, Serialize};

/// System metrics snapshot
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct SystemMetrics {
    /// CPU usage percentage
    pub cpu_usage: f32,
    /// Memory usage percentage
    pub memory_usage: f32,
    /// Disk usage percentage
    pub disk_usage: f32,
}

impl SystemMetrics {
    /// Create new metrics
    pub fn new(cpu: f32, memory: f32, disk: f32) -> Self {
        Self {
            cpu_usage: cpu,
            memory_usage: memory,
            disk_usage: disk,
        }
    }
}
