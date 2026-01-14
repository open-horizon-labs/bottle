# /bottle:getting-started

Welcome to the Open Horizon Labs toolkit. This guide helps you understand what's available and how to use it effectively.

## The Tools

You have access to four core tools that work together:

| Tool | Purpose | Key Commands |
|------|---------|--------------|
| **ba** | Task tracking | `/ba:status`, `/ba:create`, `/ba:claim` |
| **wm** | Working memory & session prep | `/dive-prep`, `/wm:compile` |
| **superego** | Metacognitive review | `/superego:review` |
| **oh-mcp** | Strategic alignment | Connected via MCP |

## When To Use What

**Starting a session?**
Run `/dive-prep` first. It gathers context about your project and creates a grounded starting point.

**Working on a task?**
Use `ba` to track what you're doing:
- `/ba:status` - See current tasks
- `/ba:claim <id>` - Start working on a task
- `/ba:finish <id>` - Mark it done

**Making significant changes?**
Run `/superego:review` before committing. It catches issues you might miss.

**Need to recall past context?**
Use `/wm:compile` to surface relevant knowledge from previous sessions.

## Recommended Workflows

### Bug Fix
```
1. /dive-prep --intent fix
2. /ba:claim <task-id>
3. Investigate and fix
4. /superego:review
5. Commit and PR
6. /ba:finish <task-id>
```

### New Feature
```
1. /dive-prep --intent plan
2. Design the approach
3. /ba:create "Implement feature X"
4. /ba:claim <new-task-id>
5. Implement with tests
6. /superego:review
7. Commit and PR
```

### Exploration
```
1. /dive-prep --intent explore
2. Read code and docs
3. Document findings
4. /wm:compile to check related context
```

## Quick Reference

| Want to... | Run |
|------------|-----|
| Start a session | `/dive-prep` |
| See your tasks | `/ba:status` |
| Get feedback on work | `/superego:review` |
| Check tool status | `/bottle:status` |
| Get help | `/bottle:help` |

## Tips

1. **Always start with `/dive-prep`** - Even quick fixes benefit from context
2. **Use ba for everything** - Track even small tasks; it helps continuity
3. **Review before committing** - `/superego:review` catches drift early
4. **Let wm learn** - It captures tacit knowledge automatically

## Need More?

- `/bottle:status` - Check installed tools
- `/bottle:help` - See all bottle commands
- `/ba:quickstart` - Get ba set up quickly
