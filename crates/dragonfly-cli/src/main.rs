//! DragonFly CLI - Command-line interface for macOS maintenance
//!
//! This is the main entry point for the DragonFly application.
//! It provides a user-friendly CLI for disk analysis, duplicate detection,
//! system monitoring, and cache cleaning.

use anyhow::Result;
use clap::{Parser, Subcommand};
use colored::Colorize;
use tracing_subscriber::EnvFilter;

use dragonfly_cli::commands::{analyze, clean, duplicates, health, monitor, recover};
#[cfg(feature = "skills")]
use dragonfly_cli::commands::skills;
use dragonfly_cli::error_tracking::{init_error_tracking, load_config};
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

    /// Enable error tracking (GlitchTip only) - sends errors to local/self-hosted server
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
        #[arg(long)]
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

    /// Display workflow cheat sheet
    #[cfg(feature = "skills")]
    #[command(about = "Display DragonFly workflow cheat sheet and quick reference")]
    Skills {
        /// Output as JSON
        #[arg(long)]
        json: bool,
    },

    /// Retro defrag-style TUI for disk cleanup
    #[cfg(feature = "tui")]
    #[command(about = "Launch retro defrag-style terminal UI for disk scanning and cleanup")]
    Defrag {
        /// Path to scan
        #[arg(default_value = "~")]
        path: String,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    // Initialize error tracking only if explicitly enabled
    let _guard = if cli.enable_error_tracking {
        let config = load_config();
        init_error_tracking(config)
    } else {
        // No-op guard - error tracking disabled for privacy
        sentry::init(("", sentry::ClientOptions::default()))
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
        Commands::TimeMachine { command } => match command {
            TimeMachineCommand::Snapshots { json } => {
                use dragonfly_cleaner::TimeMachineManager;
                use humansize::{format_size, DECIMAL};

                let snapshots = TimeMachineManager::list_snapshots()?;

                if json || cli.json {
                    let json_output = serde_json::json!({
                        "status": "ok",
                        "snapshots": snapshots.iter().map(|s| serde_json::json!({
                            "id": s.id,
                            "date": s.date,
                            "size": s.size
                        })).collect::<Vec<_>>(),
                        "count": snapshots.len()
                    });
                    println!("{}", serde_json::to_string_pretty(&json_output)?);
                } else {
                    println!("{}", "Time Machine Snapshots".bold().bright_cyan());
                    println!();
                    if snapshots.is_empty() {
                        println!("No local snapshots found.");
                    } else {
                        println!("Found {} local snapshot(s):\n", snapshots.len());
                        for (i, snapshot) in snapshots.iter().enumerate() {
                            println!("{}. {}", i + 1, snapshot.id);
                            println!("   Date: {}", snapshot.date);
                            if let Some(size) = snapshot.size {
                                println!("   Size: {}", format_size(size, DECIMAL));
                            }
                            println!();
                        }
                        println!(
                            "{}",
                            "Note: Use 'tmutil deletelocalsnapshot <id>' to delete snapshots"
                                .dimmed()
                        );
                    }
                }
                Ok(())
            }
        },
        #[cfg(feature = "skills")]
        Commands::Skills { json } => skills::handle_skills(json || cli.json).await,
        #[cfg(feature = "tui")]
        Commands::Defrag { path } => {
            // Expand ~ to home directory
            let expanded_path = if path.starts_with('~') {
                if let Some(home) = dirs::home_dir() {
                    path.replacen('~', home.to_str().unwrap_or("/"), 1)
                } else {
                    path
                }
            } else {
                path
            };
            dragonfly_tui::run_app(expanded_path).await
        },
    };

    // Report errors to GlitchTip only if enabled
    if cli.enable_error_tracking {
        if let Err(ref error) = result {
            // Convert anyhow::Error to something GlitchTip can capture
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
