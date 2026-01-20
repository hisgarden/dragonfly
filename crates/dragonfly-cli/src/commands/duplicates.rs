//! Duplicate files command handler

use crate::types::DuplicatesCommand;
use anyhow::Result;
use colored::Colorize;

pub async fn handle_duplicates(command: DuplicatesCommand, json: bool) -> Result<()> {
    match command {
        DuplicatesCommand::Scan {
            path,
            min_size,
            dry_run,
            interactive,
            json: cmd_json,
        } => {
            let output_json = json || cmd_json;
            if output_json {
                println!(
                    r#"{{"status":"ok","message":"Duplicate scan (MVP stub)","path":"{}","min_size":"{:?}","dry_run":{},"interactive":{}}}"#,
                    path.display(),
                    min_size,
                    dry_run,
                    interactive
                );
            } else {
                println!("{}", "Duplicate File Scanner".bold().bright_cyan());
                println!("Path: {}", path.display());
                if let Some(ref ms) = min_size {
                    println!("Minimum size: {}", ms);
                }
                if dry_run {
                    println!("{}", "Mode: Dry run".yellow());
                }
                if interactive {
                    println!("{}", "Mode: Interactive".cyan());
                }
                println!(
                    "\n{}",
                    "This is an MVP stub. Full implementation coming soon.".dimmed()
                );
            }
        }
        DuplicatesCommand::Stats {
            path,
            json: cmd_json,
        } => {
            let output_json = json || cmd_json;
            if output_json {
                println!(
                    r#"{{"status":"ok","message":"Duplicate statistics (MVP stub)","path":"{}"}}"#,
                    path.display()
                );
            } else {
                println!("{}", "Duplicate Statistics".bold().bright_cyan());
                println!("Path: {}", path.display());
                println!(
                    "\n{}",
                    "This is an MVP stub. Full implementation coming soon.".dimmed()
                );
            }
        }
    }
    Ok(())
}
