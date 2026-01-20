//! Domain events - Important business occurrences

use serde::{Deserialize, Serialize};

/// Domain event (MVP stub)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DomainEvent {
    /// A file was analyzed
    FileAnalyzed {
        /// Path of the analyzed file
        path: String,
    },
    /// Duplicate files were found
    DuplicateFound {
        /// Path of the first file
        path1: String,
        /// Path of the second file
        path2: String,
    },
}
