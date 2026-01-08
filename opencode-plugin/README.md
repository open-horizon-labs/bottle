# Bottle - Cloud Atlas AI for OpenCode

**One package, full stack:** ba (task tracking), wm (working memory), and superego (metacognition) for OpenCode.

## Installation

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

## What You Get

### ba - Task Tracking
Simple ownership-based task tracking for multi-agent workflows.

**Tools:**
- `ba-init` - Initialize ba for a project
- `ba-status` - Show current tasks and counts
- `ba-quickstart` - Quick reference guide

**Requires:** `ba` binary ([install instructions](https://github.com/cloud-atlas-ai/ba))

### wm - Working Memory
Accumulates tacit knowledge across sessions. Model-driven: the LLM calls wm when it needs context.

**Tools:**
- `wm init` - Initialize working memory
- `wm show state` - View accumulated knowledge
- `wm compile` - Get relevant context for current task
- `wm distill` - Extract knowledge from recent work

**Requires:** `wm` binary ([install instructions](https://github.com/cloud-atlas-ai/wm))

**How it works:** A soft hint in the system prompt tells the LLM about wm. When the model needs context, it calls the `wm` tool. Similar to superego's "pull mode."

### superego - Metacognition
Reviews your work before finishing. Provides feedback on approach, risks, and missed considerations.

**Tools:**
- `superego init` - Initialize superego
- `superego status` - Check if enabled
- `superego disable/enable` - Toggle evaluation
- `superego remove` - Remove from project

**How it works:** Evaluates sessions when idle. If concerns found, injects feedback into the conversation.

## Quick Start

1. **Install bottle:**
   ```bash
   npm install @cloud-atlas-ai/bottle
   ```

2. **Add to opencode.json:**
   ```json
   {
     "plugin": ["@cloud-atlas-ai/bottle"]
   }
   ```

3. **Install binaries (ba and wm require them):**
   ```bash
   # ba
   brew install cloud-atlas-ai/ba/ba
   # or: cargo install ba

   # wm
   brew install cloud-atlas-ai/wm/wm
   # or: cargo install wm
   ```

4. **Initialize in your project:**
   Ask OpenCode to initialize each tool:
   - "use ba-init to set up task tracking"
   - "use wm init to set up working memory"
   - "use superego init to set up metacognition"

## Differences from Claude Code

### ba
✅ Same functionality - tools wrap the CLI

### wm
⚠️ **Model-driven instead of automatic:**
- **Claude Code:** Automatically injects relevant context before every user prompt
- **OpenCode:** Model decides when to call `wm compile` (no per-prompt hooks available)
- **Why:** OpenCode doesn't have an equivalent to Claude Code's `UserPromptSubmit` hook
- **Benefit:** Preserves relevance filtering, avoids token bloat

### superego
✅ Same "pull mode" behavior - evaluates on session idle

## Individual Plugins

If you only want one tool, install from their respective repos:
- [ba-opencode](https://github.com/cloud-atlas-ai/ba/tree/main/opencode-plugin)
- [wm-opencode](https://github.com/cloud-atlas-ai/wm/tree/main/opencode-plugin)
- [superego-opencode](https://github.com/cloud-atlas-ai/superego/tree/main/opencode-plugin)

## Architecture

Bottle bundles all three plugins in a single npm package. Each plugin:
- Provides OpenCode tools
- Injects soft hints via `experimental.chat.system.transform`
- Registers event hooks where needed (superego uses `session.idle`)

## License

MIT

## Links

- [bottle (Claude Code marketplace)](https://github.com/cloud-atlas-ai/bottle)
- [ba](https://github.com/cloud-atlas-ai/ba)
- [wm](https://github.com/cloud-atlas-ai/wm)
- [superego](https://github.com/cloud-atlas-ai/superego)
