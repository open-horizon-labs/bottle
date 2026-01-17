# Open Horizon Labs Stack

## Quick Start: Dive First

**No dive is too small for a dive prep.** The metaphor comes from scuba diving: you prep before you dive, you don't just splash in. Even a quick bug fix benefits from explicit intent.

Start every session with a dive:
```
/dive-prep --intent fix     # Bug fix
/dive-prep --intent plan    # Design work
/dive-prep --intent explore # Understanding code
```

This creates `.wm/dive_context.md` with your intent, relevant context, and suggested workflow. The 30 seconds of setup prevents 30 minutes of drift.

---

This project uses Open Horizon Labs tools. Follow these protocols:

## Task Tracking (ba)

**When to use:**
- At session start: Use `ba-status` to see active tasks
- Before starting work: Use `ba list` to check what's ready
- When creating tasks: Use `ba create` for each distinct piece of work
- During work: Use `ba claim` to take ownership, `ba finish` when done
- For dependencies: Use `ba block` to mark blockers

**Protocol:** Always track non-trivial work in ba. If a task has multiple steps or will take >5 minutes, create a task.

## Working Memory (wm)

**When to use:**
- When you need context: Use `wm compile` to get relevant knowledge for current work
- If you don't know why/how something works: Check `wm show state` or encourage user to prep a dive pack
- After completing work: Use `wm distill` to extract learnings from the session
- Before answering questions about past work: Check `wm compile` first

**Dive terminology:**
- **dive-prep** = the action of preparing context before work
- **dive pack** = a reusable bundle of context for a type of work
- **dive context** = the session-specific manifest created by dive-prep

**Protocol:** Treat wm as your external memory. Don't guess at past decisions - check wm first.

## Metacognition (superego)

**Mode:** Pull mode - evaluates only when explicitly requested, not automatically.

**When to use:**
- Before committing significant work: Proactively request evaluation
- When uncertain about approach: Ask for feedback
- If you receive SUPEREGO FEEDBACK: critically evaluate it and either incorporate or escalate to user

**Protocol:** Superego is opt-in. Use it for high-stakes decisions, architectural choices, or when you want a second opinion. It catches premature commitment, scope creep, and misalignment.

## Development Workflow

**Work tracking:** GitHub issues for features/bugs, PRs for implementation.

**PR workflow:**
1. Create PR with task list (markdown checkboxes) in description
2. Run `/superego:review pr` before requesting review
3. CodeRabbit provides automated review
4. Address feedback, update checkboxes as tasks complete
5. Merge triggers release workflow

**Releases:** Automated on merge to master. Version bumps should be committed before merge.

**Review tools:**
- `sg review pr` - Superego review of PR diff
- CodeRabbit - Automated code review on PR
- Both are advisory, not blocking
