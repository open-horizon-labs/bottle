# Bottle

**Cloud Atlas AI Core Stack - Keep your tools in sync**

Unified installation and updates for the Cloud Atlas AI development stack:

- **ba** - Task tracking for AI sessions
- **wm** - Working memory for automatic tacit knowledge extraction
- **superego** - Metacognitive advisor that monitors and provides feedback

## Quick Start

```bash
# Clone and install
git clone https://github.com/cloud-atlas-ai/bottle
cd bottle
./scripts/install.sh

# Later, update everything
./scripts/update.sh
```

## What Gets Installed

### ba - Task Tracking
Simple, file-based task tracking designed for AI sessions. No server, no database, just tasks.

```bash
ba init
ba create "Add feature X" -t feature -p 1
ba claim <task-id> --session $(uuidgen)
```

More: [github.com/cloud-atlas-ai/ba](https://github.com/cloud-atlas-ai/ba)

### wm - Working Memory
Automatically extracts tacit knowledge from your coding sessions and surfaces relevant context.

```bash
wm init
# Works automatically with Claude Code
# Captures patterns, constraints, preferences over time
```

More: [github.com/cloud-atlas-ai/wm](https://github.com/cloud-atlas-ai/wm)

### superego - Metacognition
Monitors Claude's work and provides feedback before finishing or making large changes.

```bash
sg init
# Or in Claude Code: /superego:init
# Evaluates approach, catches scope drift, suggests improvements
```

More: [github.com/cloud-atlas-ai/superego](https://github.com/cloud-atlas-ai/superego)

## How They Work Together

```
You write prompt → wm injects relevant context
                ↓
Claude works on task (tracked in ba)
                ↓
superego evaluates before finishing
                ↓
wm extracts new knowledge for next time
```

## Installation Details

**What install.sh does:**
1. Checks prerequisites (Rust/cargo, Claude CLI)
2. Installs ba, wm, superego via `cargo install`
3. Installs Claude Code plugins (if Claude CLI available)
4. Provides next steps

**Per-project setup:**
```bash
cd /your/project
ba init       # Creates .ba/ for task tracking
wm init       # Creates .wm/ for working memory
sg init       # Creates .superego/ for metacognition
```

## Updating

Keep all tools in sync:

```bash
./scripts/update.sh
```

This updates:
- All binaries (`cargo install --force`)
- All Claude plugins
- Prompts you to restart Claude Code

## Other Tools

Bottle focuses on the core development stack. Other Cloud Atlas AI tools:

- **oh-mcp** - Open Horizons MCP for strategic alignment ([repo](https://github.com/cloud-atlas-ai/oh-mcp-server))
- **miranda** - Telegram bot for remote orchestration ([repo](https://github.com/cloud-atlas-ai/miranda))

Install these separately when needed.

## License

Source-available. See [LICENSE](LICENSE) for details.
