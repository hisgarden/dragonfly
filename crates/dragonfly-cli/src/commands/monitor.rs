//! System monitoring command handler

use anyhow::Result;
use colored::Colorize;
use dragonfly_monitor::{MetricsCollector, SystemMetrics};
use humansize::{format_size, DECIMAL};
use serde_json::json;
use std::io::{self, Write};
use tokio::time::{sleep, Duration};

/// Display metrics in a formatted table
fn display_metrics(metrics: &SystemMetrics) {
    print!("\x1B[2J\x1B[1;1H"); // Clear screen and move cursor to top
    println!("{}", "System Monitor".bold().bright_cyan());
    println!("{}", "=".repeat(50).dimmed());
    println!();

    // CPU
    let cpu_color = if metrics.cpu_usage_percent > 80.0 {
        "red"
    } else if metrics.cpu_usage_percent > 50.0 {
        "yellow"
    } else {
        "green"
    };
    println!(
        "CPU:    {:>6.1}% {}",
        metrics.cpu_usage_percent,
        format_bar(metrics.cpu_usage_percent / 100.0, cpu_color)
    );

    // Memory
    let mem_percent = metrics.memory_usage_percent();
    let mem_color = if mem_percent > 90.0 {
        "red"
    } else if mem_percent > 70.0 {
        "yellow"
    } else {
        "green"
    };
    println!(
        "Memory: {:>6.1}% {} ({}/{})",
        mem_percent,
        format_bar(mem_percent / 100.0, mem_color),
        format_size(metrics.memory_used_bytes, DECIMAL),
        format_size(metrics.memory_total_bytes, DECIMAL)
    );

    // Swap
    if metrics.swap_total_bytes > 0 {
        let swap_percent =
            (metrics.swap_used_bytes as f32 / metrics.swap_total_bytes as f32) * 100.0;
        let swap_color = if swap_percent > 50.0 {
            "yellow"
        } else {
            "green"
        };
        println!(
            "Swap:   {:>6.1}% {} ({}/{})",
            swap_percent,
            format_bar(swap_percent / 100.0, swap_color),
            format_size(metrics.swap_used_bytes, DECIMAL),
            format_size(metrics.swap_total_bytes, DECIMAL)
        );
    }

    // Disk
    let disk_percent = metrics.disk_usage_percent();
    let disk_color = if disk_percent > 90.0 {
        "red"
    } else if disk_percent > 80.0 {
        "yellow"
    } else {
        "green"
    };
    println!(
        "Disk:   {:>6.1}% {} ({}/{})",
        disk_percent,
        format_bar(disk_percent / 100.0, disk_color),
        format_size(metrics.disk_used_bytes, DECIMAL),
        format_size(metrics.disk_total_bytes, DECIMAL)
    );

    println!();
    println!("{}", "Press Ctrl+C to exit".dimmed());
    io::stdout().flush().unwrap();
}

/// Format a progress bar
fn format_bar(value: f32, color: &str) -> String {
    let width: usize = 20;
    let filled = (value * width as f32) as usize;
    let bar = "█".repeat(filled) + &"░".repeat(width.saturating_sub(filled));
    match color {
        "red" => bar.red().to_string(),
        "yellow" => bar.yellow().to_string(),
        "green" => bar.green().to_string(),
        _ => bar,
    }
}

pub async fn handle_monitor(interval: u64, json: bool) -> Result<()> {
    let mut collector = MetricsCollector::new();

    if json {
        // JSON mode: output single snapshot and exit
        let metrics = collector.collect().await?;
        let json_output = json!({
            "status": "ok",
            "cpu_usage_percent": metrics.cpu_usage_percent,
            "memory_total_bytes": metrics.memory_total_bytes,
            "memory_used_bytes": metrics.memory_used_bytes,
            "memory_available_bytes": metrics.memory_available_bytes,
            "swap_total_bytes": metrics.swap_total_bytes,
            "swap_used_bytes": metrics.swap_used_bytes,
            "disk_total_bytes": metrics.disk_total_bytes,
            "disk_used_bytes": metrics.disk_used_bytes,
            "disk_available_bytes": metrics.disk_available_bytes,
            "network_rx_bytes": metrics.network_rx_bytes,
            "network_tx_bytes": metrics.network_tx_bytes,
            "timestamp": metrics.timestamp
        });
        println!("{}", serde_json::to_string_pretty(&json_output)?);
        return Ok(());
    }

    // Interactive mode: continuous monitoring
    println!("{}", "System Monitor".bold().bright_cyan());
    println!("Update interval: {} seconds", interval);
    println!("{}", "Press Ctrl+C to exit".dimmed());
    sleep(Duration::from_secs(1)).await;

    loop {
        match collector.collect().await {
            Ok(metrics) => {
                display_metrics(&metrics);
            }
            Err(e) => {
                eprintln!("Error collecting metrics: {}", e);
            }
        }
        sleep(Duration::from_secs(interval)).await;
    }
}
