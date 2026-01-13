# Bottle Specification

**Version:** 0.1.0-draft
**Date:** 2026-01-11
**Status:** Design

## Overview

Bottle is a curated snapshot manager for the Open Horizon Labs tool stack. It provides one-command installation, coherent versioning, and seamless updates for users who want a batteries-included experience.

**Core principle:** The value is in the tooling. Every interaction must be effortless, predictable, and transparent.

## Philosophy

### Tools Are Independent

Each tool (ba, superego, wm, datasphere, oh-mcp, miranda) has its own:
- Repository
- Release cycle
- Version scheme
- Package registry (crates.io, npm)

Bottle does not control tool releases. Tools release when they're ready.

### Bottles Are Curated Snapshots

A bottle is a tested combination of tool versions + matching plugins. Bottles are:
- **Named:** stable, edge
- **Versioned:** dated snapshots (2026.01.15)
- **Complete:** binaries + MCP servers + plugins, all matched
- **Cohesive:** no backwards compatibility across bottles

### One Bottle Active

Users install one bottle at a time. Switching bottles replaces everything. No mixing.

### Tooling Is Everything

The bottle commands must be:
- **Impeccable:** Zero friction, zero confusion
- **Transparent:** Always show what will happen before doing it
- **Recoverable:** Clear path forward if something fails
- **Fast:** No unnecessary waiting

If the tooling isn't excellent, bottle has no value. Users can just `cargo install` things themselves.

---

## Architecture

### Repository Structure

```
bottle/
├── tools/                      # Tool definitions (how to install)
│   ├── ba.json
│   ├── superego.json
│   ├── wm.json
│   ├── datasphere.json
│   └── oh-mcp.json
│
├── bottles/                    # Curated snapshots
│   ├── stable/
│   │   ├── manifest.json       # Pinned versions
│   │   ├── CHANGELOG.md        # What changed in this bottle
│   │   └── plugins/            # Complete plugin set
│   │       ├── ba/
│   │       ├── superego/
│   │       ├── wm/
│   │       ├── datasphere/
│   │       ├── oh-mcp/
│   │       └── miranda/
│   │
│   ├── edge/
│   │   ├── manifest.json
│   │   ├── CHANGELOG.md
│   │   └── plugins/
│   │
│
├── commands/                   # Bottle management commands
│   ├── install.md
│   ├── status.md
│   ├── update.md
│   ├── switch.md
│   ├── eject.md
│   ├── diff.md
│   ├── upgrade.md
│   ├── validate.md
│   └── release.md
│
└── .claude-plugin/
    └── plugin.json             # Bottle marketplace entry
```

### Tool Definition Format

```json
{
  "name": "superego",
  "binary": "sg",
  "registry": "crates.io",
  "package": "superego",
  "type": "binary",
  "install": {
    "cargo": "cargo install superego@{version}",
    "brew": "brew install oh-labs/tap/superego"
  },
  "check": "sg --version",
  "homepage": "https://github.com/cloud-atlas-ai/superego"
}
```

```json
{
  "name": "oh-mcp",
  "type": "mcp",
  "registry": "npm",
  "package": "@cloud-atlas-ai/oh-mcp-server",
  "install": {
    "mcp": "claude mcp add oh-mcp -s user -- npx -y @cloud-atlas-ai/oh-mcp-server@{version}"
  },
  "check": "claude mcp list | grep oh-mcp",
  "homepage": "https://github.com/cloud-atlas-ai/oh-mcp-server"
}
```

### Bottle Manifest Format

```json
{
  "name": "stable",
  "version": "2026.01.15",
  "description": "Production-ready Open Horizon Labs stack",
  "tools": {
    "ba": "0.2.1",
    "superego": "0.9.0",
    "wm": "0.2.2",
    "datasphere": "0.1.0",
    "oh-mcp": "0.3.3"
  },
  "plugins": [
    "ba",
    "superego",
    "wm",
    "datasphere",
    "oh-mcp",
    "miranda"
  ],
  "prerequisites": {
    "cargo": "Required for ba, superego, wm, datasphere",
    "node": "Required for oh-mcp (uses npx)"
  }
}
```

