#!/bin/bash
# Bottle status script - check Cloud Atlas AI installation

echo "Cloud Atlas AI Status"
echo "===================="
echo ""

# Binaries
echo "Binaries:"
for cmd in ba wm sg; do
  if command -v $cmd >/dev/null; then
    version=$($cmd --version 2>/dev/null || echo "installed")
    echo "  ✓ $cmd: $version"
  else
    echo "  ✗ $cmd: not installed"
  fi
done
echo ""

# Project directories
echo "Project setup:"
for dir in .ba .wm .superego; do
  if [ -d "$dir" ]; then
    echo "  ✓ $dir/ initialized"
  else
    echo "  ✗ $dir/ not found"
  fi
done
echo ""

# Skills
echo "Skills:"
SKILL_BASE="$HOME/.codex/skills"
for skill in ba wm superego bottle; do
  if [ -f "$SKILL_BASE/$skill/SKILL.md" ]; then
    echo "  ✓ \$$skill available"
  else
    echo "  ✗ \$$skill not installed"
  fi
done
echo ""

# AGENTS.md
if [ -f "AGENTS.md" ]; then
  if grep -q "Cloud Atlas AI" AGENTS.md 2>/dev/null; then
    echo "AGENTS.md: ✓ Contains Cloud Atlas AI guidance"
  else
    echo "AGENTS.md: ⚠ Exists but missing Cloud Atlas AI section"
  fi
else
  echo "AGENTS.md: ✗ Not found"
fi

# Superego mode
if [ -f ".superego/config.yaml" ]; then
  mode=$(grep "^mode:" .superego/config.yaml 2>/dev/null | cut -d' ' -f2)
  if [ -n "$mode" ]; then
    echo "Superego mode: $mode"
  fi
fi
