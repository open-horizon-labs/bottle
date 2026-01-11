---
name: bottle
description: Cloud Atlas AI orchestration. "$bottle init" for setup, "$bottle dive" to start sessions, "$bottle web-context" for fresh docs.
---

# Bottle - Cloud Atlas AI Orchestration

Bottle is a meta-package that orchestrates the Cloud Atlas AI tools:
- **ba** - Task tracking (backlog automaton)
- **wm** - Working memory (context and learnings)
- **superego** - Metacognitive oversight

## $bottle init

Set up all Cloud Atlas AI tools for this project. Handles binaries, skills, and AGENTS.md.

### Step 1: Check/install binaries

```bash
echo "Checking Cloud Atlas AI binaries..."

# Check each binary
for cmd in ba wm sg; do
  if command -v $cmd >/dev/null; then
    echo "✓ $cmd installed: $($cmd --version 2>/dev/null || echo 'available')"
  else
    echo "✗ $cmd not found"
  fi
done
```

**If any binary is missing**, offer installation:

If **Homebrew** available (check with `command -v brew`):
```bash
brew tap cloud-atlas-ai/tap
brew install ba wm superego
```

If **Cargo** available (check with `command -v cargo`):
```bash
cargo install ba working-memory superego
```

If **neither available**, tell user:
"You need either Homebrew or Cargo to install the tools. Install Homebrew with:
```bash
/bin/bash -c \"$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)\"
```
Or install Rust (includes Cargo) with:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```"

### Step 2: Install/update child skills

```bash
SKILL_BASE="$HOME/.codex/skills"
BOTTLE_RAW="https://raw.githubusercontent.com/open-horizon-labs/bottle/main/codex-skill"
SUPEREGO_RAW="https://raw.githubusercontent.com/cloud-atlas-ai/superego/main/codex-skill"

echo "Installing Cloud Atlas AI skills..."

# Install ba skill (from bottle repo)
mkdir -p "$SKILL_BASE/ba"
curl -fsSL -o "$SKILL_BASE/ba/SKILL.md" "$BOTTLE_RAW/ba/SKILL.md"

# Install wm skill (from bottle repo)
mkdir -p "$SKILL_BASE/wm"
curl -fsSL -o "$SKILL_BASE/wm/SKILL.md" "$BOTTLE_RAW/wm/SKILL.md"

# Install superego skill (from superego repo)
mkdir -p "$SKILL_BASE/superego"
for file in SKILL.md AGENTS.md.snippet; do
  curl -fsSL -o "$SKILL_BASE/superego/$file" "$SUPEREGO_RAW/$file"
done
mkdir -p "$SKILL_BASE/superego/agents"
for agent in code.md writing.md learning.md; do
  curl -fsSL -o "$SKILL_BASE/superego/agents/$agent" "$SUPEREGO_RAW/agents/$agent"
done

echo "✓ Skills installed"
```

### Step 3: Initialize tool directories

```bash
echo "Initializing tools..."

# ba
if [ ! -d ".ba" ]; then
  ba init
  echo "✓ ba initialized"
else
  echo "✓ .ba/ already exists"
fi

# wm
if [ ! -d ".wm" ]; then
  wm init
  echo "✓ wm initialized"
else
  echo "✓ .wm/ already exists"
fi

# superego
if [ ! -d ".superego" ]; then
  sg init
  echo "✓ superego initialized"
else
  echo "✓ .superego/ already exists"
fi
```

### Step 4: Configure superego mode

```bash
# Set superego to pull mode (recommended)
if [ -f ".superego/config.yaml" ]; then
  if grep -q "^mode: always" .superego/config.yaml; then
    sed -i.bak 's/^mode: always/mode: pull/' .superego/config.yaml
    rm -f .superego/config.yaml.bak
    echo "✓ Superego set to pull mode (less intrusive)"
  fi
fi
```

### Step 5: Create/update AGENTS.md

Ask user: "Create AGENTS.md with Cloud Atlas AI protocols? This includes dive-first workflow and tool guidance. [Y/n]"

**If yes**, create AGENTS.md with content from the AGENTS.md.snippet (see $bottle agents).

**If AGENTS.md already exists**, ask: "AGENTS.md exists. Append Cloud Atlas AI section? [Y/n]"

### Step 6: Confirm completion

Tell user:
```
✓ Bottle initialization complete

Installed:
  • ba - Task tracking ($ba)
  • wm - Working memory ($wm)
  • superego - Metacognitive oversight ($superego)

Quick start - dive first:
  $bottle dive fix       # Start a bug fix session
  $ba status             # Check your tasks
  $superego              # Get feedback at decision points

No dive is too small for a dive prep. The 30 seconds of setup
prevents 30 minutes of drift.
```

## $bottle status

Check installation status of all Cloud Atlas AI tools.

```bash
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
  if grep -q "Cloud Atlas AI" AGENTS.md; then
    echo "AGENTS.md: ✓ Contains Cloud Atlas AI guidance"
  else
    echo "AGENTS.md: ⚠ Exists but missing Cloud Atlas AI section"
  fi
