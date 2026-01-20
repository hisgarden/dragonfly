//! Disk analysis command handler

use crate::types::DiskCommand;
use anyhow::Result;
use colored::Colorize;

pub async fn handle_disk(command: DiskCommand, json: bool) -> Result<()> {
    match command {
        DiskCommand::Analyze {
            path,
            min_size,
            top,
            json: cmd_json,
        } => {
            let output_json = json || cmd_json;
            if output_json {
                println!(
                    r#"{{"status":"ok","message":"Disk analysis (MVP stub)","path":"{}","min_size":"{:?}","top":{}}}"#,
                    path.display(),
                    min_size,
                    top
                );
            } else {
                println!("{}", "Disk Analysis".bold().bright_cyan());
                println!("Path: {}", path.display());
                if let Some(ref ms) = min_size {
                    println!("Minimum size: {}", ms);
                }
                println!("Top {} items", top);
                println!(
                    "\n{}",
                    "This is an MVP stub. Full implementation coming soon.".dimmed()
                );
            }
        }
        DiskCommand::Large {
            path,
            min_size,
            json: cmd_json,
        } => {
            let output_json = json || cmd_json;
            if output_json {
                println!(
                    r#"{{"status":"ok","message":"Large files search (MVP stub)","path":"{}","min_size":"{}"}}"#,
                    path.display(),
                    min_size
                );
            } else {
                println!("{}", "Finding Large Files".bold().bright_cyan());
                println!("Path: {}", path.display());
                println!("Minimum size: {}", min_size);
                println!(
                    "\n{}",
                    "This is an MVP stub. Full implementation coming soon.".dimmed()
                );
            }
        }
    }
    Ok(())
}
