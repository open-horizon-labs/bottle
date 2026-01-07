# Dive Session: Bottle Design

**Intent:** plan
**Started:** 2026-01-06T20:20:00Z
**Focus:** Design unified installer/updater for Cloud Atlas AI ecosystem

## Context

### Project
Bottle is a meta-installer for the Cloud Atlas AI ecosystem. The goal is to provide a single entry point for installing, configuring, and updating five tools that together provide 10-100x AI leverage:

- superego - Metacognitive advisor
- wm - Working memory / tacit knowledge extraction
- ba - Task tracking for AI sessions
- oh-mcp - Open Horizons MCP for strategic alignment
- miranda - Telegram bot for remote Claude orchestration

### Problem Being Solved
Users currently face:
- 4 different repos to discover and install from
- Different installation methods (cargo, npm, plugins)
- Unclear dependencies and ordering
- No coordinated updates
- Scattered documentation

Quote from the user: "Being early is the same as being late, so we need to figure out what to bottle now."

### Constraints
- DO NOT duplicate tool logic - thin orchestration only
- Tools must remain independent (separate repos)
- Each tool must work standalone
- Graceful degradation (partial installs OK)
- Follow Unix philosophy (compose via shell, no coupling)

### Relevant Knowledge

#### Installation Patterns Discovered

**superego** (Rust binary + plugin):
```
1. Binary: brew install cloud-atlas-ai/superego/superego OR cargo install superego
2. Plugin: /plugin marketplace add + /plugin install superego@superego
3. Init: /superego:init (detects binary, offers install via dialog)
4. Per-project: sg init creates .superego/
```

**wm** (Rust binary + plugin):
```
1. Binary: brew tap cloud-atlas-ai/wm && brew install wm OR cargo install wm
2. Plugin: claude plugin install wm
3. Init: wm init creates .wm/
4. Hook: Triggered by superego's stop hook (background)
```

**ba** (Rust binary, no plugin):
```
1. Binary: cargo install ba
2. Init: ba init creates .ba/
3. Simple, self-contained
```

**oh-mcp** (Node.js, npx-based + plugin):
```
1. No binary install: npx -y @cloud-atlas-ai/oh-mcp-server
2. Plugin: /plugin marketplace add + install oh-mcp@oh-mcp
3. Setup: /oh-mcp:setup adds to claude mcp config
4. Requires: OH_API_KEY env var + OH account
```

**miranda** (Node.js server + plugin):
```
1. Server: pnpm install + pnpm start (requires Telegram bot token)
2. Plugin: /plugin marketplace add + install miranda@miranda
3. Skills: /mouse (autonomous task worker) + /drummer (batch PR merger)
4. Dependencies: ba + superego (on server where Claude runs)
5. Different: Server component for remote orchestration, not a local CLI
```

#### Dependencies (Runtime)
- superego → calls wm extract (background, optional)
- wm → works with superego (optional but recommended)
- oh-mcp → independent, but pairs with superego for strategic grounding
- ba → independent
- miranda → requires ba + superego (on server), optional for local dev

#### Install Order (Recommended)
1. ba (simple, no dependencies)
2. wm (can work alone, but superego enhances it)
3. superego (calls wm, so wm should exist first)
4. oh-mcp (strategic layer, independent but synergistic)
5. miranda (server component, optional, requires 1-3)

## Workflow: Plan Mode

1. ✓ Review available context (local docs, OH mission if available)
2. → Identify options and trade-offs
3. Draft plan with concrete steps (no time estimates)
4. Surface risks and dependencies
5. Document decision rationale
6. Validate with user

## Sources
- Local: bottle/.gitignore, README.md (created)
- Git: master branch (initialized)
- Explored: superego, wm, ba, oh-mcp-server READMEs
- ba: task kk-pwfi (completed), kk-6tvk (in_progress)