else
  echo "AGENTS.md: ✗ Not found (run \$bottle init)"
fi
```

## $bottle dive [intent]

Start a focused work session with explicit intent. This is the **recommended way to begin any work**.

**Intent options:**
- `fix` - Bug fix session
- `plan` - Design/architecture work
- `explore` - Understanding code
- `ship` - Getting changes merged

**Run:**
```bash
wm dive-prep --intent <intent>
```

This creates `.wm/dive_context.md` with:
- Your explicit intent
- Relevant context from working memory
- Suggested workflow for this type of work

**Tell user:** "Dive prepped. Intent: [intent]. Context loaded to .wm/dive_context.md"

**No dive is too small.** Even a quick bug fix benefits from 30 seconds of explicit intent.

## $bottle update

Update all Cloud Atlas AI skills and binaries.

Same as running `$bottle init` - it handles both initial setup and updates.

## $bottle agents

Show the AGENTS.md content that bottle recommends. This is for reference or manual addition.

**Output the contents of the AGENTS.md.snippet file:**

```markdown
# Cloud Atlas AI Stack

## Quick Start: Dive First

**No dive is too small for a dive prep.** The metaphor comes from scuba diving: you prep before you dive, you don't just splash in. Even a quick bug fix benefits from explicit intent.

Start every session with a dive:
```
$bottle dive fix       # Bug fix
$bottle dive plan      # Design work
$bottle dive explore   # Understanding code
```

This creates `.wm/dive_context.md` with your intent, relevant context, and suggested workflow. The 30 seconds of setup prevents 30 minutes of drift.

---

## Task Tracking ($ba)

**When to use:**
- At session start: `$ba status` to see active tasks
- Before starting work: Check what's ready to claim
- When creating tasks: `ba create` for each distinct piece of work
- During work: `ba claim` to take ownership, `ba finish` when done

**Protocol:** Always track non-trivial work. If a task has multiple steps or will take >5 minutes, create a task.

## Working Memory ($wm)

**When to use:**
- Starting work: `$bottle dive <intent>` to prep context
- Need context: `wm compile` for relevant knowledge
- After completing work: `wm distill` to extract learnings
- Questions about past work: Check `wm compile` first

**Dive terminology:**
- **dive-prep** = preparing context before work
- **dive pack** = reusable bundle for a type of work
- **dive context** = session manifest from dive-prep

**Protocol:** Treat wm as your external memory. Don't guess at past decisions - check wm first.

## Metacognition ($superego)

**Mode:** Pull mode - evaluates only when explicitly requested.

**When to use $superego:**
- Before committing to a plan or approach
- When choosing between alternatives
- Before non-trivial implementations
- When the task feels complex or uncertain
- Before claiming work is "done"

**Protocol:** Superego is opt-in. Use it for high-stakes decisions, architectural choices, or when you want a second opinion. It catches premature commitment, scope creep, and misalignment.

**Results:** `has_concerns: true` = STOP and show user; `has_concerns: false` = continue.
```

## $bottle remove

Remove Cloud Atlas AI setup from this project (keeps binaries and user skills).

**Step 1:** Remove project directories:
```bash
rm -rf .ba/ .wm/ .superego/
```

**Step 2:** Remove Cloud Atlas AI section from AGENTS.md if present.

**Step 3:** Confirm: "Cloud Atlas AI removed from this project. Binaries and user skills preserved."

---

## Codex-Native Features

These features leverage Codex-specific capabilities not available in Claude Code or OpenCode.

### $bottle codex-sync

Sync knowledge from recent Codex sessions into working memory.

**Steps:**
1. Find recent Codex sessions:
```bash
SESSIONS_DIR="$HOME/.codex/sessions"
find "$SESSIONS_DIR" -name "rollout-*.jsonl" -mtime -7 | tail -5
```

2. Parse sessions for decisions, learnings, and context
3. Feed to `wm distill` for knowledge extraction

**When to use:**
- After productive Codex sessions
- Before resuming work (`codex resume`)
- Weekly knowledge consolidation

### $bottle web-context <query>

Augment working memory with fresh web results (uses Codex web search).

**Steps:**
1. Get accumulated knowledge: `wm compile`
2. Search web for: `<query>`
3. Combine project knowledge with fresh documentation

**Example:**
```
$bottle web-context "rust async patterns 2026"
```

**When to use:**
- Working with unfamiliar libraries
- Need current API documentation
- Combining project history with best practices

### $bottle resume

Prepare context for resuming a previous Codex session.

**Steps:**
1. Find the most recent Codex session
2. Parse what was being worked on
3. Combine with `wm compile` output
4. Show summary of where you left off

Integrates with `codex resume` for seamless continuity.
