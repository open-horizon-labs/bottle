# Setup Datasphere MCP

Set up the Datasphere MCP server for knowledge graph queries during Claude Code sessions.

**Execute these steps in order:**

## Step 1: Check for datasphere CLI

Check if the `ds` CLI is installed:
```bash
which ds
```

**If ds is NOT available:** Tell the user they need to install datasphere first:
```bash
cargo install datasphere
```

After installation, they should run `ds scan` in their project to build an initial knowledge graph, then come back and run this setup command again.

**If ds IS available:** Continue to Step 2.

## Step 2: Check for existing MCP registration

Check if datasphere MCP is already registered:
```bash
claude mcp list | grep datasphere
```

If the output shows datasphere is already registered, skip to Step 4.

If not registered (no output or error), continue to Step 3.

## Step 3: Register the MCP server

The MCP server is part of the datasphere repository. Check if the repository exists locally:
```bash
ls ~/projects/datasphere/mcp/index.js 2>/dev/null
```

**If the file exists:** Install dependencies and register:
```bash
cd ~/projects/datasphere/mcp && npm install
claude mcp add datasphere -s user -- node ~/projects/datasphere/mcp/index.js
```

**If the file doesn't exist:** Clone the repository first:
```bash
git clone https://github.com/cloud-atlas-ai/datasphere.git ~/projects/datasphere
cd ~/projects/datasphere/mcp && npm install
claude mcp add datasphere -s user -- node ~/projects/datasphere/mcp/index.js
```

Verify registration succeeded:
```bash
claude mcp list | grep datasphere
```

## Step 4: Inform the user

Tell the user:
1. Setup is complete
2. They need to **restart Claude Code** for the MCP to load
3. After restart, Datasphere MCP tools will be available:
   - `datasphere_query` - Search the knowledge graph for relevant insights from past sessions
   - `datasphere_related` - Find nodes similar to a specific node in the knowledge graph
4. The knowledge graph is populated by running `ds scan` in projects or `ds start` for continuous watching
