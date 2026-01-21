# SETUP.md - Example Team

AI assistant guidance for environment setup and troubleshooting.

## Environment Check

Run this to verify setup:

```bash
echo "=== Environment Check ===" && \
command -v ba > /dev/null && echo "ba: OK" || echo "ba: MISSING" && \
command -v wm > /dev/null && echo "wm: OK" || echo "wm: MISSING" && \
command -v sg > /dev/null && echo "sg: OK" || echo "sg: MISSING" && \
command -v opencode > /dev/null && echo "opencode: OK" || echo "opencode: MISSING" && \
command -v az > /dev/null && echo "azure-cli: OK" || echo "azure-cli: MISSING" && \
[ -n "$EXAMPLE_PROJECT_ID" ] && echo "Env vars: OK" || echo "Env vars: MISSING"
```

## Fix Missing Components

**Tools missing:**
```bash
bash dev_tools/bottle/bootstrap.sh
```

**Env vars missing:**
Add to `~/.zshrc`:
```bash
export EXAMPLE_PROJECT_ID="your-project-id"
```
Then: `source ~/.zshrc`

**Project not initialized:**
```bash
ba init && wm init && sg init
```

---

## Troubleshooting

### "Command not found: ba/wm/sg"
1. Check if bottle installed tools: `bottle status`
2. Ensure `~/.cargo/bin` is in PATH
3. Re-run: `bottle install --manifest dev_tools/bottle/manifest.json`

### "Figma MCP not working"
1. Check env var: `echo $FIGMA_API_KEY`
2. Get API key from: https://www.figma.com/developers/api#access-tokens
3. Add to ~/.zshrc: `export FIGMA_API_KEY="your-key"`

---

## Tool Usage

### Task Tracking (ba)
```bash
ba ready              # List available tasks
ba claim <id>         # Claim a task
ba finish <id>        # Mark complete
ba create "title"     # New task
```

### Working Memory (wm)
```bash
wm dive-prep "intent" # Prepare context for session
wm show               # Show current context
```

### Metacognitive Review (sg)
```bash
sg review             # Review before commit
```

---

## Figma (if configured)

Share Figma URLs in the AI assistant to get design context.
Requires `FIGMA_API_KEY` environment variable.
