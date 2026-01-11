# Bottle

**Cloud Atlas AI - One entry point for all the tools**

Bottle provides the complete Cloud Atlas AI stack for **Claude Code**, **OpenCode**, and **Codex**.

---

## Ecosystem Vision

Bottle is part of the Cloud Atlas AI ecosystem - tools that give you AI superpowers without surrendering your mind. The ecosystem has two delivery mechanisms serving different audiences:

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

**Bottle tools** augment Claude Code and OpenCode with:
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

**One npm package, full stack:**

```bash
npm install @cloud-atlas-ai/bottle
```

Add to your `opencode.json`:

```json
{
  "$schema": "https://opencode.ai/config.json",
  "plugin": ["@cloud-atlas-ai/bottle"]
}
```

**Initialize** - Ask the AI to initialize the stack:
- "use bottle-init to set up the full Cloud Atlas AI stack"

Bottle will automatically:
- Add child plugins (ba-opencode, wm-opencode, superego-opencode) to opencode.json
- Detect missing binaries (ba, wm, sg)
- Offer to install them via homebrew or cargo
- Initialize each tool
- Set recommended defaults (e.g., superego pull mode)
- Create AGENTS.md with usage guidance
- Install convenience commands (superego-review, wm-dive-prep)

**Note:** After bottle-init updates opencode.json, restart OpenCode to load the child plugins.

**Convenience commands:**
- `superego-review` - Run metacognitive review of current work
- `wm-dive-prep` - Prepare a grounded dive session with context

See [opencode-plugin/README.md](./opencode-plugin/README.md) for full OpenCode documentation.

---

## Claude Code Installation

### 1. Add the Marketplace

```bash
claude plugin marketplace add cloud-atlas-ai/bottle
```

### 2. Install Plugins

Install bottle and any tools you want:

```bash
# Core stack (recommended):
claude plugin install bottle@bottle
claude plugin install ba@bottle
claude plugin install wm@bottle
claude plugin install superego@bottle

# Optional:
claude plugin install datasphere@bottle
claude plugin install oh-mcp@bottle
claude plugin install miranda@bottle
```

### 3. Initialize in Your Project

Run the unified init command:

```bash
# In Claude Code:
/bottle:init
```

Bottle will automatically:
- Detect and install missing binaries (ba, wm, sg) via homebrew or cargo
- Initialize all tools
- Set recommended defaults (e.g., superego pull mode)
- Create/update AGENTS.md with tool guidance

## Core Stack

Start with these for immediate 10-100x leverage:

```bash
claude plugin marketplace add cloud-atlas-ai/bottle
claude plugin install bottle@bottle
claude plugin install ba@bottle
claude plugin install wm@bottle
claude plugin install superego@bottle
```

Then in your project:
```bash
/bottle:init
```

That's it - one command initializes everything with recommended defaults.

---

## Codex Installation

**Install the skill:**

```bash
mkdir -p ~/.codex/skills/bottle
curl -fsSL -o ~/.codex/skills/bottle/SKILL.md \
  https://raw.githubusercontent.com/cloud-atlas-ai/bottle/main/codex-skill/SKILL.md
curl -fsSL -o ~/.codex/skills/bottle/AGENTS.md.snippet \
  https://raw.githubusercontent.com/cloud-atlas-ai/bottle/main/codex-skill/AGENTS.md.snippet
```

**Initialize in Codex:**

```
$bottle init
```

Bottle will automatically:
- Install binaries (ba, wm, sg) via Homebrew or Cargo
- Install child skills ($ba, $wm, $superego)
- Initialize tool directories (.ba/, .wm/, .superego/)
- Set recommended defaults (superego pull mode)
- Create AGENTS.md with usage protocols

**Usage:**

```
$bottle dive fix       # Start a bug fix session
$bottle dive plan      # Start a planning session
$ba status             # Check your tasks
$superego              # Get feedback at decision points
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
claude plugin marketplace update bottle
claude plugin update superego@bottle
claude plugin update wm@bottle
# ... etc
```

## Individual Repos

Each tool has its own repo with detailed documentation:

- [ba](https://github.com/cloud-atlas-ai/ba)
- [wm](https://github.com/cloud-atlas-ai/wm)
- [superego](https://github.com/cloud-atlas-ai/superego)
- [datasphere](https://github.com/cloud-atlas-ai/datasphere)
- [oh-mcp-server](https://github.com/cloud-atlas-ai/oh-mcp-server)
- [miranda](https://github.com/cloud-atlas-ai/miranda)

## License

Source-available. See individual repos for license details.