### User State Format

Location: `~/.bottle/state.json`

```json
{
  "bottle": "stable",
  "bottle_version": "2026.01.15",
  "installed_at": "2026-01-15T10:30:00Z",
  "tools": {
    "ba": {
      "version": "0.2.1",
      "installed_at": "2026-01-15T10:30:05Z",
      "method": "cargo"
    },
    "superego": {
      "version": "0.9.0",
      "installed_at": "2026-01-15T10:30:15Z",
      "method": "cargo"
    }
  },
  "integrations": {
    "claude_code": {
      "installed_at": "2026-01-15T10:31:00Z"
    },
    "opencode": {
      "installed_at": "2026-01-15T10:31:05Z"
    }
  },
  "mode": "managed"
}
```

Mode values:
- `managed` - Bottle controls everything
- `ejected` - User took manual control

Integration keys: `claude_code`, `opencode`, `codex`

---

## User Commands

### /bottle:install

Install a bottle. Installs CLI tools only.

**Input:** Bottle name (optional, defaults to "stable")

**Flags:**
- `-y` - Skip confirmations
- `--dry-run` - Show what would be installed without making changes

**Flow:**
1. Check prerequisites (cargo, node, etc.)
2. Show exactly what will be installed (tools + versions)
3. Confirm
4. Install binaries at pinned versions
5. Register MCP servers
6. Write state
7. Show success + next steps (run `bottle integrate`)

**Example:**
```
bottle install stable
```
```
Installing bottle: stable (v0.1.0)

Tools:
  ba           0.1.0  install
  wm           0.2.0  install
  sg           0.3.0  install
  oh-mcp       0.3.0  install (MCP)

Proceed? [Y/n]
> Y

Installing tools...
  ba           installed
  wm           installed
  sg           installed
  oh-mcp       registered

Done! Next: run 'bottle integrate claude-code' (or codex, opencode)
```

**Note:** Platform integrations (plugins, skills) are installed separately via `bottle integrate`.

### /bottle:status

Show current state clearly.

**Output example:**
```
Bottle: stable (2026.01.15)

Tools:
  ba         0.2.1   ✓ installed
  superego   0.9.0   ✓ installed
  wm         0.2.2   ✓ installed
  datasphere 0.1.0   ✓ installed
  oh-mcp     0.3.3   ✓ registered

Integrations:
  Claude Code   ✓ installed
  OpenCode      ✓ installed
  Codex         ✗ not installed

Update available: stable (2026.01.20)
  Changes:
    superego  0.9.0 → 0.10.0
    wm        0.2.2 → 0.3.0

Run 'bottle update' to upgrade
Run 'bottle integrate codex' to add Codex integration
```

**UX requirement:** Glanceable. User knows exactly what they have in 2 seconds.

### /bottle:update

Update to latest bottle snapshot.

**Flow:**
1. Fetch latest manifest from GitHub
2. Diff against current state
3. Show what will change (before doing anything)
4. Confirm
5. Update binaries
6. Update MCP servers
7. Update plugins
8. Update state
9. Show what changed

**UX requirement:** Always preview before action. Never surprise.

### /bottle:switch

Change to a different bottle.

**Flow:**
1. Show current bottle
2. Show target bottle differences
3. Warn about what will change
4. Confirm
5. Uninstall current bottle's plugins
6. Install new bottle's plugins
7. Update binaries to new versions (up or down)
8. Update state

**UX requirement:** Make the consequences crystal clear before switching.

### /bottle:eject

Leave bottle management, keep tools.

**Flow:**
1. Explain what ejecting means
2. Show what user is keeping
3. Confirm
4. Set mode to "ejected"
5. Leave binaries and MCP servers in place
6. Optionally uninstall bottle management plugin

**Post-eject:** User manages tools manually. /bottle:status shows "ejected" mode.

**UX requirement:** Graceful exit. No lock-in.

### /bottle:integrate

Add or remove platform integrations.

