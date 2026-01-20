//! System health check command handler

use anyhow::Result;
use colored::Colorize;
use dragonfly_monitor::{MetricsCollector, SystemMetrics};
use humansize::{format_size, DECIMAL};
use serde_json::json;

/// Health status for a component
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum HealthStatus {
    Healthy,
    Warning,
    Critical,
}

/// Component health check result
#[derive(Debug, Clone)]
struct ComponentHealth {
    name: String,
    status: HealthStatus,
    message: String,
    recommendation: Option<String>,
}

impl ComponentHealth {
    fn new(name: String, status: HealthStatus, message: String) -> Self {
        Self {
            name,
            status,
            message,
            recommendation: None,
        }
    }

    fn with_recommendation(mut self, recommendation: String) -> Self {
        self.recommendation = Some(recommendation);
        self
    }
}

/// Check CPU health
fn check_cpu(metrics: &SystemMetrics) -> ComponentHealth {
    let usage = metrics.cpu_usage_percent;
    if usage > 90.0 {
        ComponentHealth::new(
            "CPU".to_string(),
            HealthStatus::Critical,
            format!("CPU usage is critically high: {:.1}%", usage),
        )
        .with_recommendation("Check for runaway processes or high system load".to_string())
    } else if usage > 70.0 {
        ComponentHealth::new(
            "CPU".to_string(),
            HealthStatus::Warning,
            format!("CPU usage is high: {:.1}%", usage),
        )
        .with_recommendation("Monitor CPU-intensive processes".to_string())
    } else {
        ComponentHealth::new(
            "CPU".to_string(),
            HealthStatus::Healthy,
            format!("CPU usage is normal: {:.1}%", usage),
        )
    }
}

/// Check memory health
fn check_memory(metrics: &SystemMetrics) -> ComponentHealth {
    let usage = metrics.memory_usage_percent();
    if usage > 95.0 {
        ComponentHealth::new(
            "Memory".to_string(),
            HealthStatus::Critical,
            format!(
                "Memory usage is critically high: {:.1}% ({}/{})",
                usage,
                format_size(metrics.memory_used_bytes, DECIMAL),
                format_size(metrics.memory_total_bytes, DECIMAL)
            ),
        )
        .with_recommendation("Close applications or restart to free memory".to_string())
    } else if usage > 85.0 {
        ComponentHealth::new(
            "Memory".to_string(),
            HealthStatus::Warning,
            format!(
                "Memory usage is high: {:.1}% ({}/{})",
                usage,
                format_size(metrics.memory_used_bytes, DECIMAL),
                format_size(metrics.memory_total_bytes, DECIMAL)
            ),
        )
        .with_recommendation("Consider closing unused applications".to_string())
    } else {
        ComponentHealth::new(
            "Memory".to_string(),
            HealthStatus::Healthy,
            format!(
                "Memory usage is normal: {:.1}% ({}/{})",
                usage,
                format_size(metrics.memory_used_bytes, DECIMAL),
                format_size(metrics.memory_total_bytes, DECIMAL)
            ),
        )
    }
}

/// Check disk health
fn check_disk(metrics: &SystemMetrics) -> ComponentHealth {
    let usage = metrics.disk_usage_percent();
    if usage > 95.0 {
        ComponentHealth::new(
            "Disk".to_string(),
            HealthStatus::Critical,
            format!(
                "Disk space is critically low: {:.1}% used ({}/{})",
                usage,
                format_size(metrics.disk_used_bytes, DECIMAL),
                format_size(metrics.disk_total_bytes, DECIMAL)
            ),
        )
        .with_recommendation(
            "Free up disk space immediately - run 'dragonfly disk analyze' to find large files"
                .to_string(),
        )
    } else if usage > 85.0 {
        ComponentHealth::new(
            "Disk".to_string(),
            HealthStatus::Warning,
            format!(
                "Disk space is low: {:.1}% used ({}/{})",
                usage,
                format_size(metrics.disk_used_bytes, DECIMAL),
                format_size(metrics.disk_total_bytes, DECIMAL)
            ),
        )
        .with_recommendation("Consider cleaning up files - run 'dragonfly clean --dry-run' to see what can be cleaned".to_string())
    } else {
        ComponentHealth::new(
            "Disk".to_string(),
            HealthStatus::Healthy,
            format!(
                "Disk space is adequate: {:.1}% used ({}/{})",
                usage,
                format_size(metrics.disk_used_bytes, DECIMAL),
                format_size(metrics.disk_total_bytes, DECIMAL)
            ),
        )
    }
}

