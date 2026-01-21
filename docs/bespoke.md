# Bespoke Bottles

Bespoke bottles let you create custom tool configurations with pinned versions, independent of curated bottle updates.

## When to Use Bespoke Bottles

- **Version pinning**: Lock specific versions you've tested together
- **Custom tool sets**: Include only the tools you need
- **Experimentation**: Test new versions before they hit stable
- **Offline/air-gapped**: Pre-configure manifests for environments without internet

## Creating a Bespoke Bottle

### From Scratch

```bash
bottle create mystack
```

This creates an empty manifest at `~/.bottle/bottles/mystack/manifest.json`:

```json
{
  "name": "mystack",
  "version": "2026.01.15",
  "description": "My custom tool versions",
  "tools": {},
  "plugins": [],
  "prerequisites": {},
  "opencode_plugins": {}
}
```

### From an Existing Bottle

```bash
bottle create mystack --from stable
```

This copies the current stable manifest as your starting point:

```json
{
  "name": "mystack",
  "version": "2026.01.15",
  "description": "Custom bottle based on stable",
  "tools": {
    "ba": "0.2.1",
    "wm": "0.3.1",
    "superego": "0.9.0",
    "oh-mcp": "0.3.3"
  },
  "plugins": ["ba", "superego", "wm", "oh-mcp", "miranda"],
  "prerequisites": {
    "cargo": "Required for ba, superego, wm, datasphere",
    "node": "Required for oh-mcp (uses npx)"
  },
  "opencode_plugins": {
    "@cloud-atlas-ai/bottle": "0.2.6",
    "ba-opencode": "0.2.1",
    "wm-opencode": "0.1.0",
    "superego-opencode": "0.8.5"
  }
}
```

## Editing the Manifest

Edit `~/.bottle/bottles/mystack/manifest.json` to customize:

### Pin a Specific Version

```json
{
  "tools": {
    "wm": "0.2.2",  // Pin to older version
    "ba": "0.2.1",
    "superego": "0.9.0"
  }
}
```

### Remove Tools You Don't Need

```json
{
  "tools": {
    "ba": "0.2.1",
    "superego": "0.9.0"
    // Removed wm and oh-mcp
  },
  "plugins": ["ba", "superego"]  // Update plugins list too
}
```

### Add a Tool Not in Stable

```json
{
  "tools": {
    "ba": "0.2.1",
    "wm": "0.3.1",
    "superego": "0.9.0",
    "datasphere": "0.1.0"  // Added datasphere
  },
  "plugins": ["ba", "superego", "wm", "datasphere"]
}
```

## Advanced Features

Bespoke bottles support additional features beyond curated bottles: MCP servers, AGENTS.md injection, and custom tools.

### MCP Servers

Register arbitrary MCP servers with Claude Code and OpenCode:

```json
{
  "mcp_servers": {
    "figma": {
      "command": "npx",
      "args": ["-y", "@anthropic/mcp-figma"],
      "env": {
        "FIGMA_TOKEN": "${FIGMA_TOKEN}"
      },
      "scope": "user"
    },
    "ado": {
      "command": "npx",
      "args": ["-y", "@anthropic/mcp-azure-devops"],
      "env": {
        "ADO_PAT": "${ADO_PAT}",
        "ADO_ORG": "myorg"
      },
      "scope": "project"
    }
  }
}
```

**Fields:**
- `command` (required): The command to run (e.g., `npx`, `node`, `python`)
- `args`: Arguments to pass to the command
- `env`: Environment variables. Use `${VAR}` syntax for required env vars
- `scope`: Either `user` (global) or `project` (per-project). Default: `user`

**Environment Variables:**
- Env vars using `${VAR}` syntax are validated at install time
- If a required env var is not set, installation fails with a clear error message
- Set the env vars before running `bottle install`

### AGENTS.md Snippets

Define custom guidance for AI agents to add to project AGENTS.md files:

