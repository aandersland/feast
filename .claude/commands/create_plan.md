---
description: Create detailed implementation plans through iterative research and collaboration
model: opus
---

# Create Implementation Plan

Create thorough, actionable implementation plans with explicit integration points.

## Quick Start

If a file path is provided, read it fully and begin research. Otherwise:

```
I'll help create an implementation plan.

Provide:
1. Task/ticket description (or path to ticket file)
2. Context, constraints, requirements

Tip: /create_plan path/to/ticket.md to start directly
```

---

## Process Overview

```
1. Context Gathering    → Read files, execute research tasks
2. Integration Mapping  → Identify ALL wiring points
3. Structure Review     → Present phases, get alignment
4. Plan Writing         → Write to ai_docs/plans/
5. Iteration            → Refine until approved
```

---

## Step 1: Context Gathering

### Read First

- Read ALL mentioned files completely (no truncation)
- Never start research tasks before reading provided files

### Execute Research Tasks

Execute these sequentially, then synthesize findings:

**Task 1 — File Discovery**
Using codebase-finder patterns (locations-only mode):
- Find all files related to the feature area
- Identify implementation, test, config, and type files

**Task 2 — Implementation Analysis**
Using codebase-analyzer patterns:
- Trace how current/similar implementations work
- Document data flow and key functions

**Task 3 — Pattern Matching**
Using codebase-finder patterns (with-examples mode):
- Find similar features to model after
- Extract reusable patterns

### Present Findings

```markdown
Based on the ticket and codebase research:

**Understanding**: [accurate summary]

**Discovered**:
- [Implementation detail with file:line]
- [Pattern or constraint]

**Questions requiring human judgment**:
- [Specific technical question]
```

Only ask questions research couldn't answer.

---

## Step 2: Integration Mapping

**Do not skip this step.** Plans fail when integration points are missed.

### Execute Integration Research

Using integration-finder patterns, identify:
- Where similar features register themselves
- What barrel exports need updating
- What configs/DI containers need entries
- What event subscriptions exist

### Complete Integration Map

Before writing any plan, answer ALL of these:

| Integration Type | Location | File:Line |
|------------------|----------|-----------|
| **Entry point** — Where does new code get invoked? | | |
| **Registration** — What registries/configs need updates? | | |
| **Exports** — What barrel files need changes? | | |
| **Consumers** — What existing code will use this? | | |
| **Events** — What pub/sub wiring is needed? | | |

**If any row is empty or unknown, research more before proceeding.**

### Present Integration Map

```markdown
**Integration Points Identified**:

| Type | Location | Notes |
|------|----------|-------|
| Entry | `src/routes/index.ts:45` | Registers route handlers |
| Registration | `src/config/features.ts:12` | Feature flag |
| Exports | `src/services/index.ts` | Barrel export |
| Consumers | `src/handlers/main.ts:89` | Will import new service |
| Events | N/A | None required |

These will be explicit steps in the plan.
```

---

## Step 3: Structure Review

Present phases before writing details:

```markdown
## Proposed Structure

**Overview**: [1-2 sentence summary]

**Phases**:
1. [Phase] — [outcome]
2. [Phase] — [outcome]
3. [Phase] — [outcome]

**Integration occurs in**: Phase [N]

Does this phasing make sense?
```

Get approval before proceeding.

---

## Step 4: Write Plan

Write to `ai_docs/plans/YYYY-MM-DD-description.md`

Use the template from `_plan-template.md`. Key requirements:

- Every phase has **Integration Points** section
- Success criteria split: **Automated** / **Integration** / **Manual**
- No open questions — resolve before writing
- All `[brackets]` resolved with actual values

---

## Step 5: Review & Iterate

```markdown
Plan created at: `ai_docs/plans/[filename]`

Review for:
- Are phases properly scoped?
- Are integration points complete?
- Are success criteria specific?
```

Continue refining until approved.

---

## Research Task Reference

| Need | Approach | Focus |
|------|----------|-------|
| What files exist? | codebase-finder patterns | locations-only |
| How is X done elsewhere? | codebase-finder patterns | with-examples |
| How does THIS code work? | codebase-analyzer patterns | Deep trace |
| Where does new code wire in? | integration-finder patterns | Registrations, exports |

---

## What NOT to Do

- **Don't skip reading provided files** — read completely before research
- **Don't write plans with unknown integration points** — research until all are identified
- **Don't leave open questions** — resolve uncertainties before writing
- **Don't use placeholder text** — all `[brackets]` must be resolved
- **Don't skip structure review** — get alignment before detailed writing
- **Don't assume patterns** — verify with actual code references
