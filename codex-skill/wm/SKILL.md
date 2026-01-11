---
name: wm
description: Working memory. "$wm dive" to prep sessions, "$wm compile" for context, "$wm distill" to extract learnings.
---

# wm - Working Memory for AI Sessions

Captures tacit knowledge from sessions and provides relevant context for current work.

## $wm dive [intent]

**Start every session with a dive.** This is an agent flow, not a single CLI command.

**Intent options:**
- `fix` - Bug fix session
- `plan` - Design/architecture work
- `explore` - Understanding code
- `ship` - Getting changes merged
- `review` - Reflect on recent work

### Dive Prep Flow

**Step 1:** If intent not provided, ask user:
```
What's your intent for this session?
[ ] fix - Fix a bug or issue
[ ] plan - Design an approach
[ ] explore - Understand something
[ ] ship - Get something deployed
```

**Step 2:** Gather context from available sources:
```bash
# Check git state
git status
git log --oneline -5

# Check for project instructions
cat AGENTS.md 2>/dev/null || cat CLAUDE.md 2>/dev/null

# Get accumulated knowledge
wm compile
```

**Step 3:** Build and write the dive manifest:

Create `.wm/dive_context.md` with:
```markdown
# Dive Session

**Intent:** [intent]
**Started:** [timestamp]

## Context
[Project instructions, git state, relevant knowledge]

## Focus
[What we're working on]

## Workflow
[Steps for this intent type]
```

**Step 4:** Confirm to user:
```
✓ Dive session prepared
  Intent: [intent]
  Context: .wm/dive_context.md

Ready to work.
```

### Named Dive Preps (CLI)

After creating a dive context, save it as a named prep:
```bash
wm dive save my-feature    # Save current context as named prep
wm dive list               # List all preps (* marks current)
wm dive switch my-feature  # Switch to a named prep
wm dive show               # Show current prep content
```

**No dive is too small.** Even a quick bug fix benefits from 30 seconds of explicit intent.

## $wm compile

Get relevant context for current work. Synthesizes knowledge from past sessions.

**Run:**
```bash
wm compile
```

**Output:** Working set of relevant knowledge for your current state.

**When to use:**
- Starting work on unfamiliar code
- Before answering questions about past decisions
- When you need context about why something was built a certain way

## $wm compile --search <query>

**Codex-native feature:** Augment working memory with fresh web results.

**Steps:**
1. Run `wm compile` to get accumulated knowledge
2. Use Codex web search for fresh documentation on `<query>`
3. Combine both into comprehensive context

**Example:**
```
$wm compile --search "tokio async patterns"
```

This gives you:
- Past project knowledge about async code
- Fresh documentation from the web

**When to use:**
- Working with unfamiliar libraries
- Need up-to-date API documentation
- Combining project history with current best practices

## $wm distill

Extract learnings from completed work sessions.

**Run:**
```bash
wm distill
```

**When to use:**
- After completing a significant piece of work
- End of a focused session
- When you've learned something worth preserving

**What it captures:**
- Decisions made and rationale
- Patterns discovered
- Gotchas and workarounds

## $wm show state

Display the accumulated knowledge state.

**Run:**
```bash
wm show state
```

## $wm show sessions

List recent work sessions.

**Run:**
```bash
wm show sessions
```

## $wm init

Initialize working memory for this project.

**Step 1:** Check if wm binary is installed:
```bash
if ! command -v wm >/dev/null; then
  echo "wm binary not installed. Install with:"
  echo "  brew install cloud-atlas-ai/tap/wm"
  echo "  # or: cargo install working-memory"
  exit 1
fi
```

**Step 2:** Initialize .wm/ directory:
```bash
if [ ! -d ".wm" ]; then
  wm init
  echo "✓ .wm/ initialized"
else
  echo "✓ .wm/ already exists"
fi
```

## $wm codex-sessions

**Codex-native feature:** Analyze recent Codex sessions for knowledge extraction.

Codex stores sessions in `~/.codex/sessions/YYYY/MM/DD/rollout-*.jsonl`.

**Steps:**
1. Find recent Codex sessions:
```bash
SESSIONS_DIR="$HOME/.codex/sessions"
RECENT=$(find "$SESSIONS_DIR" -name "rollout-*.jsonl" -mtime -7 | head -10)
echo "Recent Codex sessions:"
echo "$RECENT"
```

2. For each session, extract key decisions and learnings
3. Feed extracted content to `wm distill`

**When to use:**
- After a productive Codex session
- When resuming work (`codex resume`) and want full context
- Weekly knowledge consolidation

## $wm resume-context

**Codex-native feature:** Get context for resuming a previous session.

Integrates with `codex resume` to provide continuity.

**Steps:**
1. Check for recent Codex sessions:
```bash
LATEST=$(find "$HOME/.codex/sessions" -name "rollout-*.jsonl" -mtime -1 | tail -1)
```

2. Parse the session for:
   - What was being worked on
   - Where it left off
   - Open questions/blockers

3. Combine with `wm compile` output

**Tell user:** Summary of previous session context, ready to continue.

---

## Dive Terminology

- **dive-prep** = the action of preparing context before work
- **dive pack** = a reusable bundle of context for a type of work
- **dive context** = the session-specific manifest created by dive-prep

## Workflow Integration

Working memory integrates with the full Cloud Atlas AI stack:

1. **Before work:** `$wm dive <intent>` - prep context
2. **During work:** `$wm compile` - get relevant knowledge as needed
3. **At decision points:** `$superego` - metacognitive review
4. **After work:** `$wm distill` - capture learnings

**Protocol:** Treat wm as your external memory. Don't guess at past decisions - check wm first.
