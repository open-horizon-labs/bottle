# Example Team Bottle

## Quick Start

```bash
bash dev_tools/bottle/bootstrap.sh
```

This runs through:
1. **Bottle install** - Installs all tools from `manifest.json`
2. **OpenCode integrate** - Configures plugins/MCP in project root

## What Gets Installed

| Tool | Purpose |
|------|---------|
| **opencode** | AI coding assistant |
| **ba** | Task tracking - claim/finish work items |
| **wm** | Working memory - context from past sessions |
| **superego** | Metacognitive review - sanity check before commits |
| **azure-cli** | Azure resource management |
| **Figma MCP** | Design context - share Figma URLs |

## After Setup

```bash
# Restart terminal (or source ~/.zshrc)
source ~/.zshrc

# Start OpenCode
opencode
```

## Verify Setup

```bash
bottle verify --manifest dev_tools/bottle/manifest.json
```

---

## Cheatsheet

### Task Tracking (ba)
```bash
ba ready                 # List available tasks
ba claim <id>            # Claim a task
ba finish <id>           # Mark complete
```

### Working Memory (wm)
```bash
wm dive-prep "intent"    # Prepare context
wm show                  # Show context
```

### Metacognitive Review (sg)
```bash
sg review                # Review before commit
```

---

## Troubleshooting

**"Command not found"**
→ Run: `bash dev_tools/bottle/bootstrap.sh`

**"Figma not working"**
→ Set `FIGMA_API_KEY` in `~/.zshrc`

---

## Files

```
dev_tools/bottle/
├── manifest.json    # Tool definitions
├── bootstrap.sh     # Setup script
├── SETUP.md         # AI assistant guidance
└── README.md        # This file
```