**Usage:**
```bash
bottle integrate claude_code      # Add Claude Code integration
bottle integrate opencode         # Add OpenCode integration
bottle integrate codex            # Add Codex integration
bottle integrate --remove codex   # Remove an integration
bottle integrate --list           # Show available/installed integrations
```

**Platforms:**
| Platform | Detection | Integration Action |
|----------|-----------|-------------------|
| Claude Code | `~/.claude/` exists | `claude plugin install bottle@cloud-atlas-ai/bottle` |
| OpenCode | `opencode.json` exists | Add to plugins array in `opencode.json` |
| Codex | `~/.codex/` exists | Install skills to `~/.codex/skills/` |

**Flow (add):**
1. Check if platform is detected
2. If not detected, warn but allow (user may know better)
3. Check if already installed
4. Install integration
5. Update state
6. Confirm success

**Flow (remove):**
1. Check if integration is installed
2. Remove integration
3. Update state
4. Confirm removal

**Example:**
```
$ bottle integrate opencode

Detected: opencode.json in current directory

Installing OpenCode integration...
  ✓ Added @cloud-atlas-ai/bottle to opencode.json

Note: Restart OpenCode to load the new plugin.
```

**UX requirement:** Safe to run multiple times (idempotent).

---

## Bespoke Bottles

Users can create custom bottles for project-specific version requirements.

### Philosophy: You Build It, You Own It

**Curated bottles** (stable, edge) are maintained by the bottle project. They receive updates, testing, and long-term support.

**Bespoke bottles** are user-created and user-maintained. The bottle project makes no backwards compatibility guarantees for bespoke bottles. If the manifest format evolves, users are responsible for updating their bespoke bottles.

This is intentional:
- Keeps bottle simple (no schema versioning, no migration tooling)
- Clear ownership model
- Users can always recreate from a curated bottle as base

### /bottle:create

Create a new bespoke bottle.

**Usage:**
```bash
bottle create mybottle                    # Empty bottle
bottle create mybottle --from stable      # Copy from curated bottle
```

**Flow:**
1. Create `~/.bottle/bottles/mybottle/`
2. If `--from`, copy manifest from source bottle
3. Open manifest in editor (if `$EDITOR` set)
4. Show next steps

**Output:**
```
Created bespoke bottle: mybottle

Location: ~/.bottle/bottles/mybottle/manifest.json

Edit the manifest to pin your desired versions, then:
  bottle install mybottle

Note: Bespoke bottles are user-maintained. You're responsible
for keeping the manifest compatible with future bottle versions.
```

### Bespoke Bottle Location

```
~/.bottle/
├── state.json              # Current installation state
└── bottles/                # User-created bottles
    ├── mybottle/
    │   └── manifest.json
    └── client-project/
        └── manifest.json
```

Curated bottles are fetched from GitHub. Bespoke bottles live locally.

### Bespoke Manifest Format

Same format as curated bottles:

```json
{
  "name": "mybottle",
  "version": "2026.01.12",
  "description": "My custom tool versions",
  "tools": {
    "ba": "0.2.1",
    "superego": "0.9.0",
    "wm": "0.2.2"
  },
  "plugins": [
    "ba",
    "superego",
    "wm"
  ]
}
```

Users can:
- Pin specific versions
- Include only tools they need
- Skip plugins they don't want

### Installing a Bespoke Bottle

```bash
bottle install mybottle
```

Works identically to installing a curated bottle. The only difference is where the manifest is read from (local vs GitHub).

### Listing Available Bottles

```bash
bottle list
```

**Output:**
```
Curated bottles (from GitHub):
  stable     Production-ready stack
  edge       Latest features

Bespoke bottles (local):
  mybottle        My custom tool versions
  client-project  Pinned for client work
```

### Sharing Bespoke Bottles

Bespoke bottles are just JSON files. To share:
1. Copy `~/.bottle/bottles/mybottle/manifest.json` to your project repo
2. Teammate copies it to their `~/.bottle/bottles/`
3. They run `bottle install mybottle`

No special tooling needed. It's just a file.

---

## Curator Commands

For maintainers managing bottle releases.

