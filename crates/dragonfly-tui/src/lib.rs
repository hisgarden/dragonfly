//! # DragonFly TUI
//!
//! Terminal User Interface for DragonFly with retro "80s defrag" style animation.
//!
//! This crate provides a full-screen terminal experience for disk scanning
//! and cleanup operations.

#![warn(missing_docs)]

/// Defrag animation module
pub mod animation;

/// Main TUI application
pub mod app;

// Re-export main entry point
pub use app::run_app;
