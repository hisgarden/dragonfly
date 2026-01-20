//! Recovery command handler for restoring cleaned files

use anyhow::Result;
use colored::Colorize;
use dragonfly_cleaner::RecoveryManager;

/// List available recoveries
pub async fn handle_recover_list(json: bool) -> Result<()> {
    let recovery_dir = RecoveryManager::default_dir();
    let manager = RecoveryManager::new(recovery_dir);
    manager.initialize()?;

    let recoveries = manager.list_recoveries()?;

    if json {
        println!("{}", serde_json::to_string_pretty(&recoveries)?);
    } else {
        println!("{}", "Available Recoveries".bold().bright_cyan());
        println!();
        if recoveries.is_empty() {
            println!("No recoveries available.");
        } else {
            for recovery in recoveries {
                println!("ID: {}", recovery.id);
                println!("Date: {}", recovery.timestamp.format("%Y-%m-%d %H:%M:%S"));
                println!("Size: {} bytes", recovery.total_size);
                println!("Items: {}", recovery.items.len());
                println!(
                    "Retention until: {}",
                    recovery.retention_until.format("%Y-%m-%d %H:%M:%S")
                );
                println!();
            }
        }
    }

    Ok(())
}

/// Show recovery details
pub async fn handle_recover_show(recovery_id: String, json: bool) -> Result<()> {
    let recovery_dir = RecoveryManager::default_dir();
    let manager = RecoveryManager::new(recovery_dir);
    manager.initialize()?;

    let manifest = manager.load_manifest(&recovery_id)?;

    if json {
        println!("{}", serde_json::to_string_pretty(&manifest)?);
    } else {
        println!("{}", "Recovery Details".bold().bright_cyan());
        println!("ID: {}", manifest.id);
        println!("Date: {}", manifest.timestamp.format("%Y-%m-%d %H:%M:%S"));
        println!("Total Size: {} bytes", manifest.total_size);
        println!("Items: {}", manifest.items.len());
        println!(
            "Retention until: {}",
            manifest.retention_until.format("%Y-%m-%d %H:%M:%S")
        );
        println!();
        println!("Items:");
        for item in manifest.items {
            println!("  - {}", item.original_path.display());
            println!("    Size: {} bytes", item.size);
            println!("    Category: {}", item.category);
            println!("    Source: {}", item.source);
        }
    }

    Ok(())
}

/// Restore a recovery
pub async fn handle_recover_restore(recovery_id: String, json: bool) -> Result<()> {
    use humansize::{format_size, DECIMAL};

    let recovery_dir = RecoveryManager::default_dir();
    let manager = RecoveryManager::new(recovery_dir);
    manager.initialize()?;

    // Load manifest to show what will be restored
    let manifest = manager.load_manifest(&recovery_id)?;

    if json {
        let (restored_count, restored_size) = manager.restore_recovery(&recovery_id)?;
        println!(
            r#"{{"status":"ok","recovery_id":"{}","files_restored":{},"bytes_restored":{}}}"#,
            recovery_id, restored_count, restored_size
        );
        return Ok(());
    }

    println!("{}", "Recovery Restore".bold().bright_cyan());
    println!("Recovery ID: {}", recovery_id);
    println!("Date: {}", manifest.timestamp.format("%Y-%m-%d %H:%M:%S"));
    println!("Items to restore: {}", manifest.items.len());
    println!();

    // Restore files
    match manager.restore_recovery(&recovery_id) {
        Ok((restored_count, restored_size)) => {
            println!("{}", "Restore completed successfully!".green().bold());
            println!("Files restored: {}", restored_count);
            println!(
                "Size restored: {}",
                format_size(restored_size, DECIMAL).bold()
            );
        }
        Err(e) => {
            return Err(anyhow::anyhow!("Failed to restore recovery: {}", e));
        }
    }

    Ok(())
}

/// Clean up expired recoveries
pub async fn handle_recover_cleanup(json: bool) -> Result<()> {
    let recovery_dir = RecoveryManager::default_dir();
    let manager = RecoveryManager::new(recovery_dir);
    manager.initialize()?;

    let cleaned = manager.cleanup_expired()?;

    if json {
        println!(r#"{{"status":"ok","cleaned":{}}}"#, cleaned.len());
    } else {
        println!("{}", "Recovery Cleanup".bold().bright_cyan());
        if cleaned.is_empty() {
            println!("No expired recoveries to clean.");
        } else {
            println!("Cleaned {} expired recoveries:", cleaned.len());
            for id in cleaned {
                println!("  - {}", id);
            }
        }
    }

    Ok(())
}
