//! Duplicate file detection orchestration

use dragonfly_core::domain::value_objects::FilePath;
use dragonfly_core::error::Result;

/// Duplicate detector orchestrates finding duplicate files
#[derive(Debug, Clone, Copy)]
pub struct DuplicateDetector;

impl DuplicateDetector {
    /// Create a new duplicate detector
    pub fn new() -> Self {
        Self
    }

    /// Find duplicates in a directory (MVP stub)
    pub async fn find_duplicates(&self, _path: &FilePath) -> Result<()> {
        Ok(())
    }
}

impl Default for DuplicateDetector {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detector_creation() {
        let detector = DuplicateDetector::new();
        assert_eq!(std::mem::size_of_val(&detector), 0);
    }
}
