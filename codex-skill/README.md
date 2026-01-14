# Bottle Codex Skill

Codex skill for orchestrating the Open Horizon Labs tool suite.

## Installation

### Via Bottle CLI (Recommended)

If you have the bottle CLI installed:
```bash
bottle integrate codex
```

This installs all Open Horizon Labs skills (bottle, ba, wm, sg).

### Manual Install

Copy the skill directories to `~/.codex/skills/`:
```bash
mkdir -p ~/.codex/skills/bottle
cp codex-skill/SKILL.md codex-skill/AGENTS.md.snippet ~/.codex/skills/bottle/
cp -r codex-skill/ba codex-skill/wm codex-skill/sg ~/.codex/skills/
```

## Usage

### Standard Commands (CLI Wrappers)

| Command | Description |
|---------|-------------|
| `$bottle init` | Install bottle and set up Codex integration |
| `$bottle status` | Show current bottle state |
| `$bottle update` | Update to latest bottle snapshot |
| `$bottle remove` | Exit bottle management |

### Codex-Native Features

| Command | Description |
|---------|-------------|
| `$bottle dive <intent>` | Start a focused session (fix/plan/explore/ship) |
| `$bottle codex-sync` | Sync Codex sessions to working memory |
| `$bottle web-context <query>` | Augment context with web search |
| `$bottle agents` | Show recommended AGENTS.md content |

### Quick Start

```
$bottle init              # One-time setup
$bottle dive fix          # Start a bug fix session
$ba status                # Check your tasks
$sg review                # Get feedback at decision points
```

## What Gets Installed

### Binaries (via Homebrew or Cargo)

- `ba` - Task tracking (backlog automaton)
- `wm` - Working memory
- `sg` - Superego metacognitive oversight

Install via:
```bash
brew tap open-horizon-labs/homebrew-tap && brew install ba wm superego
# or: cargo install ba working-memory superego
```

### Skills

- `$bottle` - Orchestration (this skill)
- `$ba` - Task tracking commands
- `$wm` - Working memory commands
- `$sg` - Metacognitive evaluation

### Project Files

- `.ba/` - Task tracking data
- `.wm/` - Working memory state
- `.superego/` - Superego configuration
- `AGENTS.md` - Protocol guidance for Codex

## Dive-First Workflow

The recommended way to work:

1. **Start with intent**: `$bottle dive fix` (or plan/explore/ship)
2. **Track your work**: `$ba status`, `ba claim <task>`
3. **Get feedback**: `$sg review` at decision points
4. **Extract learnings**: `wm distill` after completing work

No dive is too small. The 30 seconds of setup prevents 30 minutes of drift.

## Files

```
codex-skill/
  SKILL.md            # Main bottle skill
  AGENTS.md.snippet   # Content for user's AGENTS.md
  README.md           # This file
  ba/SKILL.md         # Task tracking skill
  wm/SKILL.md         # Working memory skill
  sg/SKILL.md         # Superego skill
```

## Requirements

- Codex CLI
- Homebrew or Cargo (for binary installation)
- macOS or Linux
