# Bottle - Cloud Atlas AI for OpenCode

**Thin wrapper plugin:** Invokes the `bottle` CLI and integrates the full Cloud Atlas AI stack (ba, wm, superego) with OpenCode.

## Installation

The recommended way to set up the bottle ecosystem:

```bash
# 1. Install the bottle CLI
brew install oh-labs/tap/bottle
# or: cargo install bottle

# 2. Use bottle to integrate with OpenCode
bottle integrate opencode
```

This adds all 4 plugins to your `opencode.json`:
- `@cloud-atlas-ai/bottle` - CLI wrapper and orchestration
- `@cloud-atlas-ai/ba-opencode` - Task tracking
- `@cloud-atlas-ai/wm-opencode` - Working memory
- `@cloud-atlas-ai/superego-opencode` - Metacognition

## What You Get

### bottle - CLI Wrapper Tools

All tools invoke the `bottle` CLI:

- `bottle-install [name]` - Install a bottle (default: stable)
- `bottle-status` - Show installed tools and versions
- `bottle-update` - Update to latest bottle snapshot
- `bottle-switch <name>` - Switch to a different bottle
- `bottle-list` - List available bottles
- `bottle-create <name>` - Create a bespoke bottle
- `bottle-integrate` - Add/remove platform integrations
- `bottle-eject` - Stop bottle management (keep tools)
- `bottle-init` - Initialize all tools with defaults
- `bottle-help` - Show available commands

### Plus Child Plugin Tools

The child plugins provide their own tools:

**ba-opencode:**
- `ba-status`, `ba-list`, `ba-create`, `ba-claim`, `ba-finish`, `ba-block`

**wm-opencode:**
- `wm init`, `wm show`, `wm compile`, `wm distill`

**superego-opencode:**
- `sg-review`, `sg-mode`

## Quick Start

1. **Install bottle CLI:**
   ```bash
   brew install oh-labs/tap/bottle
   # or: cargo install bottle
   ```

2. **Install the bottle tool stack:**
   ```bash
   bottle install
   ```

3. **Integrate with OpenCode:**
   ```bash
   bottle integrate opencode
   ```

4. **Initialize in your project:**
   ```bash
   bottle-init
   ```
   This runs `ba init`, `wm init`, and `sg init` with recommended defaults.

## Architecture

This plugin is a **thin wrapper** that invokes the `bottle` CLI. All logic lives in the CLI - this plugin just passes through commands.

When you run `bottle integrate opencode`, it adds all 4 ecosystem plugins to your opencode.json:
- `@cloud-atlas-ai/bottle` - This plugin (CLI wrapper)
- `@cloud-atlas-ai/ba-opencode` - Task tracking tools
- `@cloud-atlas-ai/wm-opencode` - Working memory tools
- `@cloud-atlas-ai/superego-opencode` - Metacognition tools

Each child plugin can also be installed separately if you only need one tool.

## Individual Plugins

If you only want one tool, install from their respective repos:
- [ba-opencode](https://github.com/cloud-atlas-ai/ba/tree/main/opencode-plugin)
- [wm-opencode](https://github.com/cloud-atlas-ai/wm/tree/main/opencode-plugin)
- [superego-opencode](https://github.com/cloud-atlas-ai/superego/tree/main/opencode-plugin)

## License

MIT

## Links

- [bottle](https://github.com/open-horizon-labs/bottle)
- [ba](https://github.com/cloud-atlas-ai/ba)
- [wm](https://github.com/cloud-atlas-ai/wm)
- [superego](https://github.com/cloud-atlas-ai/superego)
