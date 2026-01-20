//! Domain entities - Objects with identity

use serde::{Deserialize, Serialize};

/// Health status enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HealthStatus {
    /// System is healthy
    Healthy,
    /// System has warnings
    Warning,
    /// System is in critical state
    Critical,
}

/// File entity (MVP stub)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileEntity {
    /// File path
    pub path: String,
    /// File size in bytes
    pub size: u64,
}

/// Directory entity (MVP stub)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DirectoryEntity {
    /// Directory path
    pub path: String,
}

/// System snapshot (MVP stub)
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct SystemSnapshot {
    /// Timestamp of the snapshot
    pub timestamp: u64,
}
