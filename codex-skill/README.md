# Bottle Codex Skill

Codex skill for orchestrating the Cloud Atlas AI tool suite.

## Installation

### Quick Install (Recommended)

```bash
# Install the bottle skill
mkdir -p ~/.codex/skills/bottle
curl -fsSL -o ~/.codex/skills/bottle/SKILL.md \
  https://raw.githubusercontent.com/cloud-atlas-ai/bottle/main/codex-skill/SKILL.md
curl -fsSL -o ~/.codex/skills/bottle/AGENTS.md.snippet \
  https://raw.githubusercontent.com/cloud-atlas-ai/bottle/main/codex-skill/AGENTS.md.snippet
```

Then in Codex:
```
$bottle init
```

### Manual Install

1. Clone this repo
2. Copy `codex-skill/` contents to `~/.codex/skills/bottle/`
3. Run `$bottle init` in Codex

## Usage

### Commands

| Command | Description |
|---------|-------------|
| `$bottle init` | Full setup: binaries, skills, AGENTS.md |
| `$bottle status` | Check what's installed |
| `$bottle dive <intent>` | Start a focused session (fix/plan/explore/ship) |
| `$bottle update` | Update skills and binaries |
| `$bottle agents` | Show recommended AGENTS.md content |
| `$bottle remove` | Remove from project (keeps binaries) |

### Quick Start

```
$bottle init              # One-time setup
$bottle dive fix          # Start a bug fix session
$ba status                # Check your tasks
$superego                 # Get feedback at decision points
```

## What Gets Installed

### Binaries (via Homebrew or Cargo)

- `ba` - Task tracking (backlog automaton)
- `wm` - Working memory
- `sg` - Superego metacognitive oversight

### Skills

- `$ba` - Task tracking commands
- `$wm` - Working memory commands
- `$superego` - Metacognitive evaluation
- `$bottle` - Orchestration (this skill)

### Project Files

- `.ba/` - Task tracking data
- `.wm/` - Working memory state
- `.superego/` - Superego configuration
- `AGENTS.md` - Protocol guidance for Codex

## Dive-First Workflow

The recommended way to work:

1. **Start with intent**: `$bottle dive fix` (or plan/explore/ship)
2. **Track your work**: `$ba status`, `ba claim <task>`
3. **Get feedback**: `$superego` at decision points
4. **Extract learnings**: `wm distill` after completing work

No dive is too small. The 30 seconds of setup prevents 30 minutes of drift.

## Differences from Other Platforms

| Aspect | Claude Code | OpenCode | Codex |
|--------|-------------|----------|-------|
| Extension | Plugin | TypeScript plugin | Skill (SKILL.md) |
| Auto-trigger | Yes (hooks) | Yes (hooks) | No (advisory) |
| Distribution | npm | npm | curl from GitHub |
| Config | CLAUDE.md | opencode.json | AGENTS.md |

Codex skills are advisory-only. The guidance in AGENTS.md tells Codex when to invoke skills, but cannot enforce it automatically.

## Files

```
codex-skill/
  SKILL.md            # Main skill definition
  AGENTS.md.snippet   # Content for user's AGENTS.md
  README.md           # This file
  scripts/
    init.sh           # Binary installation helper
    status.sh         # Status check helper
```

## Requirements

- Codex CLI
- Homebrew or Cargo (for binary installation)
- macOS or Linux
