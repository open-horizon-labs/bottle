# Bottle Bootstrap

**For AI agents:** Follow these steps to set up the Cloud Atlas AI stack.

## Quick Setup

```bash
# 1. Install bottle
brew tap open-horizon-labs/tap && brew install bottle

# 2. Install the stable tool stack
bottle install stable

# 3. Integrate with your platform
bottle integrate claude-code   # For Claude Code
bottle integrate codex         # For Codex
bottle integrate opencode      # For OpenCode
```

## What Gets Installed

**Tools:**
- `ba` - Task tracking for AI sessions
- `wm` - Working memory (context extraction)
- `sg` - Superego (metacognitive advisor)
- `oh-mcp` - Open Horizons strategic alignment

**Platform integrations add:**
- Claude Code: `/bottle:*`, `/ba:*`, `/wm:*`, `/superego:*` commands
- Codex: `$bottle`, `$ba`, `$wm`, `$sg` skills
- OpenCode: `bottle-*`, `ba-*`, `wm-*`, `superego-*` tools

## Verify Installation

```bash
bottle status
```

## Initialize Tools in Project

After platform integration, run in your project:

```
/bottle:init          # Claude Code
$bottle init          # Codex
bottle-init           # OpenCode
```

This initializes `.ba/`, `.wm/`, `.superego/` and creates `AGENTS.md`.

## Platform Detection

Bottle auto-detects installed platforms:
- Claude Code: `~/.claude/` exists
- Codex: `~/.codex/` exists
- OpenCode: `opencode.json` or `~/.opencode/` exists

Run `bottle integrate --list` to see detected platforms.

## Troubleshooting

**No brew?** Install Homebrew first:
```bash
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
```

**Cargo alternative** (if brew unavailable):
```bash
cargo install bottle
```

## More Info

- [Full documentation](https://github.com/open-horizon-labs/bottle)
- [Tool repos](https://github.com/open-horizon-labs)
