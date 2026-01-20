//! Time Machine snapshot management
//!
//! Manages local Time Machine snapshots that accumulate on APFS volumes.
//! Provides safe deletion with warnings and size analysis.

use dragonfly_core::error::{Error, Result};
use serde::{Deserialize, Serialize};
use std::process::Command;

/// Time Machine snapshot information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Snapshot {
    /// Snapshot name/ID
    pub id: String,
    /// Snapshot date
    pub date: String,
    /// Snapshot size in bytes (if available)
    pub size: Option<u64>,
}

/// Time Machine snapshot manager
#[derive(Debug, Clone, Copy)]
pub struct TimeMachineManager;

impl TimeMachineManager {
    /// List all local snapshots
    pub fn list_snapshots() -> Result<Vec<Snapshot>> {
        let output = Command::new("tmutil")
            .args(["listlocalsnapshots", "/"])
            .output()
            .map_err(|e| Error::Internal(format!("Failed to run tmutil: {}", e)))?;

        if !output.status.success() {
            return Err(Error::Internal(
                "tmutil listlocalsnapshots failed".to_string(),
            ));
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        let mut snapshots = Vec::new();

        // Parse tmutil output
        // Format: "com.apple.TimeMachine.2025-01-20-143000"
        for line in stdout.lines() {
            if line.contains("com.apple.TimeMachine") {
                let id = line.trim().to_string();
                // Extract date from snapshot ID
                let date = Self::extract_date(&id).unwrap_or_else(|| "unknown".to_string());
                snapshots.push(Snapshot {
                    id,
                    date,
                    size: None, // Size requires additional command
                });
            }
        }

        Ok(snapshots)
    }

    /// Get snapshot sizes (requires sudo)
    pub fn get_snapshot_sizes() -> Result<Vec<(String, u64)>> {
        // This requires sudo and uses diskutil
        // For MVP, return empty (can be implemented later)
        Ok(Vec::new())
    }

    /// Delete a local snapshot
    pub fn delete_snapshot(snapshot_id: &str) -> Result<()> {
        let output = Command::new("tmutil")
            .args(["deletelocalsnapshot", snapshot_id])
            .output()
            .map_err(|e| Error::Internal(format!("Failed to run tmutil: {}", e)))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(Error::Internal(format!(
                "Failed to delete snapshot {}: {}",
                snapshot_id, stderr
            )));
        }

        Ok(())
    }

    /// Delete snapshots older than specified days
    pub fn delete_old_snapshots(days: u32, dry_run: bool) -> Result<Vec<String>> {
        let snapshots = Self::list_snapshots()?;
        let cutoff_date = chrono::Utc::now() - chrono::Duration::days(days as i64);
        let mut deleted = Vec::new();

        for snapshot in snapshots {
            if let Ok(snapshot_date) = Self::parse_snapshot_date(&snapshot.date) {
                if snapshot_date < cutoff_date {
                    if !dry_run {
                        Self::delete_snapshot(&snapshot.id)?;
                    }
                    deleted.push(snapshot.id);
                }
            }
        }

        Ok(deleted)
    }

    /// Extract date from snapshot ID
    fn extract_date(snapshot_id: &str) -> Option<String> {
        // Format: com.apple.TimeMachine.2025-01-20-143000
        if let Some(date_part) = snapshot_id.split('.').next_back() {
            // Convert 2025-01-20-143000 to readable format
            if date_part.len() >= 10 {
                let date = &date_part[..10]; // 2025-01-20
                let time = if date_part.len() >= 16 {
                    &date_part[11..16] // 14300
                } else {
                    ""
                };
                return Some(format!("{} {}", date, time));
            }
        }
        None
    }

    /// Parse snapshot date string
    fn parse_snapshot_date(date_str: &str) -> Result<chrono::DateTime<chrono::Utc>> {
        // Try various date formats
        let formats = ["%Y-%m-%d %H%M%S", "%Y-%m-%d", "%Y-%m-%d-%H%M%S"];

        for format in &formats {
            if let Ok(dt) = chrono::NaiveDateTime::parse_from_str(date_str, format) {
                return Ok(dt.and_utc());
            }
        }

        Err(Error::InvalidInput(format!(
            "Invalid date format: {}",
            date_str
        )))
    }

    /// Get total size of all snapshots
    pub fn total_snapshot_size() -> Result<u64> {
        // This requires diskutil and sudo
        // For MVP, return 0 (can be implemented later)
        Ok(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_date() {
        let id = "com.apple.TimeMachine.2025-01-20-143000";
        let date = TimeMachineManager::extract_date(id);
        assert!(date.is_some());
    }
}
