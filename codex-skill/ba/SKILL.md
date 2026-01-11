---
name: ba
description: Task tracking. "$ba status" for current tasks, "$ba claim <id>" to start work, "$ba finish <id>" when done.
---

# ba - Task Tracking for AI Sessions

Simple, file-based task tracking. No server, no database - just files in `.ba/`.

## $ba status

Show current task status - what's ready, what's claimed, what's blocked.

**Run:**
```bash
echo "=== Ready to Work ==="
ba ready

echo ""
echo "=== Currently Claimed ==="
ba mine

echo ""
echo "=== All Open ==="
ba list --status open
```

**Tell user:** Summary of tasks ready for work, currently claimed, and total open.

## $ba init

Initialize task tracking for this project.

**Step 1:** Check if ba binary is installed:
```bash
if ! command -v ba >/dev/null; then
  echo "ba binary not installed. Install with:"
  echo "  brew install cloud-atlas-ai/tap/ba"
  echo "  # or: cargo install ba"
  exit 1
fi
```

**Step 2:** Initialize .ba/ directory:
```bash
if [ ! -d ".ba" ]; then
  ba init
  echo "✓ .ba/ initialized"
else
  echo "✓ .ba/ already exists"
fi
```

**Step 3:** Show quickstart:
```bash
ba quickstart
```

## $ba claim <id>

Claim a task to start working on it.

**Run:**
```bash
ba claim <id>
```

**Then show the task details:**
```bash
ba show <id>
```

**Tell user:** "Claimed task <id>. Task details shown above."

## $ba finish <id>

Mark a task as complete (releases claim and closes).

**Run:**
```bash
ba finish <id>
```

**Tell user:** "Task <id> completed."

## $ba create <title>

Create a new task.

**Run:**
```bash
ba create "<title>"
```

The command outputs the new task ID.

**Tell user:** "Created task <id>: <title>"

## $ba ready

Show tasks that are ready to work on (open and not blocked).

**Run:**
```bash
ba ready
```

## $ba list

List all tasks with optional filters.

**Run:**
```bash
ba list                    # All tasks
ba list --status open      # Open tasks only
ba list --status closed    # Closed tasks only
ba list --label <label>    # Tasks with specific label
```

## $ba show <id>

Show details of a specific task including comments and blocking relationships.

**Run:**
```bash
ba show <id>
```

## $ba block <id> <blocker>

Mark that task <id> is blocked by task <blocker>.

**Run:**
```bash
ba block <id> <blocker>
```

**Tell user:** "Task <id> is now blocked by <blocker>."

## $ba unblock <id> <blocker>

Remove a blocking relationship.

**Run:**
```bash
ba unblock <id> <blocker>
```

## $ba comment <id> <comment>

Add a comment to a task.

**Run:**
```bash
ba comment <id> "<comment>"
```

## $ba tree

Show the dependency tree of all tasks.

**Run:**
```bash
ba tree
```

## $ba quickstart

Show the quick start guide for using ba.

**Run:**
```bash
ba quickstart
```

---

## Workflow

Typical task workflow:

1. **Check what's ready:** `$ba status` or `$ba ready`
2. **Claim a task:** `$ba claim <id>`
3. **Do the work**
4. **Add comments if needed:** `ba comment <id> "progress note"`
5. **Finish when done:** `$ba finish <id>`

**Protocol:** Always track non-trivial work. If a task has multiple steps or will take >5 minutes, create a task.
