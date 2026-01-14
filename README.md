# Bottle

**Open Horizon Labs - One entry point for all the tools**

Bottle provides the complete Open Horizon Labs stack for **Claude Code**, **OpenCode**, and **Codex**.

## Quickstart

```bash
# Install bottle CLI
cargo install bottle
# Or via Homebrew (macOS):
# brew install open-horizon-labs/homebrew-tap/bottle

# Install the tool stack (ba, wm, sg)
bottle install stable

# Integrate with your AI coding assistant
bottle integrate claude_code  # or: opencode, codex
```

Then in your project: `/bottle:init`

---

## Ecosystem Vision

Bottle is part of the Open Horizon Labs ecosystem - tools that give you AI superpowers without surrendering your mind. The ecosystem has two delivery mechanisms serving different audiences:

```
Open Horizons (strategic layer - missions, guardrails, metis)
    |
    +-- Bottle (developer tools)
    |       CLI tools that augment AI coding assistants
    |       You use these TO BUILD software
    |
    +-- Memex (end user product)
            Desktop app that IS the AI interface
            Knowledge workers use this directly
```

**Bottle tools** augment Claude Code, OpenCode, and Codex with:
- **Intentional grounding** (wm/dive-prep) - Start sessions with relevant context
- **Semantic recall** (datasphere) - Search past knowledge
- **Metacognitive oversight** (superego) - Catch drift before it compounds
- **Strategic alignment** (oh-mcp) - Connect to your mission hierarchy
- **Task tracking** (ba) - Simple work management
- **Remote orchestration** (miranda) - Control sessions via Telegram

**Memex** implements the same concepts natively for end users:
- **Dive** = intentional grounding (native)
- **Datasphere** = semantic recall (native)
- **Superego** = metacognitive oversight (native)
- **OH-local** = strategic alignment (native)

The philosophy is shared: grounding prevents drift, recall surfaces relevant knowledge, oversight catches mistakes, and strategic context keeps you aligned with what matters.

We use bottle tools to build Memex. Memex will be the finished product for knowledge workers who want AI without the command line.

---

## Tools

- **ba** - Task tracking for AI sessions
- **wm** - Working memory for automatic tacit knowledge extraction
- **superego** - Metacognitive advisor
- **datasphere** - Knowledge graph from Claude Code sessions (Claude Code only)
- **oh-mcp** - Open Horizons MCP for strategic alignment (Claude Code only)
- **miranda** - Telegram bot for remote orchestration (Claude Code only)

---

## OpenCode Installation

**Recommended: Use bottle CLI**

```bash
# Install bottle CLI first (see Quickstart above)
bottle install stable
bottle integrate opencode
```

This adds the bottle ecosystem plugins to your `opencode.json`:
- `@cloud-atlas-ai/bottle` - CLI wrapper and orchestration
- `ba-opencode` - Task tracking
- `wm-opencode` - Working memory
- `superego-opencode` - Metacognition

**Note:** Restart OpenCode after integration to load the plugins.

**Initialize in your project:**
```
bottle-init
```

See [opencode-plugin/README.md](./opencode-plugin/README.md) for full OpenCode documentation.

---

## Claude Code Installation

**Recommended: Use bottle CLI**

```bash
# Install bottle CLI first (see Quickstart above)
bottle install stable
bottle integrate claude_code
```

This installs the `bottle@open-horizon-labs` plugin which provides all `/bottle:*` commands.

**Initialize in your project:**
```
/bottle:init
```

This will:
- Detect and install missing binaries (ba, wm, sg) via cargo or homebrew
- Initialize tool directories (.ba/, .wm/, .superego/)
- Set recommended defaults (e.g., superego pull mode)
- Create/update AGENTS.md with tool guidance

---

## Codex Installation

**Recommended: Use bottle CLI**

```bash
# Install bottle CLI first (see Quickstart above)
bottle install stable
bottle integrate codex
```

This installs all Open Horizon Labs skills (bottle, ba, wm, sg) to `~/.codex/skills/`.

**Initialize in Codex:**
```
$bottle init
```

