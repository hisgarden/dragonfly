//! Hash algorithm selection and utilities

use serde::{Deserialize, Serialize};

/// Available hash algorithms
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum HashAlgorithm {
    /// Blake3 - Fast cryptographic hash
    #[default]
    Blake3,
    /// xxHash - Very fast non-cryptographic hash
    XxHash3,
}

impl std::fmt::Display for HashAlgorithm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Blake3 => write!(f, "BLAKE3"),
            Self::XxHash3 => write!(f, "xxHash3"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_algorithm() {
        assert_eq!(HashAlgorithm::default(), HashAlgorithm::Blake3);
    }
}
