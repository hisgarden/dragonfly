#!/bin/sh
# Move Rancher Desktop data directory to external SSD
# Usage: ./move-rancher-to-ssd.sh /Volumes/YourSSD

set -e

RANCHER_DATA_DIR="$HOME/Library/Application Support/rancher-desktop"
LIMA_DIR="$RANCHER_DATA_DIR/lima"

if [ $# -eq 0 ]; then
    echo "Usage: $0 <target_ssd_path>"
    echo "Example: $0 /Volumes/MySSD"
    exit 1
fi

TARGET_SSD="$1"
TARGET_DIR="$TARGET_SSD/rancher-desktop-lima"

# Check if target SSD exists
if [ ! -d "$TARGET_SSD" ]; then
    echo "❌ Error: Target SSD path does not exist: $TARGET_SSD"
    exit 1
fi

# Check if Rancher Desktop is running
if pgrep -f "Rancher Desktop" > /dev/null; then
    echo "⚠️  Warning: Rancher Desktop appears to be running."
    echo "   Please quit Rancher Desktop before proceeding."
    read -p "   Quit Rancher Desktop now? (y/n) " -n 1 -r
    echo
    if [ "$REPLY" = "y" ] || [ "$REPLY" = "Y" ]; then
        osascript -e 'quit app "Rancher Desktop"' || true
        echo "   Waiting 5 seconds for Rancher Desktop to quit..."
        sleep 5
    else
        echo "❌ Aborted. Please quit Rancher Desktop manually and try again."
        exit 1
    fi
fi

# Check if lima directory exists
if [ ! -d "$LIMA_DIR" ]; then
    echo "❌ Error: Rancher Desktop lima directory not found: $LIMA_DIR"
    echo "   Rancher Desktop may not be installed or configured."
    exit 1
fi

# Check if already a symlink
if [ -L "$LIMA_DIR" ]; then
    echo "⚠️  Warning: $LIMA_DIR is already a symlink:"
    ls -ld "$LIMA_DIR"
    read -p "   Continue anyway? (y/n) " -n 1 -r
    echo
    if [ "$REPLY" != "y" ] && [ "$REPLY" != "Y" ]; then
        echo "❌ Aborted."
        exit 1
    fi
    # Remove existing symlink
    rm "$LIMA_DIR"
fi

# Check available space
LIMA_SIZE=$(du -sk "$LIMA_DIR" | cut -f1)
AVAILABLE_SPACE=$(df -k "$TARGET_SSD" | tail -1 | awk '{print $4}')

if [ "$LIMA_SIZE" -gt "$AVAILABLE_SPACE" ]; then
    echo "❌ Error: Not enough space on target SSD"
    echo "   Required: $(numfmt --to=iec-i --suffix=B $((LIMA_SIZE * 1024)))"
    echo "   Available: $(numfmt --to=iec-i --suffix=B $((AVAILABLE_SPACE * 1024)))"
    exit 1
fi

echo "📋 Summary:"
echo "   Source: $LIMA_DIR"
echo "   Target: $TARGET_DIR"
echo "   Size: $(du -sh "$LIMA_DIR" | cut -f1)"
echo ""
read -p "⚠️  This will move Rancher Desktop data to SSD. Continue? (y/n) " -n 1 -r
echo
if [ "$REPLY" != "y" ] && [ "$REPLY" != "Y" ]; then
    echo "❌ Aborted."
    exit 1
fi

# Create target directory
echo "📁 Creating target directory..."
mkdir -p "$TARGET_DIR"

# Move the directory
echo "🚚 Moving lima directory to SSD (this may take a while)..."
mv "$LIMA_DIR" "$TARGET_DIR/lima"

# Create symlink
echo "🔗 Creating symlink..."
ln -s "$TARGET_DIR/lima" "$LIMA_DIR"

# Verify
if [ -L "$LIMA_DIR" ] && [ -d "$TARGET_DIR/lima" ]; then
    echo "✅ Success! Rancher Desktop data moved to SSD."
    echo ""
    echo "📋 Verification:"
    ls -ld "$LIMA_DIR"
    echo ""
    echo "💡 Next steps:"
    echo "   1. Start Rancher Desktop"
    echo "   2. Verify it works correctly"
    echo "   3. If you factory reset Rancher Desktop, you'll need to recreate the symlink"
else
    echo "❌ Error: Symlink creation failed. Please check manually."
    exit 1
fi
