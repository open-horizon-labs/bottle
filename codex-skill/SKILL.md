---
name: bottle
description: Open Horizon Labs orchestration. "$bottle init" for setup, "$bottle dive" to start sessions, "$bottle web-context" for fresh docs.
---

# Bottle - Open Horizon Labs Orchestration

Bottle is a meta-package that orchestrates the Open Horizon Labs tools:
- **ba** - Task tracking (backlog automaton)
- **wm** - Working memory (context and learnings)
- **superego** - Metacognitive oversight

## Standard Commands

These commands invoke the bottle CLI. If bottle is not installed, show:
```
bottle not installed. Install with:
  brew tap open-horizon-labs/homebrew-tap && brew install bottle
  # or: cargo install bottle
```

### $bottle init

Install a bottle ecosystem and set up Codex integration.

```bash
bottle install stable && bottle integrate codex
```

This installs the stable bottle (ba, wm, sg tools) and sets up all Codex skills.

### $bottle status

Show current bottle state.

```bash
bottle status
```

### $bottle update

Update to the latest bottle snapshot.

```bash
bottle update
```

### $bottle remove

Exit bottle management.

```bash
bottle eject
```

---

## Codex-Native Features

These features leverage Codex-specific capabilities not available in Claude Code or OpenCode.

### $bottle dive [intent]

Start a focused work session with explicit intent. This is the **recommended way to begin any work**.

This is a shortcut to `$wm dive [intent]` - see that skill for the full agent flow.

**Intent options:**
- `fix` - Bug fix session
- `plan` - Design/architecture work
- `explore` - Understanding code
- `ship` - Getting changes merged

**Quick version:**

1. Gather context:
```bash
git status && git log --oneline -3
wm compile
```

2. Write `.wm/dive_context.md` with intent, context, and workflow

3. Confirm: "Dive prepped. Intent: [intent]. Ready to work."

**No dive is too small.** Even a quick bug fix benefits from 30 seconds of explicit intent.

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

### $bottle agents

Show the AGENTS.md content that bottle recommends. This is for reference or manual addition.

**Output the contents of the AGENTS.md.snippet file** (bundled with this skill).
