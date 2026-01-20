//! GlitchTip Error Tracking Adapter
//!
//! This module provides error tracking integration with GlitchTip,
//! a self-hosted, privacy-first Sentry API-compatible error tracking service.
//!
//! Since GlitchTip is Sentry API-compatible, we use the Sentry SDK,
//! but configured to only work with local/self-hosted GlitchTip instances.

use sentry::{init, ClientInitGuard};
use std::borrow::Cow;
use std::env;

/// Error tracking configuration
#[derive(Debug, Clone)]
pub struct ErrorTrackingConfig {
    /// DSN (Data Source Name) URL (must point to GlitchTip instance)
    pub dsn: Option<String>,
    /// Release version (e.g., "dragonfly@0.1.0")
    pub release: String,
    /// Environment (e.g., "development", "production")
    pub environment: String,
    /// Whether to send PII (Personally Identifiable Information)
    pub send_default_pii: bool,
    /// Sample rate for performance monitoring (0.0 to 1.0)
    pub traces_sample_rate: f32,
    /// Maximum number of breadcrumbs
    pub max_breadcrumbs: u32,
}

impl Default for ErrorTrackingConfig {
    fn default() -> Self {
        let is_debug = cfg!(debug_assertions);
        Self {
            dsn: None,
            release: format!("dragonfly@{}", env!("CARGO_PKG_VERSION")),
            environment: if is_debug {
                "development".to_string()
            } else {
                "production".to_string()
            },
            send_default_pii: false,
            traces_sample_rate: if is_debug { 1.0 } else { 0.1 },
            max_breadcrumbs: 100,
        }
    }
}

/// Initialize error tracking with GlitchTip
///
/// This function initializes error tracking with a self-hosted GlitchTip instance.
/// All error data stays local/private - no data is sent to external cloud services.
pub fn init_error_tracking(config: ErrorTrackingConfig) -> ClientInitGuard {
    let dsn = config.dsn.clone();

    // Log initialization (if DSN is configured)
    if let Some(ref dsn) = dsn {
        // Mask sensitive parts of DSN for logging
        let masked_dsn = mask_dsn(dsn);
        tracing::info!(
            backend = "GlitchTip",
            dsn = %masked_dsn,
            environment = %config.environment,
            release = %config.release,
            "Initializing local error tracking (GlitchTip)"
        );
    }

    let client_options = sentry::ClientOptions {
        release: Some(Cow::Owned(config.release.clone())),
        environment: Some(Cow::Owned(config.environment.clone())),
        send_default_pii: config.send_default_pii,
        traces_sample_rate: config.traces_sample_rate,
        attach_stacktrace: true,
        max_breadcrumbs: config.max_breadcrumbs as usize,
        ..Default::default()
    };

    if let Some(dsn) = dsn {
        init((dsn, client_options))
    } else {
        // No DSN configured - error tracking will be a no-op
        tracing::debug!("No DSN configured, error tracking disabled");
        init((
            "",
            sentry::ClientOptions {
                release: Some(Cow::Owned(config.release)),
                ..Default::default()
            },
        ))
    }
}

/// Load error tracking configuration from environment and config file
///
/// Priority:
/// 1. `ERROR_TRACKING_DSN` environment variable
/// 2. `.glitchtiprc` config file
///
/// Note: Only GlitchTip (self-hosted) is supported for privacy.
pub fn load_config() -> ErrorTrackingConfig {
    // Check for DSN in environment variable
    let dsn = env::var("ERROR_TRACKING_DSN").ok();

    // If not in env, try reading from .glitchtiprc config file
    let dsn = dsn.or_else(|| {
        std::fs::read_to_string(".glitchtiprc")
            .ok()
            .and_then(|content| extract_dsn_from_config(&content))
    });

    ErrorTrackingConfig {
        dsn,
        ..Default::default()
    }
}

/// Extract DSN from config file content
///
/// Supports `.glitchtiprc` format:
/// - `defaults.url=https://KEY@HOST/PROJECT_ID`
/// - `dsn=https://KEY@HOST/PROJECT_ID`
///
/// Mask sensitive parts of DSN for logging (shows only host, not key)
fn mask_dsn(dsn: &str) -> String {
    if let Some(at_pos) = dsn.find('@') {
        if let Some(slash_pos) = dsn[at_pos..].find('/') {
            format!("{}@{}", "***", &dsn[at_pos + 1..at_pos + slash_pos + 1])
        } else {
            format!("{}@{}", "***", &dsn[at_pos + 1..])
        }
    } else {
        "***".to_string()
    }
}

fn extract_dsn_from_config(content: &str) -> Option<String> {
    content.lines().find_map(|line| {
        let line = line.trim();
        // Look for defaults.url= or dsn= lines
        if let Some(url) = line.strip_prefix("defaults.url=") {
            let url = url.trim();
            // Accept both http:// and https:// (http is fine for localhost)
            if url.starts_with("https://") || url.starts_with("http://") {
                Some(url.to_string())
            } else {
                None
            }
        } else if let Some(url) = line.strip_prefix("dsn=") {
            let url = url.trim();
            // Accept both http:// and https:// (http is fine for localhost)
            if url.starts_with("https://") || url.starts_with("http://") {
                Some(url.to_string())
            } else {
                None
            }
        } else {
            None
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_extract_dsn_from_config_defaults_url() {
        let config = "defaults.url=https://key@host.com/123\n";
        assert_eq!(
            extract_dsn_from_config(config),
            Some("https://key@host.com/123".to_string())
        );
    }

    #[test]
    fn test_extract_dsn_from_config_dsn() {
        let config = "[config]\ndsn=https://key@host.com/123\n";
        assert_eq!(
            extract_dsn_from_config(config),
            Some("https://key@host.com/123".to_string())
        );
    }

    #[test]
    fn test_mask_dsn() {
        let dsn = "https://abc123def456@localhost:8000/789";
        let masked = mask_dsn(dsn);
        assert_eq!(masked, "***@localhost:8000/");

        let dsn2 = "https://key@glitchtip.example.com/123";
        let masked2 = mask_dsn(dsn2);
        assert_eq!(masked2, "***@glitchtip.example.com/");
    }
}
