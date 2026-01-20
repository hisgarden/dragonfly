//! Cache and temporary file cleaning command handler

use anyhow::{Context, Result};
use colored::Colorize;
use dragonfly_cleaner::{CleanTarget, SystemCleaner};
use humansize::{format_size, DECIMAL};
use serde_json::json;

pub async fn handle_clean(
    dry_run: bool,
    all: bool,
    caches: bool,
    logs: bool,
    temp: bool,
    interactive: bool,
    json: bool,
) -> Result<()> {
    let cleaner = SystemCleaner::new();

    // Determine target
    let target = if all {
        CleanTarget::All
    } else if caches {
        CleanTarget::Caches
    } else if logs {
        CleanTarget::Logs
    } else if temp {
        CleanTarget::Temp
    } else {
        // No target specified
        if json {
            println!(
                r#"{{"status":"error","message":"No target specified. Use --all, --caches, --logs, or --temp"}}"#
            );
        } else {
            println!("{}", "Cache Cleaner".bold().bright_cyan());
            println!(
                "{}",
                "No target specified. Use --all, --caches, --logs, or --temp".yellow()
            );
        }
        return Ok(());
    };

    // Perform cleaning
    let result = cleaner
        .clean(target, dry_run)
        .await
        .context("Failed to clean files")?;

    if json {
        let json_output = json!({
            "status": "ok",
            "dry_run": dry_run,
            "target": format!("{:?}", target),
            "files_found": result.files_found.len(),
            "files_cleaned": result.files_cleaned,
            "bytes_freed": result.bytes_freed,
            "bytes_freed_human": format_size(result.bytes_freed, DECIMAL)
        });
        println!("{}", serde_json::to_string_pretty(&json_output)?);
        return Ok(());
    }

    // Human-readable output
    println!("{}", "Cache Cleaner".bold().bright_cyan());
    if dry_run {
        println!("{}", "Mode: Dry run (no files will be deleted)".yellow());
    } else {
        println!("{}", "Mode: Cleaning (files will be deleted)".red().bold());
    }

    println!("Target: {:?}", target);
    println!();

    if dry_run {
        println!("Found {} files", result.files_found.len());
        println!(
            "Would free: {}",
            format_size(result.bytes_freed, DECIMAL).bold()
        );

        if interactive && !result.files_found.is_empty() {
            println!("\n{}", "Files that would be cleaned:".cyan());
            for (i, file) in result.files_found.iter().take(20).enumerate() {
                println!("  {}. {}", i + 1, file.display());
            }
            if result.files_found.len() > 20 {
                println!("  ... and {} more files", result.files_found.len() - 20);
            }
        }
    } else {
        println!("Cleaned {} files", result.files_cleaned);
        println!(
            "Freed: {}",
            format_size(result.bytes_freed, DECIMAL).bold().green()
        );
    }

    Ok(())
}
