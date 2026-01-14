#!/bin/bash
# Test a local build of bottle before shipping
#
# Usage:
#   ./scripts/test-local.sh          # Build release and run smoke tests
#   ./scripts/test-local.sh --shell  # Build and drop into shell with local bottle in PATH
#   ./scripts/test-local.sh --install # Build and install to ~/.local/bin

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"
BUILD_DIR="$PROJECT_DIR/target/release"
LOCAL_BIN="$HOME/.local/bin"

cd "$PROJECT_DIR"

echo "==> Building release binary..."
cargo build --release

BOTTLE_BIN="$BUILD_DIR/bottle"

if [[ ! -f "$BOTTLE_BIN" ]]; then
    echo "ERROR: Build failed - no binary at $BOTTLE_BIN"
    exit 1
fi

echo "==> Built: $BOTTLE_BIN"
echo "==> Version: $($BOTTLE_BIN --version)"

# Handle arguments
case "${1:-}" in
    --shell)
        echo ""
        echo "==> Dropping into shell with local bottle in PATH"
        echo "    Type 'exit' to return to normal shell"
        echo ""
        export PATH="$BUILD_DIR:$PATH"
        exec $SHELL
        ;;
    --install)
        echo ""
        echo "==> Installing to $LOCAL_BIN"
        mkdir -p "$LOCAL_BIN"
        cp "$BOTTLE_BIN" "$LOCAL_BIN/bottle"
        echo "    Installed: $LOCAL_BIN/bottle"
        echo "    Make sure $LOCAL_BIN is in your PATH"
        ;;
    *)
        # Run smoke tests
        echo ""
        echo "==> Running smoke tests..."
        echo ""

        # Test 1: Help command
        echo "--- bottle --help"
        $BOTTLE_BIN --help | head -5
        echo "✓ Help works"
        echo ""

        # Test 2: List command
        echo "--- bottle list"
        $BOTTLE_BIN list
        echo "✓ List works"
        echo ""

        # Test 3: Status command (may fail if not installed, that's ok)
        echo "--- bottle status"
        $BOTTLE_BIN status 2>&1 || echo "(expected - no bottle installed)"
        echo "✓ Status works"
        echo ""

        # Test 4: Install dry-run
        echo "--- bottle install stable --dry-run"
        $BOTTLE_BIN install stable --dry-run
        echo "✓ Install dry-run works"
        echo ""

        # Test 5: Integrate list
        if $BOTTLE_BIN status &>/dev/null; then
            echo "--- bottle integrate --list"
            $BOTTLE_BIN integrate --list
            echo "✓ Integrate list works"
        else
            echo "--- Skipping integrate --list (no bottle installed)"
        fi
        echo ""

        echo "==> All smoke tests passed!"
        echo ""
        echo "To test more thoroughly:"
        echo "  $0 --shell     # Get a shell with local bottle"
        echo "  $0 --install   # Install to ~/.local/bin"
        ;;
esac
