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
