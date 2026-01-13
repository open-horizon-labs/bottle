#!/bin/bash
# Release script for bottle
#
# Usage: ./scripts/release.sh [version]
# Examples:
#   ./scripts/release.sh        # Auto-increment patch (0.1.0 → 0.1.1)
#   ./scripts/release.sh 0.2.0  # Explicit version
#
# This script:
# 1. Updates version in Cargo.toml
# 2. Updates version in .claude-plugin/plugin.json
# 3. Runs tests
# 4. Commits the version bump
# 5. Pushes to master
# 6. Creates and pushes git tag (triggers cargo-dist workflow)

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

log() { echo -e "${GREEN}[release]${NC} $1"; }
warn() { echo -e "${YELLOW}[release]${NC} $1"; }
error() { echo -e "${RED}[release]${NC} $1" >&2; exit 1; }

# Get version - either from argument or auto-increment patch
if [ -z "$1" ]; then
    # Auto-increment patch version
    CURRENT=$(grep '^version = ' Cargo.toml | head -1 | sed 's/version = "\(.*\)"/\1/')
    if [ -z "$CURRENT" ]; then
        error "Could not read current version from Cargo.toml"
    fi
    MAJOR=$(echo "$CURRENT" | cut -d. -f1)
    MINOR=$(echo "$CURRENT" | cut -d. -f2)
    PATCH=$(echo "$CURRENT" | cut -d. -f3)
    VERSION="$MAJOR.$MINOR.$((PATCH + 1))"
    log "No version specified, auto-incrementing: $CURRENT → $VERSION"
else
    VERSION="$1"
    # Validate version format (semver)
    if ! echo "$VERSION" | grep -qE '^[0-9]+\.[0-9]+\.[0-9]+$'; then
        error "Invalid version format. Use semver (e.g., 0.2.0)"
    fi
fi

TAG="v$VERSION"

# Check we're in repo root
if [ ! -f "Cargo.toml" ]; then
    error "Must run from repository root (Cargo.toml not found)"
fi

# Check for clean working directory
if [ -n "$(git status --porcelain)" ]; then
    error "Working directory not clean. Commit or stash changes first."
fi

# Check we're on master branch
BRANCH=$(git branch --show-current)
if [ "$BRANCH" != "master" ]; then
    warn "Not on master branch (on: $BRANCH). Continue? [y/N]"
    read -r response
    if [ "$response" != "y" ]; then
        exit 1
    fi
fi

# Check tag doesn't already exist
if git rev-parse "$TAG" >/dev/null 2>&1; then
    error "Tag $TAG already exists"
fi

# Get repo URL from Cargo.toml
REPO_URL=$(grep '^repository' Cargo.toml | sed 's/.*= "//' | sed 's/"//')
if [ -z "$REPO_URL" ]; then
    REPO_URL="https://github.com/open-horizon-labs/bottle"
fi

log "Releasing $TAG"
log "Repository: $REPO_URL"

# Step 1: Update Cargo.toml version
log "Updating Cargo.toml version to $VERSION..."
sed -i '' "s/^version = \".*\"/version = \"$VERSION\"/" Cargo.toml

# Verify the change
if ! grep -q "version = \"$VERSION\"" Cargo.toml; then
    error "Failed to update version in Cargo.toml"
fi

# Step 2: Update plugin version
PLUGIN_JSON=".claude-plugin/plugin.json"
if [ -f "$PLUGIN_JSON" ]; then
    log "Updating plugin version in $PLUGIN_JSON..."
    sed -i '' "s/\"version\": \"[^\"]*\"/\"version\": \"$VERSION\"/" "$PLUGIN_JSON"

    if ! grep -q "\"version\": \"$VERSION\"" "$PLUGIN_JSON"; then
        error "Failed to update version in $PLUGIN_JSON"
    fi
fi

# Step 3: Run tests
log "Running tests..."
cargo test || error "Tests failed"

# Step 4: Build release (verify it compiles)
log "Building release binary..."
cargo build --release || error "Build failed"

# Step 5: Commit version bump
log "Committing version bump..."
git add Cargo.toml Cargo.lock
[ -f "$PLUGIN_JSON" ] && git add "$PLUGIN_JSON"

if git diff --cached --quiet; then
    log "Version already at $VERSION, skipping commit"
else
    git commit -m "Bump version to $VERSION"
fi

# Step 6: Push to origin
log "Pushing to origin..."
git push origin "$BRANCH"

# Step 7: Create and push tag
log "Creating tag $TAG..."
git tag "$TAG"

log "Pushing tag $TAG..."
git push origin "$TAG"

log ""
log "✓ Release $TAG complete!"
log ""
log "The cargo-dist workflow will now:"
log "  1. Build macOS binaries (arm64 + x86_64)"
log "  2. Create GitHub release"
log "  3. Update homebrew-tap formula"
log ""
log "Monitor progress at: $REPO_URL/actions"
log ""
log "Once complete, users can install with:"
log "  brew tap open-horizon-labs/tap"
log "  brew install bottle"
