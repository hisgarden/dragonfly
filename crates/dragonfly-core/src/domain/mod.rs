//! Domain layer - Core business logic with no external dependencies
//!
//! This module contains the heart of DragonFly: pure domain logic that is
//! independent of any infrastructure concerns like file systems, databases,
//! or network operations.
//!
//! ## Module Organization
//!
//! - [`entities`]: Domain entities with identity (File, Directory, System)
//! - [`value_objects`]: Immutable value objects (FileSize, FilePath, Percentage)
//! - [`events`]: Domain events that capture important business occurrences

pub mod entities;
pub mod events;
pub mod value_objects;

pub use entities::{DirectoryEntity, FileEntity, HealthStatus, SystemSnapshot};
pub use events::DomainEvent;
pub use value_objects::{FilePath, FileSize, Percentage};

/// Re-export commonly used domain types
pub mod prelude {
    pub use super::entities::*;
    pub use super::events::*;
    pub use super::value_objects::*;
}
