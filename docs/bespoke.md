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

### AGENTS.md Injection

Inject custom guidance into project AGENTS.md files:

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
- `sections`: Array of inline sections to inject
  - `heading`: Markdown heading (e.g., `## Section Title`)
  - `content`: Markdown content for the section
- `snippets_url`: Optional HTTPS URL to fetch additional snippet content from

**How it works:**
- Sections are wrapped in `<!-- bottle:agents-md:start -->` and `<!-- bottle:agents-md:end -->` markers
- Re-running `bottle install` updates the managed section without affecting other content
- If AGENTS.md doesn't exist, it's created

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
| Current state | `~/.bottle/state.json` |
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

For shared team configurations, consider contributing a new curated bottle or using version control for your manifest.