**Usage:**
```
$bottle dive fix       # Start a bug fix session
$bottle dive plan      # Start a planning session
$ba status             # Check your tasks
$sg review             # Get feedback at decision points
```

**Note:** Codex skills are advisory-only (no event hooks). The guidance in AGENTS.md tells Codex when to invoke skills.

See [codex-skill/README.md](./codex-skill/README.md) for full Codex documentation.

---

## What Each Tool Does

### ba - Task Tracking
Simple, file-based task tracking. No server, no database.

**Commands:** `/ba:init`, `/ba:quickstart`, `/ba:status`

### wm - Working Memory
Automatically captures tacit knowledge from sessions and injects relevant context.

**Setup:** Installs plugin + binary, works automatically

### superego - Metacognition  
Monitors Claude's work and provides feedback before finishing or making large changes.

**Commands:** `/superego:init`, `/superego:review`, `/superego:prompt`

### datasphere - Knowledge Graph
Builds a searchable knowledge graph from Claude Code sessions, making insights from past sessions queryable.

**Commands:** `/datasphere:init`, `/datasphere:setup`

### oh-mcp - Strategic Alignment
Connects Claude Code to Open Horizons for strategic context.

**Setup:** `/oh-mcp:setup` (requires OH account + API key)

### miranda - Remote Orchestration
Telegram bot for running Claude sessions remotely.

**Setup:** Server component, see [miranda docs](https://github.com/cloud-atlas-ai/miranda)

## Updating

```bash
bottle update
```

This updates all tools to the latest versions in the current bottle.

## Individual Repos

Each tool has its own repo with detailed documentation:

- [ba](https://github.com/open-horizon-labs/ba)
- [wm](https://github.com/open-horizon-labs/wm)
- [superego](https://github.com/open-horizon-labs/superego)
- [datasphere](https://github.com/open-horizon-labs/datasphere)
- [oh-mcp-server](https://github.com/open-horizon-labs/oh-mcp-server)
- [miranda](https://github.com/open-horizon-labs/miranda)

## Plugin Sync

The Claude Code plugins in `.claude-plugin/plugins/` are synced from their source repositories via a GitHub Action. To update to the latest plugin versions, run the "Sync Claude Code Plugins" workflow manually from the Actions tab.

## AI Agent Bootstrap

**For AI agents:** Follow these steps to set up the Open Horizon Labs stack.

### Quick Setup

```bash
# 1. Install bottle CLI
cargo install bottle
# Or via Homebrew: brew install open-horizon-labs/homebrew-tap/bottle

# 2. Install the stable tool stack
bottle install stable

# 3. Integrate with your platform
bottle integrate claude_code   # For Claude Code
bottle integrate codex         # For Codex
bottle integrate opencode      # For OpenCode
```

### What Gets Installed

**CLI Tools:**
- `ba` - Task tracking for AI sessions
- `wm` - Working memory (context extraction)
- `sg` - Superego (metacognitive advisor)

**MCP Servers:**
- `oh-mcp` - Open Horizons MCP server for strategic alignment

**Platform integrations add:**
- Claude Code: `/bottle:*` commands (from bottle@open-horizon-labs plugin)
- Codex: `$bottle`, `$ba`, `$wm`, `$sg` skills
- OpenCode: `bottle-*`, `ba-*`, `wm-*`, `superego-*` tools

### Verify Installation

```bash
bottle status
```

### Initialize Tools in Project

After platform integration, run in your project:

```
/bottle:init          # Claude Code
$bottle init          # Codex
bottle-init           # OpenCode
```

This initializes `.ba/`, `.wm/`, `.superego/` and creates `AGENTS.md`.

### Platform Detection

Bottle auto-detects installed platforms:
- Claude Code: `~/.claude/` exists
- Codex: `~/.codex/` exists
- OpenCode: `~/.config/opencode/` or `opencode.json` exists

Run `bottle integrate --list` to see detected platforms.

### Troubleshooting

**No cargo?** Install Rust first:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

**Homebrew alternative** (macOS):
```bash
brew install open-horizon-labs/homebrew-tap/bottle
```

## License

Source-available. See individual repos for license details.
