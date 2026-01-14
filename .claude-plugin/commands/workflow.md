# /bottle:workflow

Guide users through the recommended workflow for their current task.

## Usage

```
/bottle:workflow [intent]
```

**Intents:** `fix`, `plan`, `explore`, `review`, `ship`

If no intent provided, ask the user what they're trying to do.

## Execution

### Step 1: Determine Intent

If intent not provided, present options:

```
What are you working on?

[ ] fix     - Fix a bug or issue
[ ] plan    - Design an approach
[ ] explore - Understand something
[ ] review  - Reflect on recent work
[ ] ship    - Deploy or release
```

### Step 2: Check Prerequisites

Before showing workflow, verify tools are ready:

```bash
# Check if ba is initialized
test -d .ba && echo "ba ready" || echo "Run: ba init"

# Check if wm is initialized
test -d .wm && echo "wm ready" || echo "Run: wm init"

# Check if superego is configured
test -d .superego && echo "superego ready" || echo "Run: sg init"
```

If any tool is missing, offer to initialize:
```
Some tools aren't set up yet. Run /bottle:init to configure them?
[Yes] [No, show workflow anyway]
```

### Step 3: Show Workflow

Based on intent, display the workflow with actionable steps:

---

#### Intent: fix

```
Bug Fix Workflow
================

1. PREPARE
   → /dive-prep --intent fix
   Gathers project context and sets focus

2. CLAIM TASK
   → /ba:status (see available tasks)
   → /ba:claim <id> (or create new: /ba:create "Fix: description")

3. INVESTIGATE
   - Reproduce the issue
   - Find root cause
   - Check for related problems

4. FIX
   - Write failing test first (if applicable)
   - Implement minimal fix
   - Verify test passes

5. REVIEW
   → /superego:review
   Catches issues before commit. Fix any P1-P3 findings.

6. COMMIT
   - Clear commit message referencing the issue
   - Push and create PR

7. COMPLETE
   → /ba:finish <id>

Current status: [show ba status if available]
```

---

#### Intent: plan

```
Planning Workflow
=================

1. PREPARE
   → /dive-prep --intent plan
   Gathers context for informed planning

2. UNDERSTAND
   - What problem are we solving?
   - What constraints exist?
   - What's been tried before?

3. OPTIONS
   - List possible approaches
   - Identify trade-offs
   - Note dependencies

4. DECIDE
   - Choose approach with rationale
   - Document decision
   - Break into tasks

5. TRACK
   → /ba:create "Task 1: ..."
   → /ba:create "Task 2: ..."
   Create tasks for each piece of work

Ready to start planning? Run: /dive-prep --intent plan
```

---

#### Intent: explore

```
Exploration Workflow
====================

1. PREPARE
   → /dive-prep --intent explore
   Sets up exploration context

2. SCOPE
   - What are you trying to understand?
   - What's the boundary?

3. INVESTIGATE
   - Read relevant code
   - Check documentation
   - Trace data flow

4. RECALL
   → /wm:compile
   Surfaces related context from past sessions

5. DOCUMENT
   - Note key findings
   - Identify questions for later
   - Update project docs if needed

Start exploring: /dive-prep --intent explore
```

---

#### Intent: review

```
Review Workflow
===============

1. PREPARE
   → /dive-prep --intent review
   Gathers recent work context

2. GATHER
   - Recent commits
   - Open tasks
   - Uncommitted changes

3. REFLECT
   → /superego:review
   Get metacognitive feedback on current state

4. ASSESS
   - What went well?
   - What was harder than expected?
   - What would you do differently?

5. CAPTURE
   - Document insights
   - Update relevant docs
   - Create follow-up tasks if needed

Start review: /dive-prep --intent review
```

---

#### Intent: ship

```
Ship Workflow
=============

1. PREPARE
   → /dive-prep --intent ship
   Verifies deployment readiness

2. VERIFY
   - All tests pass?
   - No uncommitted changes?
   - Documentation updated?

3. REVIEW
   → /superego:review
   Final check before shipping

4. PR
   - Create PR with full context
   - Reference related issues
   - Describe changes clearly

5. DEPLOY
   - Follow project deployment process
   - Monitor for issues
   - Update task status

6. COMPLETE
   → /ba:finish <id>
   Mark related tasks as done

Ready to ship? Start with: /dive-prep --intent ship
```

---

### Step 4: Offer Next Action

After showing workflow, offer to start:

```
Ready to begin?

[Start with dive-prep] [Show ba status first] [Just show steps]
```

If user chooses to start, invoke `/dive-prep --intent <selected-intent>`.
