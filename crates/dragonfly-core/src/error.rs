//! Error types for domain operations
//!
//! Defines all possible errors that can occur in the domain layer.
//! Errors are strongly typed and provide context for debugging.

use thiserror::Error;

/// Result type alias for domain operations
pub type Result<T> = std::result::Result<T, Error>;

/// Domain error types
#[derive(Error, Debug)]
pub enum Error {
    /// File system related errors
    #[error("File system error: {0}")]
    FileSystem(String),

    /// Invalid input or parameter
    #[error("Invalid input: {0}")]
    InvalidInput(String),

    /// Resource not found
    #[error("Resource not found: {0}")]
    NotFound(String),

    /// Permission denied
    #[error("Permission denied: {0}")]
    PermissionDenied(String),

    /// Operation not supported
    #[error("Operation not supported: {0}")]
    NotSupported(String),

    /// Internal error
    #[error("Internal error: {0}")]
    Internal(String),

    /// IO error wrapper
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let err = Error::InvalidInput("test".to_string());
        assert!(err.to_string().contains("Invalid input"));
    }
}
