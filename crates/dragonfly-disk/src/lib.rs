//! Disk Analysis Module
//!
//! This module provides disk usage analysis capabilities for DragonFly.
//! It scans directories, calculates sizes, and identifies space usage patterns.

#![warn(
    missing_docs,
    missing_debug_implementations,
    missing_copy_implementations
)]

pub mod analyzer;
pub mod strategies;

pub use analyzer::DiskAnalyzer;
pub use strategies::AnalysisStrategy;

/// Module version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_module_loads() {
        assert!(!VERSION.is_empty());
    }
}
