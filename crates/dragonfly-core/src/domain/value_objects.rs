//! Value objects - Immutable domain concepts defined by their attributes

use serde::{Deserialize, Serialize};
use std::fmt;

/// File size in bytes with type safety
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct FileSize(pub u64);

impl FileSize {
    /// Create a new `FileSize` from bytes
    #[must_use]
    pub fn new(bytes: u64) -> Self {
        Self(bytes)
    }

    /// Get the size in bytes
    #[must_use]
    pub fn bytes(&self) -> u64 {
        self.0
    }
}

impl fmt::Display for FileSize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} bytes", self.0)
    }
}

/// File path value object
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FilePath(pub String);

impl FilePath {
    /// Create a new `FilePath`
    #[must_use]
    pub fn new(path: String) -> Self {
        Self(path)
    }

    /// Get the path as a string slice
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for FilePath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Percentage value object (0-100)
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Percentage(pub f32);

impl Percentage {
    /// Create a new Percentage, clamped between 0 and 100
    #[must_use]
    pub fn new(value: f32) -> Self {
        Self(value.clamp(0.0, 100.0))
    }

    /// Get the percentage value
    #[must_use]
    pub fn value(&self) -> f32 {
        self.0
    }
}

impl fmt::Display for Percentage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.1}%", self.0)
    }
}
