//! Disk analysis orchestration

use dragonfly_core::domain::entities::FileEntity;
use dragonfly_core::domain::value_objects::FilePath;
use dragonfly_core::error::Result;
use jwalk::WalkDir;
use rayon::prelude::*;
use std::path::Path;

/// Disk analyzer orchestrates disk analysis operations
#[derive(Debug, Clone, Copy)]
pub struct DiskAnalyzer;

/// Analysis result for a directory
#[derive(Debug, Clone)]
pub struct AnalysisResult {
    /// Total size in bytes
    pub total_size: u64,
    /// Files found
    pub files: Vec<FileEntity>,
}

impl DiskAnalyzer {
    /// Create a new disk analyzer
    pub fn new() -> Self {
        Self
    }

    /// Analyze a directory and return file sizes
    pub async fn analyze(&self, path: &FilePath) -> Result<AnalysisResult> {
        let path_str = path.as_str();
        let base_path = Path::new(path_str);

        if !base_path.exists() {
            return Err(dragonfly_core::error::Error::NotFound(format!(
                "Path does not exist: {}",
                path_str
            )));
        }

        let files: Vec<FileEntity> = WalkDir::new(base_path)
            .into_iter()
            .par_bridge()
            .filter_map(|entry| {
                let entry = entry.ok()?;
                let metadata = entry.metadata().ok()?;

                if metadata.is_file() {
                    let size = metadata.len();
                    let path_str = entry.path().to_string_lossy().to_string();
                    Some(FileEntity {
                        path: path_str,
                        size,
                    })
                } else {
                    None
                }
            })
            .collect();

        let total_size: u64 = files.iter().map(|f| f.size).sum();

        Ok(AnalysisResult { total_size, files })
    }

    /// Find large files above a minimum size
    pub async fn find_large_files(
        &self,
        path: &FilePath,
        min_size_bytes: u64,
    ) -> Result<Vec<FileEntity>> {
        let result = self.analyze(path).await?;
        let large_files: Vec<FileEntity> = result
            .files
            .into_iter()
            .filter(|f| f.size >= min_size_bytes)
            .collect();
        Ok(large_files)
    }
}

impl Default for DiskAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_analyzer_creation() {
        let analyzer = DiskAnalyzer::new();
        assert_eq!(std::mem::size_of_val(&analyzer), 0);
    }
}
