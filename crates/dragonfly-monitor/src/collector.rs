//! System metrics collection

use dragonfly_core::error::Result;

/// Collects system metrics
#[derive(Debug, Clone, Copy)]
pub struct MetricsCollector;

impl MetricsCollector {
    /// Create a new metrics collector
    pub fn new() -> Self {
        Self
    }

    /// Collect current system metrics (MVP stub)
    pub async fn collect(&self) -> Result<()> {
        Ok(())
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

    #[test]
    fn test_collector_creation() {
        let collector = MetricsCollector::new();
        assert_eq!(std::mem::size_of_val(&collector), 0);
    }
}
