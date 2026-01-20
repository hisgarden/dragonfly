//! Disk analysis orchestration

use dragonfly_core::domain::value_objects::FilePath;
use dragonfly_core::error::Result;

/// Disk analyzer orchestrates disk analysis operations
#[derive(Debug, Clone, Copy)]
pub struct DiskAnalyzer;

impl DiskAnalyzer {
    /// Create a new disk analyzer
    pub fn new() -> Self {
        Self
    }

    /// Analyze a directory (MVP stub)
    pub async fn analyze(&self, _path: &FilePath) -> Result<()> {
        Ok(())
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
