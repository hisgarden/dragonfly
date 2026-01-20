//! System metrics data types

use serde::{Deserialize, Serialize};

/// System metrics snapshot
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct SystemMetrics {
    /// CPU usage percentage
    pub cpu_usage_percent: f32,
    /// Total memory in bytes
    pub memory_total_bytes: u64,
    /// Used memory in bytes
    pub memory_used_bytes: u64,
    /// Available memory in bytes
    pub memory_available_bytes: u64,
    /// Total swap in bytes
    pub swap_total_bytes: u64,
    /// Used swap in bytes
    pub swap_used_bytes: u64,
    /// Total disk space in bytes
    pub disk_total_bytes: u64,
    /// Used disk space in bytes
    pub disk_used_bytes: u64,
    /// Available disk space in bytes
    pub disk_available_bytes: u64,
    /// Network received bytes
    pub network_rx_bytes: u64,
    /// Network transmitted bytes
    pub network_tx_bytes: u64,
    /// Timestamp (Unix epoch seconds)
    pub timestamp: u64,
}

impl SystemMetrics {
    /// Create new metrics
    pub fn new(
        cpu_usage_percent: f32,
        memory_total_bytes: u64,
        memory_used_bytes: u64,
        memory_available_bytes: u64,
        swap_total_bytes: u64,
        swap_used_bytes: u64,
        disk_total_bytes: u64,
        disk_used_bytes: u64,
        disk_available_bytes: u64,
        network_rx_bytes: u64,
        network_tx_bytes: u64,
        timestamp: u64,
    ) -> Self {
        Self {
            cpu_usage_percent,
            memory_total_bytes,
            memory_used_bytes,
            memory_available_bytes,
            swap_total_bytes,
            swap_used_bytes,
            disk_total_bytes,
            disk_used_bytes,
            disk_available_bytes,
            network_rx_bytes,
            network_tx_bytes,
            timestamp,
        }
    }

    /// Get memory usage percentage
    #[must_use]
    pub fn memory_usage_percent(&self) -> f32 {
        if self.memory_total_bytes == 0 {
            return 0.0;
        }
        (self.memory_used_bytes as f32 / self.memory_total_bytes as f32) * 100.0
    }

    /// Get disk usage percentage
    #[must_use]
    pub fn disk_usage_percent(&self) -> f32 {
        if self.disk_total_bytes == 0 {
            return 0.0;
        }
        (self.disk_used_bytes as f32 / self.disk_total_bytes as f32) * 100.0
    }
}