/// Check swap health
fn check_swap(metrics: &SystemMetrics) -> ComponentHealth {
    if metrics.swap_total_bytes == 0 {
        return ComponentHealth::new(
            "Swap".to_string(),
            HealthStatus::Healthy,
            "Swap is not configured".to_string(),
        );
    }

    let usage = (metrics.swap_used_bytes as f32 / metrics.swap_total_bytes as f32) * 100.0;
    if usage > 50.0 {
        ComponentHealth::new(
            "Swap".to_string(),
            HealthStatus::Warning,
            format!(
                "Swap usage is high: {:.1}% ({}/{})",
                usage,
                format_size(metrics.swap_used_bytes, DECIMAL),
                format_size(metrics.swap_total_bytes, DECIMAL)
            ),
        )
        .with_recommendation(
            "High swap usage indicates memory pressure - consider adding more RAM".to_string(),
        )
    } else {
        ComponentHealth::new(
            "Swap".to_string(),
            HealthStatus::Healthy,
            format!(
                "Swap usage is normal: {:.1}% ({}/{})",
                usage,
                format_size(metrics.swap_used_bytes, DECIMAL),
                format_size(metrics.swap_total_bytes, DECIMAL)
            ),
        )
    }
}

/// Run health checks for all components
fn run_health_checks(metrics: &SystemMetrics, component: Option<&str>) -> Vec<ComponentHealth> {
    let mut checks = Vec::new();

    match component {
        Some("cpu") | None => checks.push(check_cpu(metrics)),
        _ => {}
    }
    match component {
        Some("memory") | None => checks.push(check_memory(metrics)),
        _ => {}
    }
    match component {
        Some("disk") | None => checks.push(check_disk(metrics)),
        _ => {}
    }
    match component {
        Some("swap") | None => checks.push(check_swap(metrics)),
        _ => {}
    }

    checks
}

pub async fn handle_health(
    json: bool,
    recommend: bool,
    component: Option<String>,
    global_json: bool,
) -> Result<()> {
    let output_json = json || global_json;
    let mut collector = MetricsCollector::new();
    let metrics = collector.collect().await?;

    let component_filter = component.as_deref();
    let health_checks = run_health_checks(&metrics, component_filter);

    if output_json {
        let checks_json: Vec<serde_json::Value> = health_checks
            .iter()
            .map(|check| {
                let mut obj = json!({
                    "component": check.name,
                    "status": match check.status {
                        HealthStatus::Healthy => "healthy",
                        HealthStatus::Warning => "warning",
                        HealthStatus::Critical => "critical",
                    },
                    "message": check.message,
                });
                if recommend && check.recommendation.is_some() {
                    obj["recommendation"] = json!(check.recommendation);
                }
                obj
            })
            .collect();

        let overall_status = if health_checks
            .iter()
            .any(|c| c.status == HealthStatus::Critical)
        {
            "critical"
        } else if health_checks
            .iter()
            .any(|c| c.status == HealthStatus::Warning)
        {
            "warning"
        } else {
            "healthy"
        };

        let json_output = json!({
            "status": "ok",
            "overall_status": overall_status,
            "components": checks_json,
            "metrics": {
                "cpu_usage_percent": metrics.cpu_usage_percent,
                "memory_usage_percent": metrics.memory_usage_percent(),
                "disk_usage_percent": metrics.disk_usage_percent(),
                "timestamp": metrics.timestamp
            }
        });
        println!("{}", serde_json::to_string_pretty(&json_output)?);
        return Ok(());
    }

    // Human-readable output
    println!("{}", "System Health Check".bold().bright_cyan());
    if let Some(ref comp) = component {
        println!("Component: {}", comp);
    } else {
        println!("Component: All");
    }
    println!();

    let mut has_issues = false;
    for check in &health_checks {
        let status_icon = match check.status {
            HealthStatus::Healthy => "✅".green(),
            HealthStatus::Warning => "⚠️ ".yellow(),
            HealthStatus::Critical => "❌".red(),
        };
        let status_text = match check.status {
            HealthStatus::Healthy => "Healthy".green(),
            HealthStatus::Warning => "Warning".yellow(),
            HealthStatus::Critical => "Critical".red(),
        };

        println!("{} {}: {}", status_icon, check.name.bold(), status_text);
        println!("   {}", check.message.dimmed());
        if recommend {
            if let Some(ref rec) = check.recommendation {
                println!("   {} {}", "💡 Recommendation:".cyan(), rec);
            }
        }
        println!();

        if check.status != HealthStatus::Healthy {
            has_issues = true;
        }
    }

    if !has_issues {
        println!("{}", "All systems operational!".green().bold());
    } else if recommend {
        println!(
            "{}",
            "Run with --recommend to see suggestions for improving system health.".dimmed()
        );
    }

    Ok(())
}
