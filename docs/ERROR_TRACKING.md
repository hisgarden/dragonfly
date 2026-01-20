# GlitchTip Error Tracking Integration Guide

DragonFly integrates with **GlitchTip**, a self-hosted, privacy-first error tracking service that is Sentry API-compatible.

> **Privacy-First**: All error data stays local/private. No data is sent to external cloud services.

## Quick Start

### Using GlitchTip

1. **Set DSN via environment variable:**
   ```bash
   export ERROR_TRACKING_DSN="https://YOUR_KEY@YOUR_GLITCHTIP_HOST/PROJECT_ID"
   # For local development:
   export ERROR_TRACKING_DSN="https://YOUR_KEY@localhost:8000/PROJECT_ID"
   ./target/debug/dragonfly --enable-error-tracking health
   ```

2. **Or use configuration file:**
   ```bash
   cp .glitchtiprc.example .glitchtiprc
   # Edit .glitchtiprc with your GlitchTip credentials
   ./target/debug/dragonfly --enable-error-tracking health
   ```

## Configuration Priority

The error tracking adapter checks for configuration in this order:

1. `ERROR_TRACKING_DSN` environment variable (highest priority)
2. `.glitchtiprc` configuration file

## Configuration Files

### `.glitchtiprc` Format

```ini
[defaults]
org=YOUR_ORG_SLUG
project=dragonfly
url=https://YOUR_KEY@YOUR_GLITCHTIP_HOST/PROJECT_ID
```

For local development:
```ini
[defaults]
org=my-org
project=dragonfly
url=https://YOUR_KEY@localhost:8000/PROJECT_ID
```

## Getting Your DSN

### GlitchTip

1. Log into your GlitchTip instance
2. Navigate to your project settings
3. Go to "Client Keys (DSN)"
4. Copy the DSN URL

For local development with GlitchTip running on `localhost:8000`:
- The DSN format is: `https://YOUR_KEY@localhost:8000/PROJECT_ID`
- Make sure your GlitchTip instance is running and accessible

## Privacy & Security

- **No PII by default**: `send_default_pii: false` ensures no personal data is sent
- **Local-first**: All error data stays on your self-hosted GlitchTip instance
- **No cloud services**: No data is sent to external cloud providers
- **Configurable**: Can be disabled by not setting DSN or not using `--enable-error-tracking`
- **Secure**: Uses HTTPS for all communication
- **Config files are gitignored**: `.glitchtiprc` is in `.gitignore`

## Features

GlitchTip error tracking supports:

- ✅ Automatic error capture
- ✅ Stack traces
- ✅ Breadcrumbs (up to 100)
- ✅ Release tracking (e.g., `dragonfly@0.1.0`)
- ✅ Environment detection (development/production)
- ✅ Performance monitoring (10% sample rate in production, 100% in dev)

## Testing

### Test with GlitchTip (Local)

```bash
# Make sure GlitchTip is running on localhost:8000
export ERROR_TRACKING_DSN="https://YOUR_KEY@localhost:8000/PROJECT_ID"
./target/debug/dragonfly --enable-error-tracking health
```

### Verify Errors are Captured

1. Trigger an error (e.g., run an invalid command)
2. Check your GlitchTip dashboard:
   - GlitchTip: http://YOUR_GLITCHTIP_HOST/organizations/YOUR_ORG/projects/YOUR_PROJECT/

## Disabling Error Tracking

If you don't want to use error tracking:

1. **Don't use the flag**: Simply don't pass `--enable-error-tracking`
2. **Don't set DSN**: Don't configure `ERROR_TRACKING_DSN` or config files
3. **No-op mode**: If DSN is not configured, error tracking runs in no-op mode

## Troubleshooting

### Errors Not Appearing

1. **Check DSN**: Verify your DSN is correct and accessible
2. **Check flag**: Ensure `--enable-error-tracking` is set
3. **Check network**: Verify connectivity to your GlitchTip instance
4. **Check logs**: Look for debug messages about backend initialization

### GlitchTip Connection Issues

For local GlitchTip development:

1. Ensure GlitchTip backend is running: `docker compose up` (or `nerdctl compose up`)
2. Verify the DSN uses the correct host: `localhost:8000` or your actual host
3. Check GlitchTip logs for connection attempts

## Best Practices

1. **Use environment variables in production**: More secure than config files
2. **Use config files for development**: Easier to manage locally
3. **Set appropriate sample rates**: 10% is usually sufficient for performance monitoring
4. **Review errors regularly**: Check your GlitchTip dashboard
5. **Tag releases**: Errors are automatically tagged with version numbers
6. **Filter noise**: Configure filters in your GlitchTip dashboard

## API Compatibility

Since GlitchTip is Sentry API-compatible, DragonFly uses the Sentry SDK to communicate with GlitchTip. This provides:

- Full feature compatibility
- Same error format
- Same DSN format
- Seamless integration

All error data stays on your self-hosted GlitchTip instance - no external cloud services are involved.
