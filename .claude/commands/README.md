---
description: Quick reference for development workflow automation
---

# Commands & Agents

## Commands

Commands are invoked directly: `/command_name [args]`

| Command | Purpose | When to Use |
|---------|---------|-------------|
| `/create_prompt` | Elicit and structure feature specs | Unclear requirements, need scoping |
| `/create_plan` | Create implementation plans | Starting features, complex changes |
| `/implement_plan` | Execute approved plans | After plan is reviewed and approved |
| `/research_codebase` | Deep codebase investigation | Understanding unfamiliar areas |
| `/debug` | Investigate issues | When something breaks during testing |
| `/commit` | Create git commits | After completing work |

---

## Command Details

### `/create_prompt`

Elicit requirements and produce structured specs for planning.

```bash
# Direct invocation (will guide you through questions)
/create_prompt
```

**Output**: Prompt file in `ai_docs/prompts/YYYY-MM-DD-description.md`

**Use when**:
- Requirements are vague or incomplete
- Need to scope a feature before planning
- Want to surface testing strategy early

---

### `/create_plan`

Create detailed, actionable implementation plans with explicit integration points.

```bash
# With ticket or prompt file
/create_plan ai_docs/prompts/2025-01-15-feature.md

# Direct invocation (will prompt for details)
/create_plan
```

**Output**: Plan file in `ai_docs/plans/YYYY-MM-DD-description.md`

**Key features**:
- Executes parallel research tasks
- Requires integration mapping before writing
- Iterative refinement with human review

---

### `/implement_plan`

Execute an approved plan phase by phase.

```bash
/implement_plan ai_docs/plans/2025-01-15-feature.md
```

**Behavior**:
- Reads plan and tracks progress via checkboxes
- Runs automated verification after each phase
- Pauses for manual verification before proceeding
- Resumes from last unchecked item

---

### `/research_codebase`

Comprehensive investigation of how something works.

```bash
/research_codebase
# Then describe what you want to understand
```

**Output**: Research document in `ai_docs/research/YYYY-MM-DD-topic.md`

**Use for**:
- "How does authentication work in this codebase?"
- "What patterns exist for background jobs?"
- "How do similar features handle X?"

---

### `/debug`

Investigate issues without editing files.

```bash
/debug
# Then describe the problem
```

**Investigates**: Application logs, database state, git history

**Use for**:
- Feature works in tests but fails in UI
- Unexpected errors during manual testing
- "It was working yesterday"

---

### `/commit`

Create git commits for session work.

```bash
/commit
```

**Behavior**:
- Reviews changes and proposes commit groupings
- Presents plan for approval before executing
- Creates focused, atomic commits

---

## Agents

Agents are task specialists used internally by commands. You typically don't invoke these directly—commands orchestrate them as needed.

| Agent | Purpose | Use For |
|-------|---------|---------|
| `codebase-finder` | Locate files and patterns | "What files relate to X?" |
| `codebase-analyzer` | Deep implementation analysis | "How does this code work?" |
| `integration-finder` | Find wiring/registration points | "Where do I plug this in?" |

### Agent Selection Guide

| Question Type | Agent | Mode |
|---------------|-------|------|
| Where are files located? | codebase-finder | locations-only |
| How is X done elsewhere? | codebase-finder | with-examples |
| How does THIS code work? | codebase-analyzer | — |
| Where does new code wire in? | integration-finder | — |

---

## Workflow Examples

### New Feature (Clear Requirements)

```
1. /create_plan ai_docs/prompts/2025-12-09-new-feature.md
   → Creates plan, iterates until approved

2. /implement_plan ai_docs/plans/2025-01-15-eng-5678-feature.md
   → Implements phase 1, pauses for manual test

3. [You test manually, confirm]
   → Continues to phase 2...

4. /debug (if something breaks)
   → Investigates, suggests fix

5. /commit
   → Creates commits for completed work
```

### New Feature (Unclear Requirements)

```
1. /create_prompt
   → Guided elicitation, produces structured spec

2. /create_plan ai_docs/prompts/2025-01-15-feature.md
   → Plans from structured spec

3. Continue as above...
```

---

## File Locations

| Type | Location | Naming |
|------|----------|--------|
| Commands | `.claude/commands/` | `command-name.md` |
| Agents | `.claude/agents/` | `agent-name.md` |
| Plans | `ai_docs/plans/` | `YYYY-MM-DD-description.md` |
| Research | `ai_docs/research/` | `YYYY-MM-DD-topic.md` |
| Prompts | `ai_docs/prompts/` | `YYYY-MM-DD-description.md` |
