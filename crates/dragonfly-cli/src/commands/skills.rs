//! Skills command handler - displays workflow cheat sheet
//!
//! This command provides a quick reference guide for common DragonFly workflows.

use anyhow::Result;
use colored::Colorize;

/// Handle the skills command - display workflow cheat sheet
pub async fn handle_skills(json: bool) -> Result<()> {
    if json {
        // JSON output for automation
        let json_output = serde_json::json!({
            "command": "skills",
            "description": "DragonFly workflow cheat sheet",
            "workflows": {
                "disk_analysis": {
                    "goal": "Identify big directories/files quickly",
                    "commands": [
                        "dragonfly disk analyze ~/",
                        "dragonfly disk analyze ~/ --json > report.json",
                        "dragonfly disk analyze ~/ --min-size 500MB"
                    ],
                    "notes": [
                        "Use --json for automation/ingestion into other tooling",
                        "Use --min-size to focus on big items first"
                    ]
                },
                "duplicate_scan": {
                    "goal": "Locate duplicates and optionally remove safely",
                    "commands": [
                        "dragonfly duplicates scan ~/Pictures",
                        "dragonfly duplicates scan ~/Documents --interactive",
                        "dragonfly duplicates scan ~/ --dry-run"
                    ],
                    "notes": [
                        "Prefer --interactive for human confirmation",
                        "Prefer --dry-run to preview actions"
                    ]
                },
                "monitor": {
                    "goal": "View CPU, memory, disk, network at a glance",
                    "commands": [
                        "dragonfly monitor"
                    ],
                    "notes": [
                        "Useful before/after cleanup or large file operations"
                    ]
                },
                "clean": {
                    "goal": "Reclaim space by cleaning caches",
                    "commands": [
                        "dragonfly clean --dry-run"
                    ],
                    "notes": [
                        "Run without --dry-run only after verifying what will be removed"
                    ]
                }
            },
            "quick_recipes": {
                "huge_downloads": "dragonfly disk analyze ~/Downloads --min-size 200MB",
                "safe_duplicate_scan": "dragonfly duplicates scan ~/Pictures --dry-run",
                "safe_clean": "dragonfly clean --dry-run"
            }
        });
        println!("{}", serde_json::to_string_pretty(&json_output)?);
        return Ok(());
    }

    // Human-friendly formatted output
    println!();
    println!("{}", "🐉 DragonFly Skills - Workflow Cheat Sheet".bold().bright_cyan());
    println!("{}", "===========================================".dimmed());
    println!();

    // Purpose
    println!("{}", "Purpose:".bold());
    println!("  Local, fast disk-related workflows");
    println!("  • Disk usage analysis (find what uses space)");
    println!("  • Duplicate detection");
    println!("  • Basic system monitoring (CPU/mem/disk/net)");
    println!("  • Cleanup (with dry-run safety)");
    println!();

    // Safety Rules
    println!("{}", "Safety Rules:".bold().bright_yellow());
    println!("  • Prefer {} when available before deletion/cleanup", "--dry-run".green());
    println!("  • For duplicates, use {} before deleting anything", "--interactive".green());
    println!("  • Avoid scanning system-critical folders: /System, /private, etc.");
    println!();

    // Disk Analysis
    println!("{}", "1. Disk Analysis".bold().bright_cyan());
    println!("   Goal: Identify big directories/files quickly");
    println!();
    println!("   Commands:");
    println!("   {}", "dragonfly disk analyze ~/".green());
    println!("   {}", "dragonfly disk analyze ~/ --json > report.json".green());
    println!("   {}", "dragonfly disk analyze ~/ --min-size 500MB".green());
    println!();
    println!("   Notes:");
    println!("   • Use {} for automation/ingestion into other tooling", "--json".cyan());
    println!("   • Use {} to focus on big items first", "--min-size".cyan());
    println!();

    // Duplicate Scan
    println!("{}", "2. Duplicate Scan".bold().bright_cyan());
    println!("   Goal: Locate duplicates and optionally remove safely");
    println!();
    println!("   Commands:");
    println!("   {}", "dragonfly duplicates scan ~/Pictures".green());
    println!("   {}", "dragonfly duplicates scan ~/Documents --interactive".green());
    println!("   {}", "dragonfly duplicates scan ~/ --dry-run".green());
    println!();
    println!("   Notes:");
    println!("   • Prefer {} for human confirmation", "--interactive".cyan());
    println!("   • Prefer {} to preview actions", "--dry-run".cyan());
    println!();

    // Monitor
    println!("{}", "3. Monitor".bold().bright_cyan());
    println!("   Goal: View CPU, memory, disk, network at a glance");
    println!();
    println!("   Command:");
    println!("   {}", "dragonfly monitor".green());
    println!();
    println!("   Notes:");
    println!("   • Useful before/after cleanup or large file operations");
    println!();

    // Clean
    println!("{}", "4. Clean".bold().bright_cyan());
    println!("   Goal: Reclaim space by cleaning caches");
    println!();
    println!("   Command:");
    println!("   {}", "dragonfly clean --dry-run".green());
    println!();
    println!("   Notes:");
    println!("   • Run without {} only after verifying what will be removed", "--dry-run".cyan());
    println!();

    // macOS-specific tips
    println!("{}", "macOS-Specific Path Tips:".bold().bright_yellow());
    println!("   • External disks: usually under {}", "/Volumes/<DiskName>/...".cyan());
    println!("   • Photos library: {}", "~/Pictures/Photos Library.photoslibrary".cyan());
    println!("   • iOS backups: {}", "~/Library/Application Support/MobileSync/Backup/".cyan());
    println!();

    // Quick Recipes
    println!("{}", "Quick Recipes:".bold().bright_magenta());
    println!("   • What's huge in Downloads?");
    println!("     {}", "dragonfly disk analyze ~/Downloads --min-size 200MB".green());
    println!();
    println!("   • Find duplicates in Pictures safely:");
    println!("     {}", "dragonfly duplicates scan ~/Pictures --dry-run".green());
    println!();
    println!("   • Clean safely:");
    println!("     {}", "dragonfly clean --dry-run".green());
    println!();

    // Output tips
    println!("{}", "Output for Automation:".bold());
    println!("   • JSON output: {}", "dragonfly disk analyze <path> --json > report.json".cyan());
    println!("   • Keep reports in: {}", "~/dragonfly-reports/".cyan());
    println!();

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_skills_display() {
        // Test that the command runs without error
        let result = handle_skills(false).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_skills_json() {
        // Test that JSON output is valid
        let result = handle_skills(true).await;
        assert!(result.is_ok());
    }
}
