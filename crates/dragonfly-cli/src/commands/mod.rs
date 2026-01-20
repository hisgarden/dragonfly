//! CLI command handlers
//!
//! Each module handles a specific CLI command and coordinates
//! between the user interface and domain layer.

pub mod analyze;
pub mod clean;
pub mod duplicates;
pub mod health;
pub mod monitor;
pub mod recover;

#[cfg(feature = "skills")]
pub mod skills;

pub use analyze::handle_disk;
pub use clean::handle_clean;
pub use duplicates::handle_duplicates;
pub use health::handle_health;
pub use monitor::handle_monitor;
pub use recover::*;

#[cfg(feature = "skills")]
pub use skills::handle_skills;
