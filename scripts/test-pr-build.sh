#!/bin/bash
# Download and test a PR's cloud build artifacts
#
# Usage:
#   ./scripts/test-pr-build.sh          # Test artifacts from current branch's latest PR run
#   ./scripts/test-pr-build.sh 37       # Test artifacts from PR #37
#   ./scripts/test-pr-build.sh --run 123456789  # Test artifacts from specific run ID

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"
DOWNLOAD_DIR="$PROJECT_DIR/target/pr-artifacts"

cd "$PROJECT_DIR"

# Parse arguments
RUN_ID=""
PR_NUM=""

case "${1:-}" in
    --run)
        RUN_ID="$2"
        ;;
    [0-9]*)
        PR_NUM="$1"
        ;;
    *)
        # Try to find PR for current branch
        CURRENT_BRANCH=$(git branch --show-current)
        echo "==> Looking for PR from branch: $CURRENT_BRANCH"
        PR_NUM=$(gh pr list --head "$CURRENT_BRANCH" --json number --jq '.[0].number' 2>/dev/null || echo "")
        if [[ -z "$PR_NUM" ]]; then
            echo "ERROR: No PR found for current branch"
            echo ""
            echo "Usage:"
            echo "  $0           # Use PR from current branch"
            echo "  $0 37        # Use PR #37"
            echo "  $0 --run ID  # Use specific workflow run ID"
            exit 1
        fi
        ;;
esac

# Find the workflow run
if [[ -n "$PR_NUM" ]]; then
    echo "==> Finding workflow run for PR #$PR_NUM..."
    RUN_ID=$(gh run list --workflow=release.yml --json databaseId,headBranch,status,event \
        --jq ".[] | select(.event == \"pull_request\") | .databaseId" | head -1)

    if [[ -z "$RUN_ID" ]]; then
        # Try to get runs from the PR's head branch
        PR_BRANCH=$(gh pr view "$PR_NUM" --json headRefName --jq '.headRefName')
        echo "==> Looking for runs on branch: $PR_BRANCH"
        RUN_ID=$(gh run list --workflow=release.yml --branch "$PR_BRANCH" --json databaseId,status \
            --jq '.[0].databaseId' 2>/dev/null || echo "")
    fi
fi

if [[ -z "$RUN_ID" ]]; then
    echo "ERROR: Could not find workflow run"
    echo ""
    echo "Available runs:"
    gh run list --workflow=release.yml --limit 5
    exit 1
fi

echo "==> Using workflow run: $RUN_ID"
echo ""

# Check run status
RUN_STATUS=$(gh run view "$RUN_ID" --json status --jq '.status')
echo "==> Run status: $RUN_STATUS"

if [[ "$RUN_STATUS" == "in_progress" || "$RUN_STATUS" == "queued" ]]; then
    echo ""
    echo "Run is still in progress. Wait for it to complete or watch with:"
    echo "  gh run watch $RUN_ID"
    exit 1
fi

# Clean and create download directory
rm -rf "$DOWNLOAD_DIR"
mkdir -p "$DOWNLOAD_DIR"

# Download artifacts
echo ""
echo "==> Downloading artifacts..."
gh run download "$RUN_ID" --dir "$DOWNLOAD_DIR" 2>&1 || {
    echo ""
    echo "ERROR: Failed to download artifacts"
    echo "This might happen if:"
    echo "  - The run is still in progress"
    echo "  - The run failed before producing artifacts"
    echo "  - Artifacts have expired (90 days)"
    echo ""
    echo "Check the run status:"
    echo "  gh run view $RUN_ID"
    exit 1
}

echo ""
echo "==> Downloaded to: $DOWNLOAD_DIR"
ls -la "$DOWNLOAD_DIR"

# Find the binary for this architecture
ARCH=$(uname -m)
case "$ARCH" in
    arm64|aarch64)
        TARGET="aarch64-apple-darwin"
        ;;
    x86_64)
        TARGET="x86_64-apple-darwin"
        ;;
    *)
        echo "ERROR: Unsupported architecture: $ARCH"
        exit 1
        ;;
esac

echo ""
echo "==> Looking for $TARGET binary..."

# Find the artifact directory
ARTIFACT_DIR=$(find "$DOWNLOAD_DIR" -type d -name "*$TARGET*" 2>/dev/null | head -1)

if [[ -z "$ARTIFACT_DIR" ]]; then
    # Try to find any bottle binary
    BOTTLE_BIN=$(find "$DOWNLOAD_DIR" -name "bottle" -type f 2>/dev/null | head -1)
    if [[ -z "$BOTTLE_BIN" ]]; then
        echo "ERROR: Could not find bottle binary for $TARGET"
        echo ""
        echo "Available artifacts:"
        find "$DOWNLOAD_DIR" -type f | head -20
        exit 1
    fi
else
    # Look for binary in artifact dir
    BOTTLE_BIN=$(find "$ARTIFACT_DIR" -name "bottle" -type f 2>/dev/null | head -1)
    if [[ -z "$BOTTLE_BIN" ]]; then
        # Try to extract from tarball
        TARBALL=$(find "$ARTIFACT_DIR" -name "*.tar.*" 2>/dev/null | head -1)
        if [[ -n "$TARBALL" ]]; then
            echo "==> Extracting from $TARBALL..."
            EXTRACT_DIR="$DOWNLOAD_DIR/extracted"
            mkdir -p "$EXTRACT_DIR"
            tar -xf "$TARBALL" -C "$EXTRACT_DIR"
            BOTTLE_BIN=$(find "$EXTRACT_DIR" -name "bottle" -type f 2>/dev/null | head -1)
        fi
    fi
fi

if [[ -z "$BOTTLE_BIN" || ! -f "$BOTTLE_BIN" ]]; then
    echo "ERROR: Could not find bottle binary"
    echo ""
    echo "Contents of download dir:"
    find "$DOWNLOAD_DIR" -type f
    exit 1
fi

chmod +x "$BOTTLE_BIN"

echo ""
echo "==> Found binary: $BOTTLE_BIN"
echo "==> Version: $($BOTTLE_BIN --version)"
echo ""

# Run smoke tests
echo "==> Running smoke tests on cloud build..."
echo ""

echo "--- bottle --help"
$BOTTLE_BIN --help | head -5
echo "✓ Help works"
echo ""

echo "--- bottle list"
$BOTTLE_BIN list
echo "✓ List works"
echo ""

echo "--- bottle install stable --dry-run"
$BOTTLE_BIN install stable --dry-run
echo "✓ Install dry-run works"
echo ""

echo "==> Cloud build smoke tests passed!"
echo ""
echo "To use this build:"
echo "  export PATH=\"$(dirname "$BOTTLE_BIN"):\$PATH\""
echo "  bottle --version"
echo ""
echo "Or copy to your PATH:"
echo "  cp \"$BOTTLE_BIN\" ~/.local/bin/bottle"
