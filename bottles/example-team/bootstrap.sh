#!/bin/bash
# Example Team Bottle Setup
# Usage: bash dev_tools/bottle/bootstrap.sh

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$(dirname "$SCRIPT_DIR")")"
MANIFEST="$SCRIPT_DIR/manifest.json"

echo "========================================"
echo "Example Team Bottle Setup"
echo "========================================"

#---------------------------------------
# 1. Install Bottle CLI (prefer cargo)
#---------------------------------------
echo ""
echo "[1/4] Installing bottle..."

if ! command -v bottle &> /dev/null; then
    if command -v cargo &> /dev/null; then
        cargo install bottle
    elif command -v brew &> /dev/null; then
        brew tap open-horizon-labs/homebrew-tap
        brew install bottle
    else
        echo "ERROR: Need cargo or brew to install bottle"
        echo "  Install Rust: curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
        exit 1
    fi
fi
echo "✓ Bottle CLI installed"

#---------------------------------------
# 2. Install from Manifest
#---------------------------------------
echo ""
echo "[2/4] Installing tools from manifest..."

bottle install --manifest "$MANIFEST"
echo "✓ Tools installed"

#---------------------------------------
# 3. Integrate with OpenCode
#---------------------------------------
echo ""
echo "[3/4] Integrating with OpenCode..."

cd "$PROJECT_ROOT"
bottle integrate opencode --manifest "$MANIFEST"
echo "✓ OpenCode integration configured"

#---------------------------------------
# 4. Initialize Project Tools
#---------------------------------------
echo ""
echo "[4/4] Initializing project tools..."

cd "$PROJECT_ROOT"
ba init 2>/dev/null || true
wm init 2>/dev/null || true
sg init 2>/dev/null || true
echo "✓ Project tools initialized"

#---------------------------------------
# Done
#---------------------------------------
echo ""
echo "========================================"
echo "Setup Complete!"
echo "========================================"
echo ""
echo "To verify: bottle verify --manifest $MANIFEST"
echo ""
echo "To start OpenCode:"
echo "  cd $PROJECT_ROOT"
echo "  opencode"
echo ""
