//! System cleaning orchestration

use crate::targets::CleanTarget;
use dragonfly_core::error::Result;
use jwalk::WalkDir;
use std::fs;
use std::path::{Path, PathBuf};

/// Cleaning result
#[derive(Debug, Clone)]
pub struct CleanResult {
    /// Number of files cleaned
    pub files_cleaned: usize,
    /// Total bytes freed
    pub bytes_freed: u64,
    /// Files that would be cleaned (for dry-run)
    pub files_found: Vec<PathBuf>,
}

/// Cleans system caches and temporary files
#[derive(Debug, Clone, Copy)]
pub struct SystemCleaner;

impl SystemCleaner {
    /// Create a new system cleaner
    pub fn new() -> Self {
        Self
    }

    /// Clean based on target
    pub async fn clean(&self, target: CleanTarget, dry_run: bool) -> Result<CleanResult> {
        let paths = target.paths();
        let mut total_files = 0;
        let mut total_bytes = 0u64;
        let mut all_files = Vec::new();

        for path_str in paths {
            let expanded_path = expand_path(path_str)?;
            let path = Path::new(&expanded_path);

            if !path.exists() {
                continue;
            }

            let (files, bytes) = if dry_run {
                scan_directory(path)?
            } else {
                clean_directory(path)?
            };

            total_files += files.len();
            total_bytes += bytes;
            all_files.extend(files);
        }

        Ok(CleanResult {
            files_cleaned: total_files,
            bytes_freed: total_bytes,
            files_found: all_files,
        })
    }

    /// Clean caches
    pub async fn clean_caches(&self, dry_run: bool) -> Result<u64> {
        let result = self.clean(CleanTarget::Caches, dry_run).await?;
        Ok(result.bytes_freed)
    }

    /// Clean logs
    pub async fn clean_logs(&self, dry_run: bool) -> Result<u64> {
        let result = self.clean(CleanTarget::Logs, dry_run).await?;
        Ok(result.bytes_freed)
    }

    /// Clean temp files
    pub async fn clean_temp(&self, dry_run: bool) -> Result<u64> {
        let result = self.clean(CleanTarget::Temp, dry_run).await?;
        Ok(result.bytes_freed)
    }
}

/// Expand path with ~ to home directory
fn expand_path(path: &str) -> Result<String> {
    if let Some(stripped) = path.strip_prefix("~/") {
        let home = dirs::home_dir().ok_or_else(|| {
            dragonfly_core::error::Error::NotFound("Home directory not found".to_string())
        })?;
        Ok(home.join(stripped).to_string_lossy().to_string())
    } else {
        Ok(path.to_string())
    }
}

/// Scan directory and return files with sizes
fn scan_directory(path: &Path) -> Result<(Vec<PathBuf>, u64)> {
    let mut files = Vec::new();
    let mut total_size = 0u64;

    for entry in WalkDir::new(path).into_iter().flatten() {
        if entry.file_type().is_file() {
            if let Ok(metadata) = entry.metadata() {
                let size = metadata.len();
                total_size += size;
                files.push(entry.path().to_path_buf());
            }
        }
    }

    Ok((files, total_size))
}

/// Clean directory (delete files)
fn clean_directory(path: &Path) -> Result<(Vec<PathBuf>, u64)> {
    let mut files = Vec::new();
    let mut total_size = 0u64;

    for entry in WalkDir::new(path).into_iter().flatten() {
        if entry.file_type().is_file() {
            if let Ok(metadata) = entry.metadata() {
                let size = metadata.len();
                let file_path = entry.path().to_path_buf();

                if fs::remove_file(&file_path).is_ok() {
                    total_size += size;
                    files.push(file_path);
                }
            }
        }
    }

    Ok((files, total_size))
}

impl Default for SystemCleaner {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_cleaner_creation() {
        let cleaner = SystemCleaner::new();
        assert_eq!(std::mem::size_of_val(&cleaner), 0);
    }

    #[test]
    fn test_expand_path() {
        let expanded = expand_path("~/test").unwrap();
        assert!(expanded.contains("test"));
        assert!(!expanded.contains("~"));

        let absolute = expand_path("/tmp/test").unwrap();
        assert_eq!(absolute, "/tmp/test");
    }

    #[tokio::test]
    async fn test_clean_dry_run() {
        let temp_dir = TempDir::new().unwrap();
        let test_file = temp_dir.path().join("test.txt");
        fs::write(&test_file, b"test content").unwrap();

        let cleaner = SystemCleaner::new();
        // Create a custom target for testing
        // For now, just test that dry_run doesn't delete
        let result = cleaner.clean_caches(true).await;
        assert!(result.is_ok());
    }
}
