//! DragonFly CLI Library
//!
//! This library provides command-line interface components
//! for the DragonFly macOS maintenance utility.

pub mod commands;
pub mod types;
pub mod ui;

pub use types::{DiskCommand, DuplicatesCommand, RecoverCommand, TimeMachineCommand};

/// CLI version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