### /bottle:diff

Compare bottles or check for updates.

**Usage:**
```
/bottle:diff stable edge           # Compare two bottles
/bottle:diff stable latest         # Compare stable vs latest tool versions
```

**Output:**
```
Comparing: stable (2026.01.15) vs edge (2026.01.18)

  superego   0.9.0  →  0.10.0
    + Added pull mode auto-detection
    + Fixed hook timeout on large files
    ! Breaking: removed --legacy flag

  wm         0.2.2  →  0.3.0
    + Added datasphere integration
    + Added session compression

  ba         0.2.1  =  0.2.1  (unchanged)
```

**UX requirement:** Rich context, not just version numbers. Help curator decide.

### /bottle:upgrade

Bump a tool version in a bottle manifest.

**Usage:**
```
/bottle:upgrade stable superego 0.10.0
```

**Flow:**
1. Update manifest.json
2. Run /bottle:validate
3. Show what changed
4. Remind curator to update plugins if needed

**UX requirement:** Validate automatically. Catch mistakes early.

### /bottle:validate

Check bottle for issues.

**Checks:**
- All tools in manifest have valid tool definitions
- All plugins referenced exist
- Version formats are valid
- Prerequisites are documented
- No conflicts between tools

**UX requirement:** Run fast. Fail loud with clear messages.

### /bottle:release

Tag and publish a bottle update.

**Flow:**
1. Run /bottle:validate
2. Check git status (clean?)
3. Show what's being released
4. Create git tag (e.g., `stable-2026.01.20`)
5. Push tag
6. Confirm marketplace will pick up on next sync

**UX requirement:** Make releasing feel safe and reversible.

---

## Registry & Distribution

### Manifest Fetching

Bottles are fetched from GitHub raw URLs:

```
https://raw.githubusercontent.com/cloud-atlas-ai/bottle/main/bottles/stable/manifest.json
```

No git clone required. Simple HTTP fetch.

### Platform Integrations

Bottle separates **tools** (universal binaries) from **integrations** (platform-specific wrappers).

```
┌─────────────────────────────────────────────────────────────┐
│                    BOTTLE = TOOLS                           │
│                                                             │
│   stable bottle = ba 0.1.0, wm 0.2.0, sg 0.3.0             │
│   (universal - same binaries for all platforms)             │
└─────────────────────────────────────────────────────────────┘
                           │
           ┌───────────────┼───────────────┐
           ▼               ▼               ▼
┌─────────────────┐ ┌─────────────────┐ ┌─────────────────┐
│  Claude Code    │ │    OpenCode     │ │     Codex       │
│                 │ │                 │ │                 │
│ .claude-plugin/ │ │ opencode-plugin/│ │ codex-skill/    │
│ → marketplace   │ │ → npm           │ │ → skill install │
│                 │ │                 │ │                 │
│ Thin wrapper    │ │ Thin wrapper    │ │ Thin wrapper +  │
│ invokes CLI     │ │ invokes CLI     │ │ Codex-native    │
└─────────────────┘ └─────────────────┘ └─────────────────┘
```

