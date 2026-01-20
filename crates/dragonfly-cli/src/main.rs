//! DragonFly CLI - Command-line interface for macOS maintenance
//!
//! This is the main entry point for the DragonFly application.
//! It provides a user-friendly CLI for disk analysis, duplicate detection,
//! system monitoring, and cache cleaning.

use anyhow::Result;
use clap::{Parser, Subcommand};
use colored::Colorize;
use sentry::{init, ClientInitGuard};
use std::borrow::Cow;
use std::env;
use tracing_subscriber::EnvFilter;

use dragonfly_cli::commands::{analyze, clean, duplicates, health, monitor, recover};
use dragonfly_cli::{DiskCommand, DuplicatesCommand, RecoverCommand, TimeMachineCommand};

#[derive(Parser)]
#[command(
    name = "dragonfly",
    version = env!("CARGO_PKG_VERSION"),
    author = "hisgarden <https://github.com/hisgarden>",
    about = "Privacy-first macOS maintenance utility",
    long_about = "DragonFly is a modular macOS maintenance utility built in Rust.\n\n\
                   Features:\n  \
                   • Disk analysis - Find what's consuming disk space\n  \
                   • Duplicate finder - Identify and remove duplicate files\n  \
                   • System monitor - Real-time system metrics\n  \
                   • Cache cleaner - Safe cleanup of caches and temp files\n  \
                   • Health check - Comprehensive system diagnostics\n\n\
                   Privacy first: 100% local processing, zero network activity"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Enable debug logging
    #[arg(global = true, short, long)]
    debug: bool,

    /// Enable JSON output
    #[arg(global = true, long)]
    json: bool,

    /// Enable error tracking (Sentry) - sends errors to remote server
    #[arg(global = true, long)]
    enable_error_tracking: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Analyze disk usage
    #[command(about = "Analyze disk usage in a directory")]
    Disk {
        #[command(subcommand)]
        command: DiskCommand,
    },

    /// Find duplicate files
    #[command(about = "Find and manage duplicate files")]
    Duplicates {
        #[command(subcommand)]
        command: DuplicatesCommand,
    },

    /// Monitor system in real-time
    #[command(about = "Monitor CPU, memory, disk, and network usage")]
    Monitor {
        /// Update interval in seconds
        #[arg(short, long, default_value = "5")]
        interval: u64,

        /// Run in JSON output mode
        #[arg(long)]
        json: bool,
    },

    /// Clean caches and temporary files
    #[command(about = "Clean system caches and temporary files")]
    Clean {
        /// Perform a dry run (don't actually delete)
        #[arg(short, long)]
        dry_run: bool,

        /// Clean all (caches, logs, temps)
        #[arg(long)]
        all: bool,

        /// Clean caches only
        #[arg(long)]
        caches: bool,

        /// Clean logs only
        #[arg(long)]
        logs: bool,

        /// Clean temporary files
        #[arg(long)]
        temp: bool,

        /// Interactive mode (confirm each deletion)
        #[arg(short, long)]
        interactive: bool,
    },

    /// System health check
    #[command(about = "Check system health and get recommendations")]
    Health {
        /// Output as JSON
        #[arg(long)]
        json: bool,

        /// Show recommendations
        #[arg(short, long)]
        recommend: bool,

        /// Check specific component (disk, memory, cpu)
        #[arg(short, long)]
        component: Option<String>,
    },

    /// Recover cleaned files
    #[command(about = "Manage and restore cleaned files")]
    Recover {
        #[command(subcommand)]
        command: RecoverCommand,
    },

    /// Time Machine snapshot management
    #[command(about = "Manage Time Machine local snapshots")]
    TimeMachine {
        #[command(subcommand)]
        command: TimeMachineCommand,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    // Initialize Sentry only if explicitly enabled
    let _guard = if cli.enable_error_tracking {
        init_sentry()
    } else {
        // No-op guard - Sentry disabled for privacy
        init(("", sentry::ClientOptions::default()))
    };

    // Initialize logging
    init_logging(cli.debug)?;

    // Print header
    if !cli.json {
        print_header();
    }

    let result = match cli.command {
        Commands::Disk { command } => analyze::handle_disk(command, cli.json).await,
        Commands::Duplicates { command } => duplicates::handle_duplicates(command, cli.json).await,
        Commands::Monitor { interval, json } => monitor::handle_monitor(interval, json).await,
        Commands::Clean {
            dry_run,
            all,
            caches,
            logs,
            temp,
            interactive,
        } => clean::handle_clean(dry_run, all, caches, logs, temp, interactive, cli.json).await,
        Commands::Health {
            json,
            recommend,
            component,
        } => health::handle_health(json, recommend, component, cli.json).await,
        Commands::Recover { command } => match command {
            RecoverCommand::List { json } => recover::handle_recover_list(json || cli.json).await,
            RecoverCommand::Show { id, json } => {
                recover::handle_recover_show(id, json || cli.json).await
            }
            RecoverCommand::Restore { id, json } => {
                recover::handle_recover_restore(id, json || cli.json).await
            }
            RecoverCommand::Cleanup { json } => {
                recover::handle_recover_cleanup(json || cli.json).await
            }
        },
        Commands::TimeMachine { command } => {
            match command {
                TimeMachineCommand::Snapshots { json } => {
                    // TODO: Implement Time Machine snapshot listing
                    if json || cli.json {
                        println!(
                            r#"{{"status":"ok","message":"Time Machine snapshots (MVP stub)"}}"#
                        );
                    } else {
                        println!("{}", "Time Machine Snapshots".bold().bright_cyan());
                        println!(
                            "\n{}",
                            "This is an MVP stub. Full implementation coming soon.".dimmed()
                        );
                    }
                    Ok(())
                }
            }
        }
    };

    // Report errors to Sentry only if error tracking is enabled
    if cli.enable_error_tracking {
        if let Err(ref error) = result {
            // Convert anyhow::Error to something Sentry can capture
            if let Some(source) = error.source() {
                sentry::capture_error(source);
            } else {
                // Fallback: capture as message if no source error
                sentry::capture_message(&format!("Error: {}", error), sentry::Level::Error);
            }
        }
    }

    result
}

