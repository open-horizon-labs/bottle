# Bottle Init - Unified Cloud Atlas AI Setup

Initialize all installed Cloud Atlas AI tools with recommended defaults.

## Step 1: Initialize individual tools

Run the init command for each installed tool:

**ba:**
- Call `/ba:init` if ba plugin is installed
- This handles ba binary installation, project setup, and AGENTS.md

**superego:**
- Call `/superego:init` if superego plugin is installed
- This handles sg binary installation and project setup

**wm:**
- Check if wm binary exists: `command -v wm`
- If yes and `.wm/` doesn't exist, run: `wm init`
- wm works automatically once installed - no separate plugin commands needed

## Step 2: Apply recommended defaults

After individual inits complete:

**Set superego to pull mode (recommended):**
- Check if `.superego/config.yaml` exists
- Read the current mode setting
- If mode is `always`, change it to `pull`
- Command: `sed -i.bak 's/^mode: always/mode: pull/' .superego/config.yaml && rm .superego/config.yaml.bak`
- Explain: "Pull mode is less intrusive - superego reviews when you request it or before commits/PRs, rather than at every checkpoint"

## Step 3: Verify AGENTS.md

Check if AGENTS.md has guidance for all initialized tools:
- `/ba:init` should have added ba section
- Verify it's present and complete
- If anything is missing, offer to update

## Step 4: Confirm completion

Tell user:
```
✓ Bottle initialization complete

Initialized:
  • ba - Task tracking ready
  • superego - Metacognitive advisor active (pull mode)
  • wm - Working memory enabled

Configuration:
  • superego mode: pull (recommended)
  • AGENTS.md: Updated with tool guidance

Quick start:
  ba create "Your first task" -t task
  ba ready
  ba claim <id> --session $SESSION_ID

Tools work together:
  • ba tracks your work
  • superego reviews before commits (/superego:review)
  • wm captures learnings automatically
```

---

Run each tool's init command in sequence. Apply recommended defaults after inits complete. Focus on orchestration, not reimplementation.
