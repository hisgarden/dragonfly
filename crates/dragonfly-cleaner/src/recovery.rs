//! Recovery system for safe cleanup with restore capability
//!
//! This module implements a recovery-first approach where files are archived
//! before deletion, allowing users to restore them if needed.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Recovery manifest entry for a single cleaned item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecoveryItem {
    /// Original path before cleanup
    pub original_path: PathBuf,
    /// Path in archive
    pub archive_path: PathBuf,
    /// Size in bytes
    pub size: u64,
    /// SHA-256 checksum for verification
    pub checksum: String,
    /// Category (git, cache, xcode, etc.)
    pub category: String,
    /// Source application/tool
    pub source: String,
    /// Whether this can be regenerated
    pub can_regenerate: bool,
}

/// Recovery manifest for a cleanup operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecoveryManifest {
    /// Unique ID for this recovery (timestamp-based)
    pub id: String,
    /// When cleanup occurred
    pub timestamp: DateTime<Utc>,
    /// Total size of all items
    pub total_size: u64,
    /// List of cleaned items
    pub items: Vec<RecoveryItem>,
    /// Retention expiration date
    pub retention_until: DateTime<Utc>,
}

/// Recovery manager handles archiving and restoring
#[derive(Debug)]
pub struct RecoveryManager {
    recovery_dir: PathBuf,
}

impl RecoveryManager {
    /// Create a new recovery manager
    pub fn new(recovery_dir: PathBuf) -> Self {
        Self { recovery_dir }
    }

    /// Get default recovery directory
    pub fn default_dir() -> PathBuf {
        dirs::home_dir()
            .unwrap_or_else(|| PathBuf::from("~"))
            .join(".dragonfly")
            .join("recovery")
    }

    /// Initialize recovery directory structure
    pub fn initialize(&self) -> std::io::Result<()> {
        let manifests_dir = self.recovery_dir.join("manifests");
        let archives_dir = self.recovery_dir.join("archives");
        let index_file = self.recovery_dir.join("index.json");

        std::fs::create_dir_all(&manifests_dir)?;
        std::fs::create_dir_all(&archives_dir)?;

        // Create index if it doesn't exist
        if !index_file.exists() {
            let index = RecoveryIndex {
                recoveries: Vec::new(),
            };
            std::fs::write(&index_file, serde_json::to_string_pretty(&index)?)?;
        }

        Ok(())
    }

    /// Create a new recovery manifest
    pub fn create_manifest(&self, retention_days: u32) -> RecoveryManifest {
        let id = Utc::now().format("%Y-%m-%d_%H-%M-%S").to_string();
        let timestamp = Utc::now();
        let retention_until = timestamp + chrono::Duration::days(retention_days as i64);

        RecoveryManifest {
            id,
            timestamp,
            total_size: 0,
            items: Vec::new(),
            retention_until,
        }
    }

    /// Save manifest to disk
    pub fn save_manifest(&self, manifest: &RecoveryManifest) -> std::io::Result<()> {
        let manifest_file = self
            .recovery_dir
            .join("manifests")
            .join(format!("{}.json", manifest.id));

        let json = serde_json::to_string_pretty(manifest)?;
        std::fs::write(manifest_file, json)?;

        // Update index
        self.update_index(manifest)?;

        Ok(())
    }

    /// Load manifest by ID
    pub fn load_manifest(&self, id: &str) -> std::io::Result<RecoveryManifest> {
        let manifest_file = self
            .recovery_dir
            .join("manifests")
            .join(format!("{}.json", id));
        let content = std::fs::read_to_string(manifest_file)?;
        let manifest: RecoveryManifest = serde_json::from_str(&content)?;
        Ok(manifest)
    }

    /// List all available recoveries
    pub fn list_recoveries(&self) -> std::io::Result<Vec<RecoveryManifest>> {
        let index_file = self.recovery_dir.join("index.json");
        if !index_file.exists() {
            return Ok(Vec::new());
        }

        let content = std::fs::read_to_string(index_file)?;
        let index: RecoveryIndex = serde_json::from_str(&content)?;

        let mut recoveries = Vec::new();
        for id in index.recoveries {
            if let Ok(manifest) = self.load_manifest(&id) {
                recoveries.push(manifest);
            }
        }

        // Sort by timestamp (newest first)
        recoveries.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));

        Ok(recoveries)
    }

    /// Get archive directory for a recovery
    pub fn archive_dir(&self, recovery_id: &str) -> PathBuf {
        self.recovery_dir.join("archives").join(recovery_id)
    }

    /// Update recovery index
    fn update_index(&self, manifest: &RecoveryManifest) -> std::io::Result<()> {
        let index_file = self.recovery_dir.join("index.json");
        let mut index = if index_file.exists() {
            let content = std::fs::read_to_string(&index_file)?;
            serde_json::from_str(&content).unwrap_or_else(|_| RecoveryIndex {
                recoveries: Vec::new(),
            })
        } else {
            RecoveryIndex {
                recoveries: Vec::new(),
            }
        };

        if !index.recoveries.contains(&manifest.id) {
            index.recoveries.push(manifest.id.clone());
        }

        std::fs::write(index_file, serde_json::to_string_pretty(&index)?)?;
        Ok(())
    }

    /// Clean up expired recoveries
    pub fn cleanup_expired(&self) -> std::io::Result<Vec<String>> {
        let recoveries = self.list_recoveries()?;
        let now = Utc::now();
        let mut cleaned = Vec::new();

        for manifest in recoveries {
            if manifest.retention_until < now {
                let archive_dir = self.archive_dir(&manifest.id);
                if archive_dir.exists() {
                    std::fs::remove_dir_all(&archive_dir)?;
                }

                let manifest_file = self
                    .recovery_dir
                    .join("manifests")
                    .join(format!("{}.json", manifest.id));
                if manifest_file.exists() {
                    std::fs::remove_file(manifest_file)?;
                }

                cleaned.push(manifest.id);
            }
        }

        // Update index
        if !cleaned.is_empty() {
            let index_file = self.recovery_dir.join("index.json");
            let content = std::fs::read_to_string(&index_file)?;
            let mut index: RecoveryIndex = serde_json::from_str(&content)?;
            index.recoveries.retain(|id| !cleaned.contains(id));
            std::fs::write(index_file, serde_json::to_string_pretty(&index)?)?;
        }

        Ok(cleaned)
    }
}

/// Recovery index file structure
#[derive(Debug, Serialize, Deserialize)]
struct RecoveryIndex {
    recoveries: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_recovery_manager_creation() {
        let temp_dir = TempDir::new().unwrap();
        let manager = RecoveryManager::new(temp_dir.path().to_path_buf());
        assert!(manager.initialize().is_ok());
    }

    #[test]
    fn test_manifest_creation() {
        let temp_dir = TempDir::new().unwrap();
        let manager = RecoveryManager::new(temp_dir.path().to_path_buf());
        manager.initialize().unwrap();

        let manifest = manager.create_manifest(30);
        assert!(!manifest.id.is_empty());
        assert!(manifest.items.is_empty());
        assert_eq!(manifest.total_size, 0);
    }
}