**Key principles:**
- Tools are universal (ba, wm, sg work identically regardless of AI platform)
- Integrations are opt-in (user chooses which platforms to integrate with)
- Each integration is a thin wrapper that invokes the `bottle` CLI
- Plugins are NOT namespaced per bottle (one set active at a time)
- Integrations persist across bottle switches (they just invoke CLI, don't depend on tool versions)

**Platform detection:**
- Claude Code: `~/.claude/` directory exists
- OpenCode: `opencode.json` in current directory or home
- Codex: `~/.codex/` directory exists

**Integration installation:**
- Claude Code: `claude plugin install bottle@cloud-atlas-ai/bottle`
- OpenCode: Add `@cloud-atlas-ai/bottle` to `opencode.json` plugins array
- Codex: Skills installed via `$bottle init`

### State Persistence

User state in `~/.bottle/state.json`. Created on first install. Updated on every operation.

---

## Design Principles

### 1. Show, Then Do

Every command that changes state must show what will happen and ask for confirmation. No silent operations.

### 2. Fail Fast, Fail Clear

If something goes wrong:
- Stop immediately
- Show exactly what failed
- Show how to recover or retry
- Leave system in a known state

### 3. One Happy Path

The common case (install stable, update periodically) should be trivially easy. Complexity is for edge cases.

### 4. No Magic

Users can always see:
- What bottle they have
- What versions are installed
- Where things came from
- What will change

### 5. Graceful Degradation

If a tool fails to install:
- Continue with others
- Report what failed
- Offer to retry just the failed tool
- Don't leave partial state

---

## Success Criteria

Bottle is successful when:

1. **First install < 2 minutes** - From zero to working stack
2. **Status < 1 second** - Instant visibility
3. **Update is boring** - Nothing surprising ever happens
4. **Switching is safe** - Users trust they won't break anything
5. **Ejecting is clean** - No leftover mess

---

## Open Questions

### Q1: Binary version enforcement

When user runs `cargo install superego` directly (bypassing bottle), should /bottle:status warn about drift? Or just show current state?

### Q2: Partial failures

If 4/5 tools install but one fails, what's the state? Managed with caveats? Partial?

### Q3: Offline support

Should bottle cache manifests locally for offline status checks?

### Q4: Multiple machines

How does a user replicate their bottle setup on a new machine? Export/import?

---

## Implementation

### Rust Binary

Bottle is implemented as a Rust CLI binary, consistent with ba, superego, wm, and datasphere.

**Why Rust:**
- Same stack as other Cloud Atlas tools
- Fast, predictable execution
- Proper error handling with Result types
- Rich TUI support (progress bars, spinners, colors)
- Cross-platform (macOS, Linux)
- Single binary distribution

**Binary name:** `bottle`

**Installation:**
```bash
cargo install bottle
# or
brew install oh-labs/tap/bottle
```

### Command Mapping

| CLI Command | Claude Code | OpenCode | Codex |
|-------------|-------------|----------|-------|
| `bottle install` | `/bottle:install` | `bottle-install` | `$bottle install` |
| `bottle status` | `/bottle:status` | `bottle-status` | `$bottle status` |
| `bottle update` | `/bottle:update` | `bottle-update` | `$bottle update` |
| `bottle switch` | `/bottle:switch` | `bottle-switch` | `$bottle switch` |
| `bottle eject` | `/bottle:eject` | `bottle-eject` | `$bottle eject` |
| `bottle integrate` | `/bottle:integrate` | `bottle-integrate` | `$bottle integrate` |
| `bottle create` | `/bottle:create` | `bottle-create` | `$bottle create` |
| `bottle list` | `/bottle:list` | `bottle-list` | `$bottle list` |

### Platform Plugin Architecture

Each platform's plugin is a **thin wrapper** that:
1. Checks if `bottle` binary exists
2. If missing, guides installation
3. Invokes the appropriate `bottle` CLI command
4. Returns output to the AI agent

**Claude Code plugin** (`.claude-plugin/commands/*.md`):
```markdown
# /bottle:install

Run `bottle install` in the terminal and follow the prompts.

If bottle is not installed, offer to install it:
- cargo install bottle
- brew install oh-labs/tap/bottle
```

**OpenCode plugin** (`opencode-plugin/src/index.ts`):
```typescript
"bottle-install": tool({
  description: "Install a bottle (stable, edge, or bespoke)",
  args: { bottle: tool.schema.string() },
  async execute({ bottle }) {
    // Check binary exists, guide install if not
    // Run: bottle install <bottle>
    // Return output
  }
})
```

**Codex skill** (`codex-skill/SKILL.md`):
- Standard commands invoke `bottle` CLI
- Codex-native features that use Codex capabilities:
  - `$bottle web-context` - Web search integration
  - `$bottle resume` - Codex session continuity
  - `$bottle codex-sync` - Session knowledge extraction

The plugins exist for:
1. **Discoverability** - Users find commands via platform-native means
2. **Bootstrap** - Help install bottle binary if missing
3. **Platform features** - Codex-specific features that use Codex capabilities

All standard bottle operations happen in the Rust binary.

### Crate Structure

```
bottle/
├── Cargo.toml
├── src/
│   ├── main.rs              # CLI entry point
│   ├── commands/
│   │   ├── mod.rs
│   │   ├── install.rs       # bottle install
│   │   ├── status.rs        # bottle status
│   │   ├── update.rs        # bottle update
│   │   ├── switch.rs        # bottle switch
│   │   ├── eject.rs         # bottle eject
│   │   ├── integrate.rs     # bottle integrate
│   │   ├── create.rs        # bottle create (bespoke)
│   │   ├── list.rs          # bottle list
│   │   ├── diff.rs          # bottle diff (curator)
│   │   ├── upgrade.rs       # bottle upgrade (curator)
│   │   ├── validate.rs      # bottle validate (curator)
│   │   └── release.rs       # bottle release (curator)
│   ├── manifest/
│   │   ├── mod.rs
│   │   ├── tool.rs          # Tool definition parsing
│   │   ├── bottle.rs        # Bottle manifest parsing
│   │   └── state.rs         # User state management
│   ├── install/
│   │   ├── mod.rs
│   │   ├── cargo.rs         # cargo install wrapper
│   │   ├── brew.rs          # brew install wrapper
│   │   ├── mcp.rs           # claude mcp add wrapper
│   │   └── plugin.rs        # claude plugin install wrapper
│   ├── integrate/
│   │   ├── mod.rs           # Platform integration logic
│   │   ├── claude_code.rs   # Claude Code plugin management
│   │   ├── opencode.rs      # OpenCode plugin management
│   │   └── codex.rs         # Codex skill management
│   ├── fetch.rs             # GitHub raw manifest fetching
│   └── ui.rs                # Progress bars, spinners, colors
```

### Key Dependencies

```toml
[dependencies]
clap = { version = "4", features = ["derive"] }  # CLI parsing
serde = { version = "1", features = ["derive"] } # JSON parsing
serde_json = "1"
reqwest = { version = "0.11", features = ["blocking"] }  # HTTP fetch
indicatif = "0.17"           # Progress bars
console = "0.15"             # Colors, terminal control
dialoguer = "0.11"           # Interactive prompts
dirs = "5"                   # ~/.bottle path
chrono = "0.4"               # Timestamps
thiserror = "1"              # Error handling
```

### Error Handling

All commands return `Result<(), BottleError>` where:

```rust
#[derive(thiserror::Error, Debug)]
pub enum BottleError {
    #[error("Failed to fetch manifest: {0}")]
    FetchError(#[from] reqwest::Error),

    #[error("Tool installation failed: {tool} - {reason}")]
    InstallError { tool: String, reason: String },

    #[error("No bottle installed. Run `bottle install` first.")]
    NoBottleInstalled,

    #[error("State file corrupted: {0}")]
    StateCorrupted(String),

    // ... etc
}
```

Every error includes:
1. What failed
2. Why it failed
3. How to fix it

### UX Primitives

**Progress:**
```
Installing stable bottle...
  [1/5] ba 0.2.1         ████████████████████ 100%
  [2/5] superego 0.9.0   ████████░░░░░░░░░░░░  40%
```

**Confirmation:**
```
This will install:
  • ba 0.2.1
  • superego 0.9.0
  • wm 0.2.2
  • datasphere 0.1.0
  • oh-mcp 0.3.3

Proceed? [Y/n]
```

**Status:**
```
Bottle: stable (2026.01.15)

  ba         0.2.1   ✓
  superego   0.9.0   ✓
  wm         0.2.2   ✓
  datasphere 0.1.0   ✓
  oh-mcp     0.3.3   ✓

All tools up to date.
```

---

## Next Steps

1. Finalize open questions
2. Scaffold Rust crate with clap CLI
3. Implement `bottle status` (simplest command)
4. Implement `bottle install stable`
5. Restructure repo: add tools/, bottles/stable/
6. Create stable bottle with current tool versions
7. Iterate on UX based on real usage
