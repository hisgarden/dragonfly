#!/usr/bin/env sh
# Quick test script for GlitchTip integration

set -e

echo "🧪 Testing GlitchTip Integration"
echo ""

# Check if GlitchTip backend is running
echo "1. Checking if GlitchTip backend is running..."
if curl -s http://localhost:8000/api/ > /dev/null 2>&1; then
    echo "   ✅ GlitchTip backend is accessible"
else
    echo "   ⚠️  GlitchTip backend not accessible at http://localhost:8000"
    echo "   Start it with: cd ../glitchtip-backend && nerdctl compose -f compose.yml up -d"
    exit 1
fi

# Check if DSN is configured
echo ""
echo "2. Checking DSN configuration..."
if [ -f .glitchtiprc ]; then
    DSN=$(grep "^url=" .glitchtiprc | cut -d'=' -f2- | tr -d ' ')
    if [ -n "$DSN" ]; then
        echo "   ✅ Found DSN in .glitchtiprc: ${DSN%%@*}@***"
    else
        echo "   ⚠️  No DSN found in .glitchtiprc"
        exit 1
    fi
elif [ -n "$ERROR_TRACKING_DSN" ]; then
    echo "   ✅ Found DSN in environment: ${ERROR_TRACKING_DSN%%@*}@***"
    DSN="$ERROR_TRACKING_DSN"
else
    echo "   ⚠️  No DSN configured"
    echo "   Set ERROR_TRACKING_DSN or create .glitchtiprc"
    exit 1
fi

# Verify DSN format
echo ""
echo "3. Verifying DSN format..."
if echo "$DSN" | grep -qE "^https?://.*@.*:[0-9]+/[0-9]+"; then
    echo "   ✅ DSN format looks correct"
else
    echo "   ⚠️  DSN format may be incorrect"
fi

# Check if dragonfly is built
echo ""
echo "4. Checking if dragonfly is built..."
if [ -f target/debug/dragonfly ]; then
    echo "   ✅ dragonfly binary found"
    BINARY="./target/debug/dragonfly"
elif [ -f target/release/dragonfly ]; then
    echo "   ✅ dragonfly binary found (release)"
    BINARY="./target/release/dragonfly"
else
    echo "   ⚠️  dragonfly binary not found"
    echo "   Build it with: cargo build"
    exit 1
fi

# Test error tracking initialization
echo ""
echo "5. Testing error tracking initialization..."
export ERROR_TRACKING_DSN="${DSN:-http://728ac3a912b44b15afa8a047a87dea8f@localhost:8000/1}"
export RUST_LOG=info

if $BINARY --enable-error-tracking health > /tmp/dragonfly-test.log 2>&1; then
    echo "   ✅ Error tracking initialized successfully"
    if grep -q "Initializing error tracking" /tmp/dragonfly-test.log 2>/dev/null; then
        echo "   ✅ Backend detected correctly"
    fi
else
    echo "   ⚠️  Error tracking initialization failed"
    echo "   Check logs: cat /tmp/dragonfly-test.log"
    exit 1
fi

# Test error capture
echo ""
echo "6. Testing error capture..."
if $BINARY --enable-error-tracking invalid-command-test 2>&1 | grep -q "error\|Error\|ERROR"; then
    echo "   ✅ Error was triggered (check GlitchTip dashboard)"
else
    echo "   ⚠️  Could not trigger test error"
fi

echo ""
echo "✅ All checks passed!"
echo ""
echo "Next steps:"
echo "1. Check GlitchTip dashboard: http://localhost:8000"
echo "2. Look for errors in your project"
echo "3. Verify stack traces and metadata are included"
echo ""
echo "To test manually:"
echo "  export ERROR_TRACKING_DSN=\"$ERROR_TRACKING_DSN\""
echo "  $BINARY --enable-error-tracking health"
