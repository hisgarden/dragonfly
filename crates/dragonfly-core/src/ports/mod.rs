//! Port traits (interfaces) for Hexagonal Architecture
//!
//! Ports define the boundaries of the application. They represent contracts
//! that adapters must implement to interact with the domain layer.
//!
//! - **Driving Ports** (Primary): Called by external actors to drive the application
//! - **Driven Ports** (Secondary): Called by the application to interact with external systems

use crate::domain::entities::{DirectoryEntity, FileEntity, SystemSnapshot};
use crate::domain::value_objects::FilePath;
use crate::error::Result;
use async_trait::async_trait;
use std::collections::HashMap;

/// Repository for file operations (Driven Port)
///
/// This port defines how the domain layer interacts with the file system.
/// Adapters implement this to provide concrete file system access.
#[async_trait]
pub trait FileRepository: Send + Sync {
    /// Scan a directory and return all files
    async fn scan_directory(&self, path: &FilePath) -> Result<Vec<FileEntity>>;

    /// Get file metadata
    async fn get_file_metadata(&self, path: &FilePath) -> Result<FileEntity>;

    /// Delete a file
    async fn delete_file(&self, path: &FilePath) -> Result<()>;

    /// Move a file to a new location
    async fn move_file(&self, from: &FilePath, to: &FilePath) -> Result<()>;

    /// Calculate hash of a file (for duplicate detection)
    async fn calculate_hash(&self, path: &FilePath) -> Result<String>;

    /// Check if file exists
    async fn exists(&self, path: &FilePath) -> Result<bool>;

    /// Get file size
    async fn get_size(&self, path: &FilePath) -> Result<u64>;
}

/// Repository for directory operations (Driven Port)
#[async_trait]
pub trait DirectoryRepository: Send + Sync {
    /// Analyze a directory and return summary
    async fn analyze_directory(&self, path: &FilePath) -> Result<DirectoryEntity>;

    /// Get directory size recursively
    async fn get_directory_size(&self, path: &FilePath) -> Result<u64>;

    /// Delete a directory and all contents
    async fn delete_directory(&self, path: &FilePath) -> Result<()>;

    /// List all directories at path
    async fn list_directories(&self, path: &FilePath) -> Result<Vec<FilePath>>;
}

/// Repository for system information (Driven Port)
#[async_trait]
pub trait SystemRepository: Send + Sync {
    /// Get current system snapshot
    async fn get_system_snapshot(&self) -> Result<SystemSnapshot>;

    /// Get system uptime in seconds
    async fn get_uptime(&self) -> Result<u64>;

    /// Get available disk space in bytes
    async fn get_available_disk_space(&self) -> Result<u64>;

    /// Get total disk space in bytes
    async fn get_total_disk_space(&self) -> Result<u64>;

    /// Get CPU usage percentage (0-100)
    async fn get_cpu_usage(&self) -> Result<f32>;

    /// Get memory usage percentage (0-100)
    async fn get_memory_usage(&self) -> Result<f32>;
}

/// Event publisher for domain events (Driven Port)
#[async_trait]
pub trait EventPublisher: Send + Sync {
    /// Publish a domain event
    async fn publish_event(&self, event_type: &str, event_data: serde_json::Value) -> Result<()>;
}

/// Logger interface (Driven Port)
pub trait Logger: Send + Sync {
    /// Log an info message
    fn info(&self, message: &str);

    /// Log a warning message
    fn warn(&self, message: &str);

    /// Log an error message
    fn error(&self, message: &str);

    /// Log a debug message
    fn debug(&self, message: &str);

    /// Log with structured context
    fn with_context(&self, context: HashMap<String, String>, message: &str);
}

/// Notifier for user-facing messages (Driven Port)
#[async_trait]
pub trait Notifier: Send + Sync {
    /// Send a notification to the user
    async fn notify(&self, title: &str, message: &str) -> Result<()>;

    /// Send a progress update
    async fn progress(&self, current: usize, total: usize, message: &str) -> Result<()>;

    /// Ask user for confirmation
    async fn confirm(&self, message: &str) -> Result<bool>;
}

