# Sentry Integration Guide

## ✅ Current Status

Sentry.io error tracking is **fully integrated** into DragonFly.

### Configuration

- **DSN**: Configured in `.sentryclirc` (gitignored for security)
- **Organization**: `hollowplanet`
- **Project**: `dragonfly`
- **DSN URL**: `https://464241c1134e8e304fe7f0173d904b21@o4510738128502784.ingest.us.sentry.io/4510740361576448`

## How It Works

1. **Automatic Initialization**: Sentry initializes at application startup
2. **DSN Detection**: Reads from:
   - `SENTRY_DSN` environment variable (priority)
   - `.sentryclirc` file `defaults.url` field (fallback)
3. **Error Capture**: All `anyhow::Error` instances are automatically captured
4. **Privacy**: No PII sent by default (`send_default_pii: false`)

## Testing Sentry Integration

### Option 1: Test with Environment Variable

```bash
export SENTRY_DSN="https://464241c1134e8e304fe7f0173d904b21@o4510738128502784.ingest.us.sentry.io/4510740361576448"
./target/debug/dragonfly health
```

### Option 2: Test with Config File

The `.sentryclirc` file is already configured. Just run:

```bash
./target/debug/dragonfly health
```

### Option 3: Trigger a Test Error

Create a test command that intentionally fails to verify error capture:

```bash
# This will trigger an error that should appear in Sentry
./target/debug/dragonfly invalid-command 2>&1
```

## Verifying in Sentry Dashboard

1. Go to: https://sentry.io/organizations/hollowplanet/projects/dragonfly/
2. Check the "Issues" tab for new errors
3. Verify:
   - Release: `dragonfly@0.1.0`
   - Environment: `development` (for debug builds) or `production` (for release builds)
   - Stack traces are included
   - Breadcrumbs are captured

## Next Steps

### 1. Add Auth Token (Optional - for sentry-cli)

If you want to upload debug symbols or use `sentry-cli`:

```bash
# Edit .sentryclirc and add your auth token
# Get token from: https://sentry.io/settings/account/api/auth-tokens/
```

### 2. Test Error Reporting

Create a simple test to verify errors are captured:

```rust
// In a command handler, test error capture:
if let Err(e) = some_operation() {
    // This will automatically be sent to Sentry
    return Err(e);
}
```

### 3. Add Custom Context (Optional)

You can add custom context to Sentry events:

```rust
use sentry::configure_scope;

configure_scope(|scope| {
    scope.set_tag("command", "health");
    scope.set_context("user", {
        let mut map = std::collections::BTreeMap::new();
        map.insert("os".to_string(), "macos".into());
        map
    });
});
```

### 4. Performance Monitoring

Sentry is configured with:
- **Development**: 100% transaction sampling
- **Production**: 10% transaction sampling

This helps track slow operations without overwhelming Sentry.

## Privacy Considerations

✅ **No PII by default**: `send_default_pii: false`  
✅ **Local-first**: All processing happens locally  
✅ **Opt-in**: Only errors are reported, no user data  
✅ **Secure**: HTTPS-only communication  

## Troubleshooting

### Sentry Not Capturing Errors?

1. **Check DSN**: Verify `.sentryclirc` has correct `url` field
2. **Check Environment**: Ensure DSN is being read (check logs)
3. **Check Sentry Dashboard**: Look for connection issues
4. **Test Manually**: Use `sentry::capture_message("test", sentry::Level::Info)`

### No Events in Dashboard?

- Verify DSN is correct
- Check Sentry project settings
- Ensure network connectivity
- Check Sentry dashboard filters

## Integration Checklist

- [x] Sentry SDK added to dependencies
- [x] DSN configured in `.sentryclirc`
- [x] Error capture implemented
- [x] Privacy settings configured (no PII)
- [x] Environment detection (dev/prod)
- [x] Release tracking (version-based)
- [ ] Auth token added (optional)
- [ ] Debug symbols upload configured (optional)
- [ ] Custom context added (optional)
- [ ] Test error captured and verified in dashboard

## Useful Commands

```bash
# Build with Sentry
task build

# Run and test Sentry
./target/debug/dragonfly health

# Check Sentry dashboard
open https://sentry.io/organizations/hollowplanet/projects/dragonfly/

# Install sentry-cli (optional)
brew install getsentry/tools/sentry-cli
```