```json
{
  "agents_md": {
    "sections": [
      {
        "heading": "## Design Review Protocol",
        "content": "Before implementing any UI changes, create a Figma mockup and get design review approval."
      },
      {
        "heading": "## ADO Integration",
        "content": "Link all PRs to work items. Use 'AB#1234' syntax in commit messages."
      }
    ],
    "snippets_url": "https://example.com/team-agents-snippets.md"
  }
}
```

**Fields:**
- `sections`: Array of inline sections to include
  - `heading`: Markdown heading (e.g., `## Section Title`)
  - `content`: Markdown content for the section
- `snippets_url`: Optional HTTPS URL to fetch additional snippet content from

**How it works:**
- During install/update, bottle saves the combined snippet to `~/.bottle/bottles/<name>/agents-md-snippet`
- Run `bottle agents-md` to output the snippet for the active bottle
- The AI agent is responsible for reading the snippet and applying it to AGENTS.md
- This agent-driven approach allows intelligent placement and conflict resolution

### Custom Tools

Install tools not managed by curated bottles:

```json
{
  "custom_tools": {
    "internal-cli": {
      "install": {
        "brew": "internal-tap/cli",
        "cargo": "internal-cli",
        "npm": "@company/internal-cli",
        "binary_url": "https://releases.example.com/cli/{platform}/cli"
      },
      "version": "1.2.3",
      "verify": "internal-cli --version"
    }
  }
}
```

**Install Methods (tried in order):**
1. `brew`: Homebrew formula (e.g., `tap/formula`)
2. `cargo`: Cargo crate name
3. `npm`: npm package name (installed globally)
4. `binary_url`: Direct binary download URL (must use HTTPS)

**Brew version note:** Versioned formulas (`formula@version`) only work for some core Homebrew packages like `python@3.9`. Tap formulas (e.g., `org/tap/formula`) don't support the `@version` syntax - use `latest` or omit the version for tap formulas.

**URL Placeholders:**
- `{arch}`: CPU architecture (`x86_64`, `aarch64`)
- `{arm64}`: Apple Silicon alias (`arm64` for aarch64, otherwise same as `{arch}`)
- `{os}`: Operating system (`darwin`, `linux`, `windows`)
- `{platform}`: Combined `{os}-{arch}` (e.g., `darwin-aarch64`)

**Fields:**
- `install` (required): At least one installation method
- `version` (required): Version to install (or `latest`)
- `verify`: Optional command to verify installation (simple commands only, no quoted args)

### Full Example

Here's a complete bespoke bottle with all features:

```json
{
  "name": "enterprise",
  "version": "2026.01.15",
  "description": "Enterprise team configuration",
  "tools": {
    "ba": "0.2.1",
    "wm": "0.3.2",
    "superego": "0.9.1"
  },
  "plugins": ["ba", "superego", "wm"],
  "prerequisites": {
    "cargo": "Required for ba, superego, wm",
    "node": "Required for MCP servers"
  },
  "mcp_servers": {
    "figma": {
      "command": "npx",
      "args": ["-y", "@anthropic/mcp-figma"],
      "env": {
        "FIGMA_TOKEN": "${FIGMA_TOKEN}"
      }
    }
  },
  "agents_md": {
    "sections": [
      {
        "heading": "## Team Standards",
        "content": "Follow our coding standards at docs.example.com/standards"
      }
    ]
  },
  "custom_tools": {
    "team-cli": {
      "install": {
        "brew": "ourteam/tap/team-cli"
      },
      "version": "2.0.0",
      "verify": "team-cli --version"
    }
  }
}
```

## Installing Your Bespoke Bottle

```bash
bottle install mystack
```

This will:
1. Install/upgrade tools to your pinned versions
2. Record the installation in `~/.bottle/state.json`

## Switching Between Bottles

```bash
# Switch to your bespoke bottle
bottle switch mystack

# Switch back to curated stable
bottle switch stable
```

## Updating Bespoke Bottles

**Bespoke bottles don't auto-update.** When you run `bottle update`, it checks for updates to your current bottle:

- **Curated bottles** (stable, edge): Fetches latest from GitHub
- **Bespoke bottles**: Only updates if you manually edit the manifest

To update a bespoke bottle:

1. Edit `~/.bottle/bottles/mystack/manifest.json`
2. Change the versions you want to update
3. Run `bottle update` or `bottle install mystack`

### Checking Latest Versions

To see what's available:

```bash
# Compare your bottle to latest releases
bottle diff mystack latest

# Or check crates.io directly
cargo search ba
cargo search wm
cargo search superego
```

## Listing Your Bottles

```bash
bottle list
```

Shows both curated and bespoke bottles:

```
Curated bottles:
  stable    Production-ready Open Horizon Labs stack
  edge      Latest versions (may be unstable)

Bespoke bottles:
  mystack   Custom bottle based on stable
```

## File Locations

| Item | Location |
|------|----------|
| Bespoke bottles | `~/.bottle/bottles/<name>/manifest.json` |
| Bottle state | `~/.bottle/bottles/<name>/state.json` |
| AGENTS.md snippet | `~/.bottle/bottles/<name>/agents-md-snippet` |
| Active bottle pointer | `~/.bottle/active` |
| Curated manifests | Fetched from GitHub |

## Example: Testing a New wm Version

```bash
# 1. Create bespoke bottle from stable
bottle create test-wm --from stable

# 2. Edit to use newer wm
#    ~/.bottle/bottles/test-wm/manifest.json
#    Change: "wm": "0.3.1" -> "wm": "0.4.0"

# 3. Install it
bottle install test-wm

# 4. Test in your projects...

# 5. If it works, switch back to stable and wait for update
#    Or keep using your bespoke bottle
bottle switch stable
```

## Troubleshooting

### "Bottle not found"

Make sure the manifest exists:
```bash
ls ~/.bottle/bottles/mystack/manifest.json
```

### Version install fails

Check if the version exists:
```bash
cargo search ba --limit 10
```

### Want to start over

Delete the bespoke bottle:
```bash
rm -rf ~/.bottle/bottles/mystack
```

## Limitations

- **No automatic updates**: You maintain the manifest
- **No compatibility guarantees**: If manifest format changes, you update it
- **Local only**: Bespoke bottles aren't synced or shared

For shared team configurations, see the Team Setup Pattern below.

---

## Team Setup Pattern

For teams that want to share a bottle configuration via version control, use a **project-local bespoke bottle**. This pattern enables AI-assisted developer onboarding.

### The Problem

Setup is always painful. AI assistants can help—but only if they have context about team-specific tooling. Without that context, setup remains a docs-reading exercise.

### The Solution

A three-layer structure that separates concerns:

```
dev_tools/bottle/
├── manifest.json   ← machine layer: tools, versions, MCP servers
├── SETUP.md        ← agent layer: verification, troubleshooting, guidance
├── README.md       ← human layer: quick start, what's installed
└── bootstrap.sh    ← one-time orchestration (VPN, auth, bottle install)
```

**manifest.json** — structured data for `bottle install` and `bottle integrate`

**SETUP.md** — prose guidance for AI assistants. Includes:
- Environment verification commands
- How to fix common issues
- Tool usage cheatsheets

**README.md** — human-readable quick start

**bootstrap.sh** — optional script for first-time setup (auth flows, VPN checks)

### How It Works

1. New developer clones the repo
2. Runs `bash dev_tools/bottle/bootstrap.sh` (or follows README)
3. Opens AI coding assistant in the project
4. AI reads SETUP.md, can verify setup and troubleshoot issues
5. Setup becomes a conversation, not a docs hunt

### Design Principles

- **Bottle handles packages**: install, update, integrate
- **AI handles guidance**: run verification, explain errors, respect preferences
- **User stays in control**: taste and nuance belong to humans

### Example

See `bottles/example-team/` for a complete working example with:
- manifest.json with tools, MCP servers, env vars
- SETUP.md with verification and troubleshooting
- README.md with quick start
- bootstrap.sh for one-time setup

### Using a Project-Local Bottle

```bash
# From project root
bottle install --manifest dev_tools/bottle/manifest.json
bottle integrate opencode --manifest dev_tools/bottle/manifest.json
```

The `--manifest` flag tells bottle to use the project-local manifest instead of a user-installed bottle.
