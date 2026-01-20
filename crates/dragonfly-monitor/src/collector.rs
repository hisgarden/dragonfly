//! System metrics collection

use crate::metrics::SystemMetrics;
use dragonfly_core::error::Result;
use sysinfo::System;

/// Get disk usage for root filesystem (returns (total_bytes, used_bytes))
#[cfg(target_os = "macos")]
fn get_disk_usage(_path: &str) -> Option<(u64, u64)> {
    use std::ffi::CString;
    use std::mem;

    unsafe {
        let mut stat: libc::statfs = mem::zeroed();
        let c_path = CString::new("/").ok()?;

        if libc::statfs(c_path.as_ptr(), &mut stat) == 0 {
            let total = (stat.f_blocks as u64) * (stat.f_bsize as u64);
            let free = (stat.f_bavail as u64) * (stat.f_bsize as u64);
            let used = total.saturating_sub(free);
            Some((total, used))
        } else {
            None
        }
    }
}

#[cfg(not(target_os = "macos"))]
fn get_disk_usage(_path: &str) -> Option<(u64, u64)> {
    // Fallback for non-macOS: return None to use placeholder
    None
}

/// Collects system metrics
#[derive(Debug)]
pub struct MetricsCollector {
    system: System,
}

impl MetricsCollector {
    /// Create a new metrics collector
    pub fn new() -> Self {
        let mut system = System::new_all();
        system.refresh_all();
        Self { system }
    }

    /// Collect current system metrics
    pub async fn collect(&mut self) -> Result<SystemMetrics> {
        self.system.refresh_all();

        let cpu_usage = self.system.global_cpu_info().cpu_usage();
        let total_memory = self.system.total_memory();
        let used_memory = self.system.used_memory();
        let total_swap = self.system.total_swap();
        let used_swap = self.system.used_swap();

        // Get disk usage for root filesystem
        let (disk_total, disk_used) = get_disk_usage("/").unwrap_or((0, 0));

        Ok(SystemMetrics {
            cpu_usage_percent: cpu_usage,
            memory_total_bytes: total_memory,
            memory_used_bytes: used_memory,
            memory_available_bytes: total_memory.saturating_sub(used_memory),
            swap_total_bytes: total_swap,
            swap_used_bytes: used_swap,
            disk_total_bytes: disk_total,
            disk_used_bytes: disk_used,
            disk_available_bytes: disk_total.saturating_sub(disk_used),
            network_rx_bytes: 0, // Would need network monitoring
            network_tx_bytes: 0,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        })
    }
}

impl Default for MetricsCollector {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn should_collect_cpu_metrics() {
        let mut collector = MetricsCollector::new();
        let metrics = collector.collect().await.unwrap();

        // CPU usage should be between 0 and 100
        assert!(metrics.cpu_usage_percent >= 0.0);
        assert!(metrics.cpu_usage_percent <= 100.0);
    }

    #[tokio::test]
    async fn should_collect_memory_metrics() {
        let mut collector = MetricsCollector::new();
        let metrics = collector.collect().await.unwrap();

        // Memory values should be valid
        assert!(metrics.memory_total_bytes > 0);
        assert!(metrics.memory_used_bytes <= metrics.memory_total_bytes);
        assert_eq!(
            metrics.memory_available_bytes,
            metrics
                .memory_total_bytes
                .saturating_sub(metrics.memory_used_bytes)
        );
    }

    #[tokio::test]
    async fn should_collect_swap_metrics() {
        let mut collector = MetricsCollector::new();
        let metrics = collector.collect().await.unwrap();

        // Swap used should not exceed total
        assert!(metrics.swap_used_bytes <= metrics.swap_total_bytes);
    }

    #[tokio::test]
    async fn should_collect_disk_metrics() {
        let mut collector = MetricsCollector::new();
        let metrics = collector.collect().await.unwrap();

        // Disk values should be valid
        assert!(metrics.disk_total_bytes > 0);
        assert!(metrics.disk_used_bytes <= metrics.disk_total_bytes);
        assert_eq!(
            metrics.disk_available_bytes,
            metrics
                .disk_total_bytes
                .saturating_sub(metrics.disk_used_bytes)
        );
    }

    #[tokio::test]
    async fn should_include_timestamp() {
        let mut collector = MetricsCollector::new();
        let metrics = collector.collect().await.unwrap();

        // Timestamp should be recent (within last minute)
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        assert!(metrics.timestamp <= now);
        assert!(metrics.timestamp > now.saturating_sub(60));
    }

    #[tokio::test]
    async fn should_collect_multiple_times() {
        let mut collector = MetricsCollector::new();

        let metrics1 = collector.collect().await.unwrap();
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        let metrics2 = collector.collect().await.unwrap();

        // Timestamps should differ
        assert!(metrics2.timestamp >= metrics1.timestamp);
    }

    #[test]
    fn test_collector_creation() {
        let collector = MetricsCollector::new();
        assert!(!collector.system.cpus().is_empty());
    }
}
