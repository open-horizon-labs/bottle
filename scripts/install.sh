#!/bin/bash
# Bottle installer - Cloud Atlas AI core stack
# Installs: ba, wm, superego

set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

log() { echo -e "${GREEN}[bottle]${NC} $1"; }
warn() { echo -e "${YELLOW}[bottle]${NC} $1"; }
error() { echo -e "${RED}[bottle]${NC} $1" >&2; exit 1; }
info() { echo -e "${BLUE}[bottle]${NC} $1"; }

# Check if command exists
has_command() {
    command -v "$1" >/dev/null 2>&1
}

# Check prerequisites
check_prereqs() {
    log "Checking prerequisites..."

    if ! has_command cargo; then
        error "cargo not found. Install Rust: https://rustup.rs"
    fi

    if ! has_command claude; then
        warn "claude CLI not found. Install from: https://claude.com/code"
        warn "Continuing with binary installation only..."
        SKIP_PLUGINS=1
    fi
}

# Install a cargo binary
install_cargo_bin() {
    local name="$1"
    log "Installing $name..."

    if cargo install "$name"; then
        info "✓ $name installed successfully"
    else
        error "Failed to install $name"
    fi
}

# Install Claude Code plugin
install_plugin() {
    local name="$1"

    if [ "$SKIP_PLUGINS" = "1" ]; then
        return 0
    fi

    log "Installing $name plugin..."

    # Check if plugin is already installed
    if claude plugin list 2>/dev/null | grep -q "^$name"; then
        info "✓ $name plugin already installed"
        return 0
    fi

    # Try to install from marketplace
    if claude plugin marketplace list 2>/dev/null | grep -q "$name"; then
        if claude plugin install "$name@$name"; then
            info "✓ $name plugin installed"
        else
            warn "Failed to install $name plugin from marketplace"
        fi
    else
        warn "$name plugin not found in marketplace"
        info "You can install it manually from: https://github.com/cloud-atlas-ai/$name"
    fi
}

# Main installation
main() {
    echo ""
    log "Cloud Atlas AI Bottle Installer"
    log "Installing core stack: ba, wm, superego"
    echo ""

    check_prereqs

    # Install binaries in dependency order
    install_cargo_bin "ba"
    install_cargo_bin "wm"
    install_cargo_bin "superego"

    echo ""
    log "Binaries installed. Installing Claude Code plugins..."
    echo ""

    # Install plugins
    install_plugin "superego"
    install_plugin "wm"

    echo ""
    log "Installation complete!"
    echo ""
    info "Next steps:"
    info "  1. cd /your/project"
    info "  2. ba init       # Initialize task tracking"
    info "  3. wm init       # Initialize working memory"
    info "  4. sg init       # Initialize superego"
    echo ""
    info "Or use Claude Code commands:"
    info "  /superego:init"
    echo ""
    info "To update: ./scripts/update.sh"
    info "Learn more: https://github.com/cloud-atlas-ai/bottle"
    echo ""
}

main "$@"
