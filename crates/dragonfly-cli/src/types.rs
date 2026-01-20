//! Command type definitions

use clap::Subcommand;
use std::path::PathBuf;

#[derive(Subcommand)]
pub enum DiskCommand {
    /// Analyze disk usage
    Analyze {
        /// Path to analyze
        #[arg(default_value = ".")]
        path: PathBuf,

        /// Minimum file size to consider (e.g., 100MB, 1GB)
        #[arg(short, long)]
        min_size: Option<String>,

        /// Number of top items to show
        #[arg(short, long, default_value = "10")]
        top: usize,

        /// Output as JSON
        #[arg(long)]
        json: bool,
    },

    /// Find large files
    Large {
        /// Path to search
        #[arg(default_value = ".")]
        path: PathBuf,

        /// Minimum file size (e.g., 100MB, 1GB)
        #[arg(short, long, default_value = "100MB")]
        min_size: String,

        /// Output as JSON
        #[arg(long)]
        json: bool,
    },
}

#[derive(Subcommand)]
pub enum DuplicatesCommand {
    /// Find duplicate files
    Scan {
        /// Path to scan
        #[arg(default_value = ".")]
        path: PathBuf,

        /// Minimum file size to consider
        #[arg(short, long)]
        min_size: Option<String>,

        /// Dry run (don't delete)
        #[arg(long)]
        dry_run: bool,

        /// Interactive mode (select which files to delete)
        #[arg(short, long)]
        interactive: bool,

        /// Output as JSON
        #[arg(long)]
        json: bool,
    },

    /// Show duplicate statistics
    Stats {
        /// Path to analyze
        #[arg(default_value = ".")]
        path: PathBuf,

        /// Output as JSON
        #[arg(long)]
        json: bool,
    },
}

#[derive(Subcommand)]
pub enum RecoverCommand {
    /// List all recoveries
    List {
        /// Output as JSON
        #[arg(long)]
        json: bool,
    },
    /// Show details of a recovery
    Show {
        /// Recovery ID
        id: String,
        /// Output as JSON
        #[arg(long)]
        json: bool,
    },
    /// Restore files from a recovery
    Restore {
        /// Recovery ID
        id: String,
        /// Output as JSON
        #[arg(long)]
        json: bool,
    },
    /// Cleanup old recoveries
    Cleanup {
        /// Output as JSON
        #[arg(long)]
        json: bool,
    },
}

#[derive(Subcommand)]
pub enum TimeMachineCommand {
    /// List Time Machine snapshots
    Snapshots {
        /// Output as JSON
        #[arg(long)]
        json: bool,
    },
}
