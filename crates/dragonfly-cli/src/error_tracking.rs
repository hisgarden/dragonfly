//! Generic Error Tracking Adapter
//!
//! This module provides a generic adapter for error tracking that supports
//! multiple backends: Sentry.io and GlitchTip (Sentry API-compatible).
//!
//! Since GlitchTip is Sentry API-compatible, we use the Sentry SDK for both,
//! but with different DSN configurations and detection logic.

use sentry::{init, ClientInitGuard};
use std::borrow::Cow;
use std::env;

/// Error tracking backend type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ErrorTrackingBackend {
    /// Sentry.io (sentry.io)
    Sentry,
    /// GlitchTip (self-hosted Sentry-compatible)
    GlitchTip,
    /// Auto-detect based on DSN URL
    #[default]
    Auto,
}

impl ErrorTrackingBackend {
    /// Detect backend type from DSN URL
    pub fn from_dsn(dsn: &str) -> Self {
        let dsn_lower = dsn.to_lowercase();
        if dsn_lower.contains("sentry.io") || dsn_lower.contains("ingest.sentry.io") {
            ErrorTrackingBackend::Sentry
        } else if dsn_lower.contains("glitchtip")
            || dsn_lower.contains("localhost:8000")
            || dsn_lower.contains("127.0.0.1:8000")
            || dsn_lower.contains("glitchtip.com")
            || (dsn_lower.contains("localhost") && dsn_lower.contains(":8000"))
        {
            ErrorTrackingBackend::GlitchTip
        } else {
            // Default to auto if we can't determine
            ErrorTrackingBackend::Auto
        }
    }

    /// Get display name for the backend
    pub fn display_name(&self) -> &'static str {
        match self {
            ErrorTrackingBackend::Sentry => "Sentry.io",
            ErrorTrackingBackend::GlitchTip => "GlitchTip",
            ErrorTrackingBackend::Auto => "Auto-detected",
        }
    }
}

/// Error tracking configuration
#[derive(Debug, Clone)]
pub struct ErrorTrackingConfig {
    /// DSN (Data Source Name) URL
    pub dsn: Option<String>,
    /// Backend type (auto-detected if not specified)
    pub backend: ErrorTrackingBackend,
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
            backend: ErrorTrackingBackend::Auto,
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

/// Initialize error tracking with the given configuration
///
/// This function supports both Sentry.io and GlitchTip backends.
/// Since GlitchTip is Sentry API-compatible, we use the same Sentry SDK
/// for both, but with different DSN configurations.
pub fn init_error_tracking(config: ErrorTrackingConfig) -> ClientInitGuard {
    let dsn = config.dsn.clone();
    let backend = if config.backend == ErrorTrackingBackend::Auto {
        dsn.as_ref()
            .map(|d| ErrorTrackingBackend::from_dsn(d))
            .unwrap_or(ErrorTrackingBackend::Auto)
    } else {
        config.backend
    };

    // Log which backend is being used (if DSN is configured)
    if let Some(ref dsn) = dsn {
        // Mask sensitive parts of DSN for logging
        let masked_dsn = mask_dsn(dsn);
        tracing::info!(
            backend = %backend.display_name(),
            dsn = %masked_dsn,
            environment = %config.environment,
            release = %config.release,
            "Initializing error tracking"
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
/// 2. `SENTRY_DSN` environment variable (for backward compatibility)
/// 3. `.sentryclirc` or `.glitchtiprc` config file
/// 4. `ERROR_TRACKING_BACKEND` environment variable (optional, defaults to Auto)
pub fn load_config() -> ErrorTrackingConfig {
    // Check for DSN in environment variables (priority order)
    let dsn = env::var("ERROR_TRACKING_DSN")
        .ok()
        .or_else(|| env::var("SENTRY_DSN").ok()); // Backward compatibility

    // Check for backend type
    let backend = env::var("ERROR_TRACKING_BACKEND")
        .ok()
        .and_then(|b| match b.to_lowercase().as_str() {
            "sentry" => Some(ErrorTrackingBackend::Sentry),
            "glitchtip" => Some(ErrorTrackingBackend::GlitchTip),
            "auto" => Some(ErrorTrackingBackend::Auto),
            _ => None,
        })
        .unwrap_or(ErrorTrackingBackend::Auto);

    // If not in env, try reading from config files
    let dsn = dsn.or_else(|| {
        // Try .sentryclirc first (for backward compatibility)
        std::fs::read_to_string(".sentryclirc")
            .ok()
            .and_then(|content| extract_dsn_from_config(&content))
            .or_else(|| {
                // Try .glitchtiprc (GlitchTip-specific config)
                std::fs::read_to_string(".glitchtiprc")
                    .ok()
                    .and_then(|content| extract_dsn_from_config(&content))
            })
    });

    ErrorTrackingConfig {
        dsn,
        backend,
        ..Default::default()
    }
}

/// Extract DSN from config file content
///
/// Supports both `.sentryclirc` and `.glitchtiprc` formats:
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
    fn test_backend_detection_sentry() {
        let dsn = "https://key@o123.ingest.sentry.io/456";
        assert_eq!(
            ErrorTrackingBackend::from_dsn(dsn),
            ErrorTrackingBackend::Sentry
        );
    }

    #[test]
    fn test_backend_detection_glitchtip() {
        let dsn = "https://key@glitchtip.example.com/456";
        assert_eq!(
            ErrorTrackingBackend::from_dsn(dsn),
            ErrorTrackingBackend::GlitchTip
        );
    }

    #[test]
    fn test_backend_detection_localhost() {
        let dsn = "https://key@localhost:8000/456";
        assert_eq!(
            ErrorTrackingBackend::from_dsn(dsn),
            ErrorTrackingBackend::GlitchTip
        );
    }

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
