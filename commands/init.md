# Bottle Init - Unified Cloud Atlas AI Setup

Initialize all installed Cloud Atlas AI tools with recommended defaults.

## Step 1: Initialize individual tools

Run the init command for each installed tool:

**ba:**
- Check if ba binary exists: `command -v ba`
- If yes and `.ba/` doesn't exist, run: `ba init`
- This creates `.ba/` directory and initial configuration

**superego:**
- Check if sg binary exists: `command -v sg`
- If yes and `.superego/` doesn't exist, run: `sg init`
- This creates `.superego/` directory and sets up metacognitive monitoring

**wm:**
- Check if wm binary exists: `command -v wm`
- If yes and `.wm/` doesn't exist, run: `wm init`
- This creates `.wm/` directory and enables working memory capture

## Step 2: Apply recommended defaults

After individual inits complete:

**Set superego to pull mode (recommended):**
- Check if `.superego/config.yaml` exists
- Read the current mode setting
- If mode is `always`, change it to `pull`
- Command: `sed -i.bak 's/^mode: always/mode: pull/' .superego/config.yaml && rm .superego/config.yaml.bak`
- Explain: "Pull mode is less intrusive - superego reviews when you request it or before commits/PRs, rather than at every checkpoint"

## Step 3: Create/update AGENTS.md

Create AGENTS.md with guidance for all initialized tools:
- Add sections for ba, wm, and superego
- Include "When to use" and "Protocol" guidance for each
- If AGENTS.md already exists, preserve user content while updating tool sections

## Step 4: Confirm completion

Tell user:
```bash
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
