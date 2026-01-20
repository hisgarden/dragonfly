//! # `DragonFly` Core
//!
//! Core domain types, entities, value objects, and business logic for `DragonFly`.
//!
//! This crate implements the **Domain Layer** of Clean Architecture, containing:
//! - Pure business logic with no external dependencies
//! - Domain entities and value objects
//! - Port traits (interfaces for dependency inversion)
//! - Use case implementations
//!
//! ## Architecture
//!
//! ```text
//! ┌─────────────────────────────────────────┐
//! │         dragonfly-core                  │
//! │  ┌───────────────────────────────────┐  │
//! │  │  Domain Layer (Pure Logic)        │  │
//! │  │  - Entities                       │  │
//! │  │  - Value Objects                  │  │
//! │  │  - Domain Events                  │  │
//! │  └───────────────────────────────────┘  │
//! │  ┌───────────────────────────────────┐  │
//! │  │  Ports (Interfaces)               │  │
//! │  │  - Repository Traits              │  │
//! │  │  - Service Traits                 │  │
//! │  └───────────────────────────────────┘  │
//! │  ┌───────────────────────────────────┐  │
//! │  │  Use Cases (Business Rules)       │  │
//! │  │  - Application Logic              │  │
//! │  └───────────────────────────────────┘  │
//! └─────────────────────────────────────────┘
//! ```
//!
//! ## Design Principles
//!
//! ### SOLID Principles
//! - **S**ingle Responsibility: Each module has one reason to change
//! - **O**pen/Closed: Open for extension, closed for modification
//! - **L**iskov Substitution: Subtypes are substitutable for base types
//! - **I**nterface Segregation: Many specific interfaces over one general
//! - **D**ependency Inversion: Depend on abstractions (ports), not concretions
//!
//! ### Domain-Driven Design (DDD)
//! - **Ubiquitous Language**: Code reflects domain expert terminology
//! - **Bounded Contexts**: Clear boundaries between different domains
//! - **Rich Domain Models**: Entities encapsulate behavior, not just data
//! - **Value Objects**: Immutable, equality by value
//! - **Domain Events**: Capture important business occurrences
//!
//! ### Hexagonal Architecture (Ports & Adapters)
//! - **Ports**: Define boundaries (traits in this crate)
//! - **Domain Logic**: Independent of infrastructure
//! - **Dependency Inversion**: Domain doesn't depend on infrastructure
//!
//! ## Usage Example
//!
//! ```rust
//! use dragonfly_core::domain::{FileSize, FilePath, FileEntity};
//!
//! // Create value objects (immutable, type-safe)
//! // 100 MB = 100 * 1024 * 1024 bytes
//! let size = FileSize::new(100 * 1024 * 1024);
//! let path = FilePath::new("/Users/me/large_file.dat".to_string());
//!
//! // Create domain entity
//! let file = FileEntity {
//!     path: path.as_str().to_string(),
//!     size: size.bytes(),
//! };
//!
//! // Use value objects
//! assert_eq!(size.bytes(), 104_857_600);
//! assert_eq!(path.as_str(), "/Users/me/large_file.dat");
//! ```
//!
//! ## Module Organization
//!
//! - [`domain`]: Core domain entities, value objects, and domain events
//! - [`ports`]: Port traits (interfaces) for dependency inversion
//! - [`use_cases`]: Business use cases and application logic
//! - [`error`]: Domain-specific error types
//!
//! ## Testing Philosophy
//!
//! This crate emphasizes **Test-Driven Development (TDD)**:
//! - Write tests first, then implementation
//! - Unit tests for all domain logic
//! - Property-based tests for value objects
//! - Mock implementations for ports during testing
//!
//! ## No Infrastructure Dependencies
//!
//! This crate MUST NOT depend on:
//! - File system operations (use ports instead)
//! - Network operations
//! - Database operations
//! - UI frameworks
//! - External services
//!
//! All external interactions are defined as port traits that
//! will be implemented by infrastructure adapters.

#![warn(
    missing_docs,
    missing_debug_implementations,
    missing_copy_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unstable_features,
    unused_import_braces,
    unused_qualifications,
    clippy::all,
    clippy::pedantic,
    clippy::cargo
)]
#![allow(clippy::module_name_repetitions)]

/// Domain layer containing entities, value objects, and domain events
///
/// This module contains pure business logic with no external dependencies.
/// All types here should be:
/// - Serializable (for JSON output, AI integration)
/// - Well-tested (unit and property-based tests)
/// - Immutable where possible (especially value objects)
/// - Self-contained (no infrastructure concerns)
pub mod domain;

/// Error types for domain operations
///
/// Defines all possible errors that can occur in the domain layer.
/// Errors are strongly typed and provide context for debugging.
pub mod error;

/// Port traits (interfaces) for dependency inversion
///
/// Ports define the boundaries of the application. External systems
/// interact with the domain through these ports, implemented as traits.
///
/// - **Driving Ports** (Primary/Input): Called by external actors
/// - **Driven Ports** (Secondary/Output): Called by the domain
pub mod ports;

/// Use cases (application business rules)
///
/// Use cases orchestrate the flow of data to and from entities,
/// and direct those entities to use their business rules to achieve
/// the goals of the use case.
pub mod use_cases;

// Re-export commonly used types for convenience
pub use error::{Error, Result};

// Re-export domain types
pub use domain::{
    entities::{DirectoryEntity, FileEntity, HealthStatus, SystemSnapshot},
    value_objects::{FilePath, FileSize, Percentage},
    DomainEvent,
};

// Version information
/// The version of the dragonfly-core crate
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version_is_set() {
        assert!(!VERSION.is_empty());
        assert!(VERSION.contains('.'));
    }
}
