# Sentry Error Tracking Setup

DragonFly integrates with [Sentry.io](https://sentry.io) for error tracking and monitoring. This helps identify issues in production and provides insights into application health.

## Features

- **Automatic Error Capture**: All errors are automatically reported to Sentry
- **Privacy-First**: No PII (Personally Identifiable Information) is sent by default
- **Environment Detection**: Automatically detects development vs production
- **Release Tracking**: Tracks errors by application version
- **Performance Monitoring**: Samples 10% of transactions in production

## Setup

### Option 1: Environment Variable (Recommended)

Set the `SENTRY_DSN` environment variable:

```bash
export SENTRY_DSN="https://YOUR_KEY@oYOUR_ORG_ID.ingest.sentry.io/YOUR_PROJECT_ID"
```

### Option 2: Configuration File

1. Copy the example configuration:
   ```bash
   cp .sentryclirc.example .sentryclirc
   ```

2. Edit `.sentryclirc` and fill in your Sentry credentials:
   ```ini
   [auth]
   token=YOUR_SENTRY_AUTH_TOKEN_HERE

   [defaults]
   org=YOUR_ORG_SLUG
   project=YOUR_PROJECT_SLUG
   url=https://YOUR_KEY@oYOUR_ORG_ID.ingest.sentry.io/YOUR_PROJECT_ID
   ```

## Getting Your Sentry Credentials

1. **DSN (Data Source Name)**:
   - Go to: https://sentry.io/settings/YOUR_ORG/projects/YOUR_PROJECT/keys/
   - Copy the DSN (format: `https://KEY@oORG_ID.ingest.sentry.io/PROJECT_ID`)

2. **Auth Token** (for sentry-cli):
   - Go to: https://sentry.io/settings/account/api/auth-tokens/
   - Create a new token with `project:read` and `project:write` scopes

3. **Organization & Project**:
   - Organization slug: Found in your Sentry URL
   - Project slug: Found in your Sentry project settings

## Privacy & Security

- **No PII by default**: `send_default_pii: false` ensures no personal data is sent
- **Local-first**: All processing happens locally; only errors are reported
- **Configurable**: Can be disabled by not setting DSN
- **Secure**: Uses HTTPS for all communication

## How It Works

1. **Initialization**: Sentry initializes at application startup
2. **Error Capture**: When an error occurs, it's automatically captured
3. **Context**: Error includes stack trace, environment, and release version
4. **Reporting**: Errors are sent to Sentry dashboard for analysis

## Testing

To test Sentry integration:

```bash
# Set your DSN
export SENTRY_DSN="https://YOUR_KEY@oYOUR_ORG_ID.ingest.sentry.io/YOUR_PROJECT_ID"

# Run the application
./target/debug/dragonfly health

# Check Sentry dashboard for events
```

## Disabling Sentry

If you don't want to use Sentry:

1. **Don't set DSN**: Simply don't configure `SENTRY_DSN` or `.sentryclirc`
2. **No-op mode**: Sentry will run in no-op mode and won't send any data

## Integration with CI/CD

For CI/CD pipelines, use environment variables:

```yaml
# GitHub Actions example
env:
  SENTRY_DSN: ${{ secrets.SENTRY_DSN }}
```

## Debug Symbols Upload (Optional)

For better stack traces, upload debug symbols:

```bash
# Install sentry-cli
brew install getsentry/tools/sentry-cli

# Configure authentication
sentry-cli login

# Upload debug symbols (for release builds)
sentry-cli debug-files upload target/release/
```

## Monitoring

Once configured, you can monitor:

- **Error rates**: Track error frequency over time
- **Error types**: Categorize and prioritize issues
- **Affected users**: See how many users encounter errors
- **Performance**: Monitor slow operations
- **Releases**: Track errors by version

## Best Practices

1. **Use environment variables in production**: More secure than config files
2. **Set appropriate sample rates**: 10% is usually sufficient for performance monitoring
3. **Review errors regularly**: Check Sentry dashboard for new issues
4. **Tag releases**: Sentry automatically tags errors with version numbers
5. **Filter noise**: Configure filters in Sentry to ignore expected errors
