# Quick Start: GlitchTip Integration

This is a quick guide to get DragonFly working with your GlitchTip instance.

## Your Configuration

Your GlitchTip DSN is already configured:
- **DSN**: `http://728ac3a912b44b15afa8a047a87dea8f@localhost:8000/1`
- **Config File**: `.glitchtiprc` (already created)

## Quick Test

### 1. Ensure GlitchTip is Running

```bash
cd /Volumes/OWC_1M2/workspace/util/glitchtip-backend
nerdctl compose -f compose.yml ps
# Should show web, worker, postgres, and valkey running
```

If not running:
```bash
nerdctl compose -f compose.yml up -d
```

### 2. Build DragonFly

```bash
cd /Volumes/OWC_1M2/workspace/util/dragonfly
cargo build
```

### 3. Run with Error Tracking

```bash
./target/debug/dragonfly --enable-error-tracking health
```

You should see initialization logs indicating GlitchTip backend is detected.

### 4. Trigger a Test Error

```bash
./target/debug/dragonfly --enable-error-tracking invalid-command 2>&1
```

### 5. Check GlitchTip Dashboard

1. Open: http://localhost:8000
2. Navigate to your project
3. Check the "Issues" tab
4. You should see the error you triggered

## Using Environment Variable (Alternative)

Instead of using `.glitchtiprc`, you can set:

```bash
export ERROR_TRACKING_DSN="http://728ac3a912b44b15afa8a047a87dea8f@localhost:8000/1"
./target/debug/dragonfly --enable-error-tracking health
```

## Automated Test

Run the test script:

```bash
./test-glitchtip.sh
```

This will:
- ✅ Check GlitchTip backend is running
- ✅ Verify DSN configuration
- ✅ Test error tracking initialization
- ✅ Trigger a test error

## Verification Checklist

- [ ] GlitchTip backend is running (`nerdctl compose ps`)
- [ ] `.glitchtiprc` file exists with your DSN
- [ ] DragonFly builds successfully (`cargo build`)
- [ ] Error tracking initializes (`--enable-error-tracking` flag)
- [ ] Errors appear in GlitchTip dashboard
- [ ] Stack traces are included
- [ ] Release version is correct (`dragonfly@0.1.0`)

## Troubleshooting

### "Connection refused" errors

- Check GlitchTip is running: `nerdctl compose ps`
- Verify DSN host: Should be `localhost:8000`
- Test connectivity: `curl http://localhost:8000/api/`

### Errors not appearing in GlitchTip

- Check DSN is correct in `.glitchtiprc`
- Verify `--enable-error-tracking` flag is used
- Check GlitchTip logs: `nerdctl logs glitchtip-backend-web-1 -f`
- Enable debug logging: `RUST_LOG=debug ./target/debug/dragonfly --enable-error-tracking health`

### Backend not detected

The adapter should auto-detect GlitchTip from `localhost:8000`. If not:

```bash
export ERROR_TRACKING_BACKEND="glitchtip"
```

## Next Steps

1. ✅ Test with real errors
2. ✅ Set up error alerts in GlitchTip
3. ✅ Configure release tracking
4. ✅ Set up filtering rules
5. ✅ Document team procedures

For detailed documentation, see [ERROR_TRACKING.md](./ERROR_TRACKING.md).
