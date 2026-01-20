---
name: bottle
description: Open Horizon Labs meta-tool. One entry point for ba, wm, and superego. Use "bottle-init" to set up all tools, "bottle-status" to check state.
license: MIT
---

# Bottle - Open Horizon Labs Meta-Tool

Bottle provides the complete Open Horizon Labs stack with one entry point.

## Commands

### bottle-init

Initialize all Open Horizon Labs tools with recommended defaults.

→ *See [reference/init.md](reference/init.md) for detailed steps*

**What it does:**
1. Installs binaries (ba, wm, sg) via Homebrew or Cargo
2. Initializes `.ba/`, `.wm/`, `.superego/` directories
3. Sets superego to pull mode (recommended)
4. Creates/updates AGENTS.md

**Run:** Follow the init reference for step-by-step execution.

### bottle-status

Show status of all Open Horizon Labs tools.

→ *See [reference/status.md](reference/status.md)*

**Quick check:**
```bash
ba list --status in_progress  # Your active tasks
sg status                      # Superego mode
wm show                        # Working memory state
```

### bottle-install

Install individual tool binaries.

→ *See [reference/install.md](reference/install.md)*

### bottle-update

Update all tool binaries to latest versions.

→ *See [reference/update.md](reference/update.md)*

### bottle-integrate

Set up integration for a specific platform (claude_code, codex, opencode).

→ *See [reference/integrate.md](reference/integrate.md)*

### bottle-help

Show available bottle commands and quick reference.

→ *See [reference/help.md](reference/help.md)*

## The Tools

Bottle orchestrates these Open Horizon Labs tools:

| Tool | Purpose |
|------|---------|
| **ba** | Task tracking for LLM sessions |
| **wm** | Working memory capture and recall |
| **superego** | Metacognitive oversight |

## Quick Start

```bash
# Initialize everything
bottle-init

# Check what's running
bottle-status

# Start working
ba ready                        # See available tasks
ba claim <id> --session $SESSION_ID  # Take a task
```

## Installation

```bash
brew tap open-horizon-labs/homebrew-tap && brew install bottle
bottle install stable
bottle integrate opencode
```

Then run `bottle-init` in your project.
