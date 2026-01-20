# Error Tracking Integration Guide

DragonFly supports multiple error tracking backends through a generic adapter:

- **Sentry.io** - Cloud-hosted error tracking service
- **GlitchTip** - Self-hosted, Sentry API-compatible error tracking

Both backends use the Sentry SDK since GlitchTip is Sentry API-compatible.

## Quick Start

### Using Sentry.io

1. **Set DSN via environment variable:**
   ```bash
   export ERROR_TRACKING_DSN="https://KEY@oORG_ID.ingest.sentry.io/PROJECT_ID"
   ./target/debug/dragonfly --enable-error-tracking health
   ```

2. **Or use configuration file:**
   ```bash
   cp .sentryclirc.example .sentryclirc
   # Edit .sentryclirc with your Sentry credentials
   ./target/debug/dragonfly --enable-error-tracking health
   ```

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

3. **Explicitly specify backend (optional):**
   ```bash
   export ERROR_TRACKING_BACKEND="glitchtip"
   export ERROR_TRACKING_DSN="https://YOUR_KEY@localhost:8000/PROJECT_ID"
   ```

## Configuration Priority

The error tracking adapter checks for configuration in this order:

1. `ERROR_TRACKING_DSN` environment variable (highest priority)
2. `SENTRY_DSN` environment variable (backward compatibility)
3. `.glitchtiprc` configuration file
4. `.sentryclirc` configuration file (backward compatibility)
5. `ERROR_TRACKING_BACKEND` environment variable (optional, defaults to Auto)

## Backend Auto-Detection

The adapter automatically detects which backend to use based on the DSN URL:

- URLs containing `sentry.io` → Sentry.io
- URLs containing `glitchtip` or `localhost:8000` → GlitchTip
- Otherwise → Auto (will attempt to use the DSN as-is)

You can also explicitly set the backend:

```bash
export ERROR_TRACKING_BACKEND="sentry"    # Force Sentry.io
export ERROR_TRACKING_BACKEND="glitchtip" # Force GlitchTip
export ERROR_TRACKING_BACKEND="auto"      # Auto-detect (default)
```

## Configuration Files

### `.sentryclirc` Format

```ini
[auth]
token=YOUR_SENTRY_AUTH_TOKEN_HERE

[defaults]
org=YOUR_ORG_SLUG
project=dragonfly
url=https://KEY@oORG_ID.ingest.sentry.io/PROJECT_ID
```

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

### Sentry.io

1. Go to: https://sentry.io/settings/YOUR_ORG/projects/YOUR_PROJECT/keys/
2. Copy the DSN (format: `https://KEY@oORG_ID.ingest.sentry.io/PROJECT_ID`)

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
- **Local-first**: All processing happens locally; only errors are reported
- **Configurable**: Can be disabled by not setting DSN or not using `--enable-error-tracking`
- **Secure**: Uses HTTPS for all communication
- **Config files are gitignored**: `.sentryclirc` and `.glitchtiprc` are in `.gitignore`

## Features

Both backends support:

- ✅ Automatic error capture
- ✅ Stack traces
- ✅ Breadcrumbs (up to 100)
- ✅ Release tracking (e.g., `dragonfly@0.1.0`)
- ✅ Environment detection (development/production)
- ✅ Performance monitoring (10% sample rate in production, 100% in dev)

## Testing

### Test with Sentry.io

```bash
export ERROR_TRACKING_DSN="https://YOUR_KEY@oORG_ID.ingest.sentry.io/PROJECT_ID"
./target/debug/dragonfly --enable-error-tracking health
```

### Test with GlitchTip (Local)

```bash
# Make sure GlitchTip is running on localhost:8000
export ERROR_TRACKING_DSN="https://YOUR_KEY@localhost:8000/PROJECT_ID"
./target/debug/dragonfly --enable-error-tracking health
```

### Verify Errors are Captured

1. Trigger an error (e.g., run an invalid command)
2. Check your error tracking dashboard:
   - Sentry.io: https://sentry.io/organizations/YOUR_ORG/projects/YOUR_PROJECT/
   - GlitchTip: http://YOUR_GLITCHTIP_HOST/organizations/YOUR_ORG/projects/YOUR_PROJECT/

## Disabling Error Tracking

If you don't want to use error tracking:

1. **Don't use the flag**: Simply don't pass `--enable-error-tracking`
2. **Don't set DSN**: Don't configure `ERROR_TRACKING_DSN` or config files
3. **No-op mode**: If DSN is not configured, error tracking runs in no-op mode

## Migration from Sentry.io to GlitchTip

If you're currently using Sentry.io and want to switch to GlitchTip:

1. **Update your DSN:**
   ```bash
   export ERROR_TRACKING_DSN="https://YOUR_KEY@YOUR_GLITCHTIP_HOST/PROJECT_ID"
   ```

2. **Or update config file:**
   ```bash
   cp .glitchtiprc.example .glitchtiprc
   # Edit .glitchtiprc with your GlitchTip DSN
   ```

3. **Optionally set backend explicitly:**
   ```bash
   export ERROR_TRACKING_BACKEND="glitchtip"
   ```

The adapter will automatically detect GlitchTip from the DSN URL, so explicit backend setting is optional.

## Troubleshooting

### Errors Not Appearing

1. **Check DSN**: Verify your DSN is correct and accessible
2. **Check flag**: Ensure `--enable-error-tracking` is set
3. **Check network**: Verify connectivity to your error tracking server
4. **Check logs**: Look for debug messages about backend initialization

### Backend Not Detected Correctly

If auto-detection fails, explicitly set the backend:

```bash
export ERROR_TRACKING_BACKEND="glitchtip"  # or "sentry"
```

### GlitchTip Connection Issues

For local GlitchTip development:

1. Ensure GlitchTip backend is running: `docker compose up` (or `nerdctl compose up`)
2. Verify the DSN uses the correct host: `localhost:8000` or your actual host
3. Check GlitchTip logs for connection attempts

## Best Practices

1. **Use environment variables in production**: More secure than config files
2. **Use config files for development**: Easier to manage locally
3. **Set appropriate sample rates**: 10% is usually sufficient for performance monitoring
4. **Review errors regularly**: Check your error tracking dashboard
5. **Tag releases**: Errors are automatically tagged with version numbers
6. **Filter noise**: Configure filters in your error tracking dashboard

## API Compatibility

Since GlitchTip is Sentry API-compatible, you can use the same Sentry SDK for both backends. The adapter handles:

- DSN format differences
- Backend detection
- Configuration loading
- Error reporting

No code changes are needed when switching between Sentry.io and GlitchTip - just update the DSN!
