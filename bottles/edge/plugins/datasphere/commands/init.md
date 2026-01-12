# Initialize Datasphere

Initialize datasphere for this project, building a searchable knowledge graph from Claude Code sessions.

## Step 1: Check current state
- Check if `~/.datasphere/` already exists with data - if so, tell user it's already initialized and offer to show stats with `ds stats` or restart the daemon with `ds start`
- Check if `ds` binary is available (`command -v ds`) - if yes, skip to Step 3

## Step 2: Install ds binary

**Detect available package managers:**
- Cargo: `command -v cargo` OR `test -f ~/.cargo/bin/cargo`

**Offer installation based on what's available:**

If **Cargo** available:
```bash
cargo install datasphere
# or if cargo not in PATH:
~/.cargo/bin/cargo install datasphere
```

If **Cargo not available**, offer to install Rust:
- **Install Rust** (cross-platform):
  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```
  Then restart shell and: `cargo install datasphere`

**For local development** (if user specifies a path):
```bash
cargo install --path /path/to/datasphere
# or: ~/.cargo/bin/cargo install --path /path/to/datasphere
```

## Step 3: Start datasphere daemon

After `ds` binary is available, start the daemon to watch all Claude Code sessions:
```bash
ds start
```

This runs continuously and:
- Watches `~/.claude/projects/` for all projects
- Automatically processes new and modified sessions
- Builds the knowledge graph in `~/.datasphere/`

Note: `ds scan` only processes a single session (not a full project scan). For proper workflow, always use `ds start` to run in daemon mode.

## Step 4: Confirm

Tell user datasphere is now running. They can:
- `ds query "your question"` - Search the knowledge graph
- `ds stats` - View database statistics
- `ds scan` - One-shot scan of a single session only

---
Be concise. Detect what's available, offer appropriate options, guide user through setup.
