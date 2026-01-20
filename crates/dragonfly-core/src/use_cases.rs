//! Use cases (application business rules)
//!
//! Use cases orchestrate the flow of data to and from entities,
//! and direct those entities to use their business rules to achieve
//! the goals of the use case.

use crate::domain::value_objects::FilePath;
use crate::error::Result;
use async_trait::async_trait;

/// Use case for analyzing disk usage
#[async_trait]
pub trait AnalyzeDiskUseCase: Send + Sync {
    /// Analyze disk usage in a directory
    async fn analyze(&self, path: &FilePath) -> Result<()>;
}

/// Use case for finding duplicate files
#[async_trait]
pub trait FindDuplicatesUseCase: Send + Sync {
    /// Find duplicate files in a directory
    async fn find_duplicates(&self, path: &FilePath) -> Result<()>;
}

/// Use case for system health check
#[async_trait]
pub trait HealthCheckUseCase: Send + Sync {
    /// Perform a health check
    async fn check_health(&self) -> Result<()>;
}

/// Use case for cleaning caches
#[async_trait]
pub trait CleanUseCase: Send + Sync {
    /// Clean system caches
    async fn clean(&self, dry_run: bool) -> Result<u64>;
}

/// Use case for monitoring system
#[async_trait]
pub trait MonitorUseCase: Send + Sync {
    /// Monitor system metrics
    async fn monitor(&self, interval: u64) -> Result<()>;
}
