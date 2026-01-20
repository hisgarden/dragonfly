//! System Monitoring Module
//!
//! This module provides real-time system metrics collection and monitoring.
//! Collects CPU, memory, disk, and network statistics.

#![warn(
    missing_docs,
    missing_debug_implementations,
    missing_copy_implementations
)]

pub mod collector;
pub mod metrics;

pub use collector::MetricsCollector;
pub use metrics::SystemMetrics;

/// Module version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