/// Cache service for storing analysis results (Driven Port)
#[async_trait]
pub trait CacheService: Send + Sync {
    /// Store a value in cache
    async fn set(&self, key: &str, value: serde_json::Value) -> Result<()>;

    /// Get a value from cache
    async fn get(&self, key: &str) -> Result<Option<serde_json::Value>>;

    /// Remove a value from cache
    async fn delete(&self, key: &str) -> Result<()>;

    /// Clear entire cache
    async fn clear(&self) -> Result<()>;

    /// Check if key exists
    async fn exists(&self, key: &str) -> Result<bool>;
}

/// Configuration service (Driven Port)
pub trait ConfigService: Send + Sync {
    /// Get a configuration value
    ///
    /// # Errors
    ///
    /// Returns an error if the key doesn't exist or the value can't be converted to a string
    fn get_string(&self, key: &str) -> Result<String>;

    /// Get a boolean configuration value
    ///
    /// # Errors
    ///
    /// Returns an error if the key doesn't exist or the value can't be converted to a boolean
    fn get_bool(&self, key: &str) -> Result<bool>;

    /// Get an integer configuration value
    ///
    /// # Errors
    ///
    /// Returns an error if the key doesn't exist or the value can't be converted to an integer
    fn get_int(&self, key: &str) -> Result<i64>;

    /// Get configuration as JSON
    ///
    /// # Errors
    ///
    /// Returns an error if the key doesn't exist or the value can't be parsed as JSON
    fn get_json(&self, key: &str) -> Result<serde_json::Value>;
}

/// Use case for disk analysis (Driving Port)
#[async_trait]
pub trait AnalyzeDiskUseCase: Send + Sync {
    /// Analyze disk usage in a directory
    async fn analyze(&self, path: &FilePath) -> Result<DirectoryEntity>;

    /// Find large files in a directory
    async fn find_large_files(&self, path: &FilePath, min_size: u64) -> Result<Vec<FileEntity>>;
}

/// Use case for duplicate detection (Driving Port)
#[async_trait]
pub trait FindDuplicatesUseCase: Send + Sync {
    /// Find duplicate files in a directory
    async fn find_duplicates(&self, path: &FilePath, min_size: u64)
        -> Result<Vec<Vec<FileEntity>>>;

    /// Calculate potential space savings from duplicates
    async fn calculate_savings(&self, duplicates: &[Vec<FileEntity>]) -> Result<u64>;
}

/// Use case for system health check (Driving Port)
#[async_trait]
pub trait HealthCheckUseCase: Send + Sync {
    /// Check overall system health
    async fn check_health(&self) -> Result<SystemSnapshot>;

    /// Get health recommendations
    async fn get_recommendations(&self, snapshot: &SystemSnapshot) -> Result<Vec<String>>;
}

/// Use case for cleaning (Driving Port)
#[async_trait]
pub trait CleanUseCase: Send + Sync {
    /// Clean caches
    async fn clean_caches(&self, dry_run: bool) -> Result<u64>;

    /// Clean logs
    async fn clean_logs(&self, days: u32, dry_run: bool) -> Result<u64>;

    /// Clean temporary files
    async fn clean_temp(&self, dry_run: bool) -> Result<u64>;

    /// Clean old files
    async fn clean_old_files(&self, days: u32, min_size: u64, dry_run: bool) -> Result<u64>;
}

/// Monitor use case for real-time metrics (Driving Port)
#[async_trait]
pub trait MonitorUseCase: Send + Sync {
    /// Start continuous monitoring
    async fn start_monitoring(&self, interval_secs: u64) -> Result<()>;

    /// Get current metrics
    async fn get_metrics(&self) -> Result<SystemSnapshot>;

    /// Stop monitoring
    async fn stop_monitoring(&self) -> Result<()>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_port_traits_exist() {
        // This test verifies that all port traits are properly defined
        // Actual implementation tests will be in adapter tests
        assert!(std::any::type_name::<dyn FileRepository>().contains("FileRepository"));
    }
}
