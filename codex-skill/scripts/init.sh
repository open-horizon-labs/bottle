#!/bin/bash
# Bottle init script - installs Cloud Atlas AI binaries and skills
set -e

echo "Cloud Atlas AI Setup"
echo "===================="
echo ""

# Check for package managers
HAS_BREW=$(command -v brew >/dev/null && echo "yes" || echo "no")
HAS_CARGO=$(command -v cargo >/dev/null && echo "yes" || echo "no")

if [ "$HAS_BREW" = "no" ] && [ "$HAS_CARGO" = "no" ]; then
  echo "ERROR: Neither Homebrew nor Cargo found."
  echo ""
  echo "Install Homebrew:"
  echo '  /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"'
  echo ""
  echo "Or install Rust (includes Cargo):"
  echo '  curl --proto "=https" --tlsv1.2 -sSf https://sh.rustup.rs | sh'
  exit 1
fi

# Install binaries
echo "Installing binaries..."

install_binary() {
  local name=$1
  local brew_name=$2
  local cargo_name=$3

  if command -v "$name" >/dev/null; then
    echo "  ✓ $name already installed"
    return 0
  fi

  if [ "$HAS_BREW" = "yes" ]; then
    echo "  Installing $name via Homebrew..."
    brew install "cloud-atlas-ai/$brew_name/$brew_name" 2>/dev/null || \
      brew tap cloud-atlas-ai/tap && brew install "$brew_name"
  elif [ "$HAS_CARGO" = "yes" ]; then
    echo "  Installing $name via Cargo..."
    cargo install "$cargo_name"
  fi

  if command -v "$name" >/dev/null; then
    echo "  ✓ $name installed"
  else
    echo "  ✗ Failed to install $name"
    return 1
  fi
}

install_binary "ba" "ba" "ba"
install_binary "wm" "wm" "working-memory"
install_binary "sg" "superego" "superego"

echo ""

# Install skills
echo "Installing Codex skills..."
SKILL_BASE="$HOME/.codex/skills"

install_skill() {
  local name=$1
  local repo=$2
  local files=$3

  mkdir -p "$SKILL_BASE/$name"

  for file in $files; do
    curl -fsSL -o "$SKILL_BASE/$name/$file" \
      "https://raw.githubusercontent.com/cloud-atlas-ai/$repo/main/codex-skill/$file" 2>/dev/null || \
      echo "  Warning: Could not download $file for $name"
  done
  echo "  ✓ \$$name skill installed"
}

# Install individual skills
install_skill "ba" "ba" "SKILL.md"
install_skill "wm" "wm" "SKILL.md"

# Superego has additional files
mkdir -p "$SKILL_BASE/superego/agents"
curl -fsSL -o "$SKILL_BASE/superego/SKILL.md" \
  "https://raw.githubusercontent.com/cloud-atlas-ai/superego/main/codex-skill/SKILL.md" 2>/dev/null
curl -fsSL -o "$SKILL_BASE/superego/AGENTS.md.snippet" \
  "https://raw.githubusercontent.com/cloud-atlas-ai/superego/main/codex-skill/AGENTS.md.snippet" 2>/dev/null
for agent in code.md writing.md learning.md; do
  curl -fsSL -o "$SKILL_BASE/superego/agents/$agent" \
    "https://raw.githubusercontent.com/cloud-atlas-ai/superego/main/codex-skill/agents/$agent" 2>/dev/null
done
echo "  ✓ \$superego skill installed"

# Install bottle skill
install_skill "bottle" "bottle" "SKILL.md AGENTS.md.snippet"

echo ""

# Initialize project directories
echo "Initializing project..."

if [ ! -d ".ba" ]; then
  ba init 2>/dev/null && echo "  ✓ .ba/ initialized" || echo "  ✗ ba init failed"
else
  echo "  ✓ .ba/ already exists"
fi

if [ ! -d ".wm" ]; then
  wm init 2>/dev/null && echo "  ✓ .wm/ initialized" || echo "  ✗ wm init failed"
else
  echo "  ✓ .wm/ already exists"
fi

if [ ! -d ".superego" ]; then
  sg init 2>/dev/null && echo "  ✓ .superego/ initialized" || echo "  ✗ sg init failed"
else
  echo "  ✓ .superego/ already exists"
fi

# Set superego to pull mode
if [ -f ".superego/config.yaml" ]; then
  if grep -q "^mode: always" .superego/config.yaml 2>/dev/null; then
    sed -i.bak 's/^mode: always/mode: pull/' .superego/config.yaml
    rm -f .superego/config.yaml.bak
    echo "  ✓ Superego set to pull mode"
  fi
fi

echo ""
echo "Setup complete!"
echo ""
echo "Quick start:"
echo "  \$bottle dive fix    # Start a bug fix session"
echo "  \$ba status          # Check your tasks"
echo "  \$superego           # Get feedback at decision points"
