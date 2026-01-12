# Setup Open Horizons MCP

Set up the Open Horizons MCP server for strategic alignment integration.

**Execute these steps in order:**

## Step 1: Check for existing API key config

Check if `~/.config/openhorizons/config.json` exists and has an api_key:
```bash
cat ~/.config/openhorizons/config.json 2>/dev/null
```

If the file exists and has a valid api_key, skip to Step 3.

If not configured, check if `sg` (superego CLI) is available:
```bash
which sg
```

**If sg is available:** Tell the user to run `sg setup-oh` in their terminal. This will:
- Open browser to get API key
- Prompt them to paste the key
- Create the config file

**If sg is NOT available:** Ask the user for their API key, then create the config:
```bash
mkdir -p ~/.config/openhorizons
```

Then write to `~/.config/openhorizons/config.json`:
```json
{
  "api_key": "<USER_PROVIDED_KEY>",
  "api_url": "https://app.openhorizons.me"
}
```

Tell them to get a key from https://app.openhorizons.me/settings/api-keys if they don't have one.

## Step 2: Add MCP server to Claude Code

Use the `claude mcp add` command to register the server with npx (works across all Node.js versions):
```bash
claude mcp add oh-mcp --scope user -- npx -y @cloud-atlas-ai/oh-mcp-server
```

This adds oh-mcp to `~/.claude.json` so it's available across all projects. Using `npx` ensures it works regardless of which Node.js version each project uses.

## Step 3: Inform the user

Tell the user:
1. Setup is complete
2. They need to **restart Claude Code** for the MCP to load
3. After restart, OH MCP tools will be available:
   - `oh_get_contexts` - List workspaces
   - `oh_get_endeavors` - Browse endeavors
   - `oh_log_decision` - Log decisions
   - `oh_about` - Test the connection
