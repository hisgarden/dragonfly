//! Disk analysis command handler

use crate::types::DiskCommand;
use anyhow::{Context, Result};
use colored::Colorize;
use dragonfly_core::domain::value_objects::FilePath;
use dragonfly_disk::DiskAnalyzer;
use humansize::{format_size, DECIMAL};
use serde_json::json;
use std::cmp::Reverse;

/// Parse size string like "100MB", "1GB" to bytes
fn parse_size(size_str: &str) -> Result<u64> {
    let size_str = size_str.trim().to_uppercase();
    let (num_str, unit) = if size_str.ends_with("KB") {
        (size_str.trim_end_matches("KB"), 1024)
    } else if size_str.ends_with("MB") {
        (size_str.trim_end_matches("MB"), 1024 * 1024)
    } else if size_str.ends_with("GB") {
        (size_str.trim_end_matches("GB"), 1024 * 1024 * 1024)
    } else if size_str.ends_with("TB") {
        (size_str.trim_end_matches("TB"), 1024_u64.pow(4))
    } else if size_str.ends_with('B') {
        (size_str.trim_end_matches('B'), 1)
    } else {
        // Assume bytes if no unit
        (size_str.as_str(), 1)
    };

    let num: u64 = num_str
        .parse()
        .with_context(|| format!("Invalid size format: {}", size_str))?;
    Ok(num * unit)
}

pub async fn handle_disk(command: DiskCommand, json: bool) -> Result<()> {
    match command {
        DiskCommand::Analyze {
            path,
            min_size,
            top,
            json: cmd_json,
        } => {
            let output_json = json || cmd_json;
            let file_path = FilePath::new(path.to_string_lossy().to_string());
            let analyzer = DiskAnalyzer::new();

            let result = analyzer
                .analyze(&file_path)
                .await
                .context("Failed to analyze directory")?;

            let mut files = result.files;

            // Filter by min_size if provided
            if let Some(ref ms) = min_size {
                let min_bytes = parse_size(ms)?;
                files.retain(|f| f.size >= min_bytes);
            }

            // Sort by size descending
            files.sort_by_key(|f| Reverse(f.size));

            // Take top N
            let top_files: Vec<_> = files.into_iter().take(top).collect();

            if output_json {
                let json_output = json!({
                    "status": "ok",
                    "path": file_path.as_str(),
                    "total_size": result.total_size,
                    "total_files": top_files.len(),
                    "files": top_files.iter().map(|f| json!({
                        "path": f.path,
                        "size": f.size
                    })).collect::<Vec<_>>()
                });
                println!("{}", serde_json::to_string_pretty(&json_output)?);
            } else {
                println!("{}", "Disk Analysis".bold().bright_cyan());
                println!("Path: {}", file_path.as_str());
                println!("Total size: {}", format_size(result.total_size, DECIMAL));
                println!("Total files: {}", top_files.len());
                if let Some(ref ms) = min_size {
                    println!("Minimum size filter: {}", ms);
                }
                println!("\nTop {} largest files:\n", top);
                for (i, file) in top_files.iter().enumerate() {
                    println!(
                        "{:3}. {} - {}",
                        i + 1,
                        format_size(file.size, DECIMAL).bold(),
                        file.path
                    );
                }
            }
        }
        DiskCommand::Large {
            path,
            min_size,
            json: cmd_json,
        } => {
            let output_json = json || cmd_json;
            let file_path = FilePath::new(path.to_string_lossy().to_string());
            let analyzer = DiskAnalyzer::new();

            let min_bytes = parse_size(&min_size)
                .with_context(|| format!("Invalid size format: {}", min_size))?;

            let large_files = analyzer
                .find_large_files(&file_path, min_bytes)
                .await
                .context("Failed to find large files")?;

            // Sort by size descending
            let mut sorted_files = large_files;
            sorted_files.sort_by_key(|f| Reverse(f.size));

            if output_json {
                let json_output = json!({
                    "status": "ok",
                    "path": file_path.as_str(),
                    "min_size": min_size,
                    "min_size_bytes": min_bytes,
                    "files_found": sorted_files.len(),
                    "files": sorted_files.iter().map(|f| json!({
                        "path": f.path,
                        "size": f.size
                    })).collect::<Vec<_>>()
                });
                println!("{}", serde_json::to_string_pretty(&json_output)?);
            } else {
                println!("{}", "Finding Large Files".bold().bright_cyan());
                println!("Path: {}", file_path.as_str());
                println!(
                    "Minimum size: {} ({})",
                    min_size,
                    format_size(min_bytes, DECIMAL)
                );
                println!("Files found: {}\n", sorted_files.len());
                for (i, file) in sorted_files.iter().enumerate() {
                    println!(
                        "{:3}. {} - {}",
                        i + 1,
                        format_size(file.size, DECIMAL).bold(),
                        file.path
                    );
                }
            }
        }
    }
    Ok(())
}
