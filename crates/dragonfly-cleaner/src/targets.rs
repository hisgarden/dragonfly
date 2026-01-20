//! Cleaning targets

use serde::{Deserialize, Serialize};

/// Targets that can be cleaned
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CleanTarget {
    /// System caches
    Caches,
    /// Log files
    Logs,
    /// Temporary files
    Temp,
    /// All targets
    All,
}

impl CleanTarget {
    /// Get list of paths for this target
    pub fn paths(&self) -> Vec<&'static str> {
        match self {
            Self::Caches => vec!["~/Library/Caches", "/Library/Caches"],
            Self::Logs => vec!["~/Library/Logs", "/var/log"],
            Self::Temp => vec!["/tmp", "/var/tmp"],
            Self::All => vec![
                "~/Library/Caches",
                "/Library/Caches",
                "~/Library/Logs",
                "/var/log",
                "/tmp",
                "/var/tmp",
            ],
        }
    }
}
