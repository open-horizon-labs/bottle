# bottle-init

Initialize all installed Cloud Atlas AI tools with recommended defaults. Handles binary installation automatically.

## Step 1: Check and install binaries

For each tool (ba, wm, sg), check if binary exists:

**If binary missing:**

1. **Detect available package managers:**
   - Homebrew: `command -v brew`
   - Cargo: `command -v cargo` OR `test -f ~/.cargo/bin/cargo`

2. **Offer installation:**

   If **Homebrew** available (preferred for macOS):
   ```bash
   brew install cloud-atlas-ai/ba/ba
   brew install cloud-atlas-ai/wm/wm
   brew install cloud-atlas-ai/superego/superego
   ```

   If **Cargo** available:
   ```bash
   cargo install ba
   cargo install working-memory  # (published as 'working-memory', provides 'wm' binary)
   cargo install superego
   ```

   If **neither available**, offer to install a package manager:
   - **Install Homebrew** (macOS/Linux):
     ```bash
     /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
     ```

   - **Install Rust** (cross-platform):
     ```bash
     curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
     ```

3. After installation, verify binaries are available before proceeding

## Step 2: Initialize individual tools

Once binaries are available, initialize each:

**ba:**
- If `.ba/` doesn't exist, run: `ba init`
- This creates `.ba/` directory and initial configuration

**superego:**
- If `.superego/` doesn't exist, run: `sg init`
- This creates `.superego/` directory and sets up metacognitive monitoring

**wm:**
- If `.wm/` doesn't exist, run: `wm init`
- This creates `.wm/` directory and enables working memory capture

## Step 3: Apply recommended defaults

After individual inits complete:

**Set superego to pull mode (recommended):**
- Check if `.superego/config.yaml` exists
- Read the current mode setting
- If mode is `always`, change it to `pull`
- Command: `sed -i.bak 's/^mode: always/mode: pull/' .superego/config.yaml && rm .superego/config.yaml.bak`
- Explain: "Pull mode is less intrusive - superego reviews when you request it or before commits/PRs, rather than at every checkpoint"

## Step 4: Create/update AGENTS.md

Create AGENTS.md with guidance for all initialized tools:
- Add sections for ba, wm, and superego
- Include "When to use" and "Protocol" guidance for each
- If AGENTS.md already exists, preserve user content while updating tool sections

## Step 5: Confirm completion

Tell user:
```bash
Bottle initialization complete

Initialized:
  - ba - Task tracking ready
  - superego - Metacognitive advisor active (pull mode)
  - wm - Working memory enabled

Configuration:
  - superego mode: pull (recommended)
  - AGENTS.md: Updated with tool guidance

Quick start:
  ba create "Your first task" -t task
  ba ready
  ba claim <id> --session $SESSION_ID

Tools work together:
  - ba tracks your work
  - superego reviews before commits (sg review)
  - wm captures learnings automatically
```

---

Run each tool's init command in sequence. Apply recommended defaults after inits complete. Focus on orchestration, not reimplementation.
