# Bottle

**Cloud Atlas AI - One entry point for all the tools**

Bottle provides the complete Cloud Atlas AI stack for both **Claude Code** and **OpenCode**:

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

**Install binaries** (ba, wm, and superego require their CLI binaries):

```bash
# via Homebrew
brew install cloud-atlas-ai/ba/ba
brew install cloud-atlas-ai/wm/wm
brew install cloud-atlas-ai/superego/superego

# or via Cargo
cargo install ba
cargo install working-memory  # (published as 'working-memory')
cargo install superego
```

**Initialize tools** - Ask the AI:
- "use ba-init to set up task tracking"
- "initialize wm for working memory"
- "set up superego for metacognition"

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

This will:
- Initialize all installed tools (ba, wm, superego)
- Set recommended defaults (e.g., superego pull mode)
- Create/update AGENTS.md with tool guidance
- Offer to install binaries if missing

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
