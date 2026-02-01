#!/bin/bash

# Script Name: remove_tm_snapshots.sh
# Description: Remove Time Machine snapshots to free up disk space
# Usage: sudo ./remove_tm_snapshots.sh
#
# This script uses macOS's tmutil command to delete local Time Machine snapshots.
# Run with sudo privileges as tmutil requires root access.

# remove_tm_snapshots.sh
# Script to remove Time Machine snapshots
# Usage: sudo ./remove_tm_snapshots.sh

set -euo pipefail

echo "=== Time Machine Snapshot Removal ==="
echo "This script will remove all local Time Machine snapshots."
echo ""

# Check if running as root
if [[ $EUID -ne 0 ]]; then
    echo "This script must be run with sudo privileges."
    echo "Please run: sudo $0"
    exit 1
fi

# Check if tmutil is available
if ! command -v tmutil &> /dev/null; then
    echo "Error: tmutil command not found. This script requires macOS."
    exit 1
fi

# Display current disk usage
echo "Current disk usage before snapshot removal:"
df -h / | grep -v Filesystem

echo ""
echo "Current Time Machine snapshots:"
if tmutil listlocalsnapshots / 2>/dev/null | grep -q "com.apple.TimeMachine"; then
    tmutil listlocalsnapshots /
    echo ""
    read -p "Do you want to remove all Time Machine snapshots? (y/N): " -n 1 -r
    echo ""

    if [[ $REPLY =~ ^[Yy]$ ]]; then
        echo "Removing Time Machine snapshots..."
        tmutil deletelocalsnapshots /

        if [ $? -eq 0 ]; then
            echo "✅ Time Machine snapshots removed successfully!"
        else
            echo "❌ Failed to remove Time Machine snapshots."
            exit 1
        fi
    else
        echo "Operation cancelled."
        exit 0
    fi
else
    echo "No Time Machine snapshots found."
fi

echo ""
echo "Disk usage after snapshot removal:"
df -h / | grep -v Filesystem

echo ""
echo "=== Done ==="
