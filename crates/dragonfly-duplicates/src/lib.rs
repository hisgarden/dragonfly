//! Duplicate File Finder Module
//!
//! This module provides duplicate file detection using fast hashing.
//! Supports Blake3 for cryptographic hashing and xxHash for speed.

#![warn(
    missing_docs,
    missing_debug_implementations,
    missing_copy_implementations
)]

pub mod detector;
pub mod hasher;

pub use detector::DuplicateDetector;
pub use hasher::HashAlgorithm;

/// Module version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
