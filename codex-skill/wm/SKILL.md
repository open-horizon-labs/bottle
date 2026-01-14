---
name: wm
description: Working memory. "$wm dive" to prep sessions, "$wm compile" for context, "$wm distill" to extract learnings.
---

# wm - Working Memory for AI Sessions

Captures tacit knowledge from sessions and provides relevant context for current work.

All commands invoke the `wm` CLI. If wm is not installed, show:
```
wm not installed. Install with:
  brew tap open-horizon-labs/homebrew-tap && brew install wm
  # or: cargo install working-memory
```

## $wm compile

Get relevant context for current work. Synthesizes knowledge from past sessions.

```bash
wm compile
```

## $wm distill

Extract learnings from completed work sessions.

```bash
wm distill
```

## $wm init

Initialize working memory for this project.

```bash
wm init
```

## $wm show state

Display the accumulated knowledge state.

```bash
wm show state
```

## $wm show sessions

List recent work sessions.

```bash
wm show sessions
```

## $wm dive save <name>

Save current dive context as a named prep.

```bash
wm dive save <name>
```

## $wm dive list

List all saved dive preps.

```bash
wm dive list
```

## $wm dive switch <name>

Switch to a named dive prep.

```bash
wm dive switch <name>
```

## $wm dive show

Show current dive prep content.

```bash
wm dive show
```

---

## Codex-Native Features

These features leverage Codex-specific capabilities.

### $wm dive [intent]

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
git status
git log --oneline -5
cat AGENTS.md 2>/dev/null || cat CLAUDE.md 2>/dev/null
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
Dive session prepared
  Intent: [intent]
  Context: .wm/dive_context.md

Ready to work.
```

**No dive is too small.** Even a quick bug fix benefits from 30 seconds of explicit intent.

### $wm compile --search <query>

Augment working memory with fresh web results (uses Codex web search).

**Steps:**
1. Run `wm compile` to get accumulated knowledge
2. Use Codex web search for fresh documentation on `<query>`
3. Combine both into comprehensive context

### $wm codex-sessions

Analyze recent Codex sessions for knowledge extraction.

**Steps:**
1. Find recent sessions in `~/.codex/sessions/`
2. Extract key decisions and learnings
3. Feed to `wm distill`

### $wm resume-context

Get context for resuming a previous Codex session. Integrates with `codex resume`.

---

## Dive Terminology

- **dive-prep** = the action of preparing context before work
- **dive pack** = a reusable bundle of context for a type of work
- **dive context** = the session-specific manifest created by dive-prep

## Workflow Integration

1. **Before work:** `$wm dive <intent>` - prep context
2. **During work:** `$wm compile` - get relevant knowledge as needed
3. **At decision points:** `$sg review` - metacognitive review
4. **After work:** `$wm distill` - capture learnings

**Protocol:** Treat wm as your external memory. Don't guess at past decisions - check wm first.
