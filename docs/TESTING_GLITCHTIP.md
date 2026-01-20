# Testing GlitchTip Integration

This guide helps you test the GlitchTip integration with DragonFly.

## Prerequisites

1. **GlitchTip Backend Running**: Ensure your GlitchTip instance is running
   ```bash
   cd /path/to/glitchtip-backend
   nerdctl compose -f compose.yml up -d
   ```

2. **Get Your DSN**: 
   - Log into GlitchTip UI (usually `http://localhost:8000`)
   - Create or select a project
   - Go to Project Settings → Client Keys (DSN)
   - Copy the DSN URL

## Quick Test

### 1. Set Environment Variable

```bash
export ERROR_TRACKING_DSN="https://YOUR_KEY@localhost:8000/PROJECT_ID"
```

### 2. Build DragonFly

```bash
cd /path/to/dragonfly
cargo build
```

### 3. Run with Error Tracking Enabled

```bash
./target/debug/dragonfly --enable-error-tracking health
```

### 4. Trigger a Test Error

```bash
# This should generate an error that gets sent to GlitchTip
./target/debug/dragonfly --enable-error-tracking invalid-command 2>&1
```

### 5. Verify in GlitchTip

1. Open GlitchTip UI: `http://localhost:8000`
2. Navigate to your project
3. Check the "Issues" tab
4. You should see the error you triggered

## Using Configuration File

### Option 1: Use .glitchtiprc

```bash
cp .glitchtiprc.example .glitchtiprc
# Edit .glitchtiprc with your DSN
./target/debug/dragonfly --enable-error-tracking health
```

### Option 2: Use .sentryclirc (Backward Compatible)

```bash
cp .sentryclirc.example .sentryclirc
# Edit .sentryclirc with your GlitchTip DSN
./target/debug/dragonfly --enable-error-tracking health
```

## Testing Different Scenarios

### Test Auto-Detection

The adapter should automatically detect GlitchTip from the DSN:

```bash
export ERROR_TRACKING_DSN="https://key@localhost:8000/123"
./target/debug/dragonfly --enable-error-tracking health
# Should log: "Initializing error tracking" with backend="GlitchTip"
```

### Test Explicit Backend Selection

```bash
export ERROR_TRACKING_BACKEND="glitchtip"
export ERROR_TRACKING_DSN="https://key@localhost:8000/123"
./target/debug/dragonfly --enable-error-tracking health
```

### Test Error Capture

Create a simple test that triggers an error:

```bash
# Invalid command should trigger an error
./target/debug/dragonfly --enable-error-tracking nonexistent-command 2>&1

# Check GlitchTip dashboard for the error
```

## Verification Checklist

- [ ] GlitchTip backend is running and accessible
- [ ] DSN is correctly configured
- [ ] `--enable-error-tracking` flag is used
- [ ] Error appears in GlitchTip dashboard
- [ ] Stack traces are included
- [ ] Release version is correct (`dragonfly@0.1.0`)
- [ ] Environment is correct (`development` or `production`)

## Troubleshooting

### Errors Not Appearing in GlitchTip

1. **Check DSN**: Verify the DSN is correct
   ```bash
   echo $ERROR_TRACKING_DSN
   ```

2. **Check GlitchTip Logs**:
   ```bash
   nerdctl logs glitchtip-backend-web-1 -f
   ```

3. **Check Network**: Ensure GlitchTip is accessible
   ```bash
   curl http://localhost:8000/api/
   ```

4. **Check DragonFly Logs**: Look for initialization messages
   ```bash
   RUST_LOG=debug ./target/debug/dragonfly --enable-error-tracking health
   ```

### Backend Not Detected

If auto-detection fails, explicitly set the backend:

```bash
export ERROR_TRACKING_BACKEND="glitchtip"
```

### Connection Refused

If you see connection errors:

1. Verify GlitchTip is running: `nerdctl compose ps`
2. Check the DSN host matches your GlitchTip URL
3. For localhost, ensure you're using `localhost:8000` not `127.0.0.1:8000` (or vice versa)

## Advanced Testing

### Test with Custom Release

```bash
export ERROR_TRACKING_DSN="https://key@localhost:8000/123"
./target/debug/dragonfly --enable-error-tracking health
# Check GlitchTip for release: "dragonfly@0.1.0"
```

### Test Performance Monitoring

Performance monitoring samples 10% of transactions in production, 100% in development:

```bash
# Development build (100% sample rate)
cargo build
./target/debug/dragonfly --enable-error-tracking health

# Release build (10% sample rate)
cargo build --release
./target/release/dragonfly --enable-error-tracking health
```

### Test Multiple Errors

Generate multiple errors to verify batching:

```bash
for i in {1..5}; do
  ./target/debug/dragonfly --enable-error-tracking invalid-cmd-$i 2>&1
done
# Check GlitchTip for multiple error events
```

## Integration with CI/CD

For automated testing in CI:

```yaml
# Example GitHub Actions
env:
  ERROR_TRACKING_DSN: ${{ secrets.GLITCHTIP_DSN }}
  ERROR_TRACKING_BACKEND: "glitchtip"

steps:
  - name: Test with GlitchTip
    run: |
      cargo build
      ./target/debug/dragonfly --enable-error-tracking health
```

## Next Steps

After verifying the integration works:

1. ✅ Update production configuration
2. ✅ Set up error alerts in GlitchTip
3. ✅ Configure release tracking
4. ✅ Set up filtering rules
5. ✅ Document team procedures
