#!/bin/bash
# Bottle updater - Update all Cloud Atlas AI tools

set -e

# Colors
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

log() { echo -e "${GREEN}[bottle]${NC} $1"; }
warn() { echo -e "${YELLOW}[bottle]${NC} $1"; }
info() { echo -e "${BLUE}[bottle]${NC} $1"; }

# Check if command exists
has_command() {
    command -v "$1" >/dev/null 2>&1
}

# Update a cargo binary
update_cargo_bin() {
    local name="$1"
    log "Updating $name..."

    if cargo install --force "$name"; then
        info "✓ $name updated"
    else
        warn "Failed to update $name"
    fi
}

# Update plugins
update_plugins() {
    if ! has_command claude; then
        warn "Claude CLI not found, skipping plugin updates"
        return 0
    fi

    log "Updating Claude Code plugins..."

    # Update marketplace metadata first
    if claude plugin marketplace list 2>/dev/null | grep -q "superego"; then
        log "Updating superego marketplace..."
        claude plugin marketplace update superego 2>/dev/null || warn "Marketplace update failed (may not be needed)"
    fi

    if claude plugin marketplace list 2>/dev/null | grep -q "wm"; then
        log "Updating wm marketplace..."
        claude plugin marketplace update wm 2>/dev/null || warn "Marketplace update failed (may not be needed)"
    fi

    # Update plugins
    if claude plugin list 2>/dev/null | grep -q "^superego"; then
        log "Updating superego plugin..."
        claude plugin update superego && info "✓ superego plugin updated" || warn "Plugin update failed"
    fi

    if claude plugin list 2>/dev/null | grep -q "^wm"; then
        log "Updating wm plugin..."
        claude plugin update wm && info "✓ wm plugin updated" || warn "Plugin update failed"
    fi
}

main() {
    echo ""
    log "Updating Cloud Atlas AI tools..."
    echo ""

    # Update binaries
    update_cargo_bin "ba"
    update_cargo_bin "wm"
    update_cargo_bin "superego"

    echo ""

    # Update plugins
    update_plugins

    echo ""
    log "Update complete!"
    echo ""
    info "Note: Restart Claude Code to use updated plugins"
    echo ""
}

main "$@"