/// Initialize Sentry error tracking (only called when --enable-error-tracking is set)
/// Reads DSN from SENTRY_DSN environment variable or .sentryclirc file
/// Returns a no-op guard if no DSN is configured
fn init_sentry() -> ClientInitGuard {
    // Check for DSN in environment variable first
    let dsn = env::var("SENTRY_DSN").ok();

    // If not in env, try reading from .sentryclirc file
    let dsn = dsn.or_else(|| {
        std::fs::read_to_string(".sentryclirc")
            .ok()
            .and_then(|content| {
                // Look for defaults.url= line
                content
                    .lines()
                    .find(|line| line.starts_with("defaults.url="))
                    .and_then(|line| {
                        let url = line.strip_prefix("defaults.url=")?.trim();
                        // Sentry URL format: https://KEY@HOST/PROJECT_ID
                        // Extract the full DSN
                        if url.starts_with("https://") {
                            Some(url.to_string())
                        } else {
                            None
                        }
                    })
            })
    });

    let is_debug = cfg!(debug_assertions);
    let release = format!("dragonfly@{}", env!("CARGO_PKG_VERSION"));
    let environment = if is_debug {
        "development"
    } else {
        "production"
    };

    if let Some(dsn) = dsn {
        init((
            dsn,
            sentry::ClientOptions {
                release: Some(Cow::Owned(release.clone())),
                environment: Some(Cow::Borrowed(environment)),
                // Privacy: Don't send PII by default
                send_default_pii: false,
                // Sample rate for performance monitoring (100% in dev, 10% in production)
                traces_sample_rate: if is_debug { 1.0 } else { 0.1 },
                // Attach stack traces to all events
                attach_stacktrace: true,
                // Enable breadcrumbs for better debugging
                max_breadcrumbs: 100,
                ..Default::default()
            },
        ))
    } else {
        // No DSN configured - Sentry will be a no-op
        // This allows the app to run without Sentry configured
        init((
            "",
            sentry::ClientOptions {
                release: Some(Cow::Owned(release)),
                ..Default::default()
            },
        ))
    }
}

fn init_logging(debug: bool) -> Result<()> {
    let env_filter = if debug {
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("debug"))
    } else {
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"))
    };

    tracing_subscriber::fmt()
        .with_env_filter(env_filter)
        .with_writer(std::io::stderr)
        .init();

    Ok(())
}

fn print_header() {
    println!();
    println!(
        "{} {}",
        "🐉".bright_cyan(),
        "DragonFly".bold().bright_cyan()
    );
    println!("{}", format!("v{}", env!("CARGO_PKG_VERSION")).dimmed());
    println!("{}", "Privacy-first macOS maintenance utility".dimmed());
    println!();
}
