# Bottle

**Open Horizon Labs - One entry point for all the tools**

Bottle provides the complete Open Horizon Labs stack for **Claude Code**, **OpenCode**, and **Codex**.

## Quickstart

```bash
brew tap open-horizon-labs/homebrew-tap && brew install bottle
bottle install stable
bottle integrate claude_code  # or: codex, opencode
```

Then initialize in your project:
```
/bottle:init          # Claude Code
$bottle init          # Codex
bottle-init           # OpenCode
```

This installs binaries, initializes `.ba/`, `.wm/`, `.superego/`, and creates `AGENTS.md`.

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

## Platform-Specific Docs

For detailed installation and usage per platform:
- **Claude Code**: See [.claude-plugin/commands/getting-started.md](.claude-plugin/commands/getting-started.md)
- **OpenCode**: See [opencode-plugin/README.md](./opencode-plugin/README.md)
- **Codex**: See [codex-skill/README.md](./codex-skill/README.md)

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

See platform-specific docs for manual update commands.

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

## License

Source-available. See individual repos for license details.
