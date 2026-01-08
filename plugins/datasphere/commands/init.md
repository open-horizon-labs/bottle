# Initialize Datasphere

Initialize datasphere for this project, building a searchable knowledge graph from Claude Code sessions.

## Step 1: Check current state
- Check if `~/.datasphere/` already exists with data - if so, tell user it's already initialized and offer to re-scan with `ds scan` or show stats with `ds stats`
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

## Step 3: Run initial scan

After `ds` binary is available, run initial scan of Claude Code sessions:
```bash
ds scan
```

This scans sessions in `~/.claude/projects/` and builds the knowledge graph in `~/.datasphere/`.

## Step 4: Confirm

Tell user datasphere is now initialized. They can:
- `ds query "your question"` - Search the knowledge graph
- `ds stats` - View database statistics
- `ds start` - Run daemon mode to watch for new sessions

---
Be concise. Detect what's available, offer appropriate options, guide user through setup.
