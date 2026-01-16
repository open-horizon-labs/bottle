#!/bin/bash
# Update manifest.json with latest versions from crates.io, npm, and GitHub
#
# Usage: ./scripts/update-manifest.sh [--dry-run]

set -e

MANIFEST="bottles/stable/manifest.json"
DRY_RUN=false

if [ "$1" = "--dry-run" ]; then
    DRY_RUN=true
fi

# Colors
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

log() { echo -e "${GREEN}[update]${NC} $1"; }
warn() { echo -e "${YELLOW}[update]${NC} $1"; }

# Fetch latest version from crates.io
crate_version() {
    curl -s "https://crates.io/api/v1/crates/$1" | jq -r '.crate.max_version // "0.0.0"'
}

# Fetch latest version from npm
npm_version() {
    npm view "$1" version 2>/dev/null || echo "0.0.0"
}

# Fetch version from GitHub raw file (Cargo.toml)
github_cargo_version() {
    local repo=$1
    curl -s "https://raw.githubusercontent.com/$repo/main/Cargo.toml" 2>/dev/null | \
        grep '^version' | head -1 | sed 's/version = "\(.*\)"/\1/'
}

# Fetch version from GitHub raw file (package.json)
github_npm_version() {
    local repo=$1
    curl -s "https://raw.githubusercontent.com/$repo/main/package.json" 2>/dev/null | \
        jq -r '.version // "0.0.0"'
}

log "Fetching latest versions..."

# Tools (prefer crates.io, fallback to GitHub)
BA_VERSION=$(crate_version "ba")
SUPEREGO_VERSION=$(crate_version "superego")

# wm is not on crates.io, use GitHub
WM_VERSION=$(github_cargo_version "open-horizon-labs/wm")
[ -z "$WM_VERSION" ] && WM_VERSION="0.0.0"

# oh-mcp from GitHub
OH_MCP_VERSION=$(github_npm_version "open-horizon-labs/oh-mcp-server")
[ -z "$OH_MCP_VERSION" ] && OH_MCP_VERSION="0.0.0"

# OpenCode plugins from npm
BOTTLE_OC_VERSION=$(npm_version "@cloud-atlas-ai/bottle")
BA_OC_VERSION=$(npm_version "ba-opencode")
WM_OC_VERSION=$(npm_version "wm-opencode")
SUPEREGO_OC_VERSION=$(npm_version "superego-opencode")

echo ""
log "Latest versions found:"
echo "  tools:"
echo "    ba:       $BA_VERSION"
echo "    wm:       $WM_VERSION"
echo "    superego: $SUPEREGO_VERSION"
echo "    oh-mcp:   $OH_MCP_VERSION"
echo "  opencode_plugins:"
echo "    @cloud-atlas-ai/bottle: $BOTTLE_OC_VERSION"
echo "    ba-opencode:            $BA_OC_VERSION"
echo "    wm-opencode:            $WM_OC_VERSION"
echo "    superego-opencode:      $SUPEREGO_OC_VERSION"
echo ""

if [ "$DRY_RUN" = true ]; then
    log "Dry run - no changes made"
    exit 0
fi

# Update manifest using jq
log "Updating $MANIFEST..."

jq --arg ba "$BA_VERSION" \
   --arg wm "$WM_VERSION" \
   --arg sg "$SUPEREGO_VERSION" \
   --arg oh "$OH_MCP_VERSION" \
   --arg bottle_oc "$BOTTLE_OC_VERSION" \
   --arg ba_oc "$BA_OC_VERSION" \
   --arg wm_oc "$WM_OC_VERSION" \
   --arg sg_oc "$SUPEREGO_OC_VERSION" \
   '.tools.ba = $ba |
    .tools.wm = $wm |
    .tools.superego = $sg |
    .tools["oh-mcp"] = $oh |
    .opencode_plugins["@cloud-atlas-ai/bottle"] = $bottle_oc |
    .opencode_plugins["ba-opencode"] = $ba_oc |
    .opencode_plugins["wm-opencode"] = $wm_oc |
    .opencode_plugins["superego-opencode"] = $sg_oc' \
   "$MANIFEST" > "$MANIFEST.tmp" && mv "$MANIFEST.tmp" "$MANIFEST"

log "Done!"
echo ""
log "Next step: release the updated manifest"
echo "  bottle release stable -m \"Update tool versions\""
