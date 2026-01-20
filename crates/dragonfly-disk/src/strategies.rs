//! Analysis strategies for different scan approaches

use serde::{Deserialize, Serialize};

/// Different strategies for disk analysis
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum AnalysisStrategy {
    /// Deep scan - visit every file
    #[default]
    Deep,
    /// Quick estimate - sample-based
    Quick,
    /// Incremental - use cached results
    Incremental,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strategy_default() {
        assert_eq!(AnalysisStrategy::default(), AnalysisStrategy::Deep);
    }
}
