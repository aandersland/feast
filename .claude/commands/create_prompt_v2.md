---
description: Create chunk-level spec from a roadmap with minimal elicitation
model: opus
---

# Create Prompt V2

Create a detailed prompt for a specific chunk from an existing roadmap. Inherits strategic context from the roadmap—only elicits chunk-specific details.

## Arguments

Expects: `[roadmap-path] [chunk-number]`

Example: `/create_prompt_v2 ai_docs/roadmaps/2024-12-16-user-auth.md 1`

---

## Behavior

### Step 1: Parse Arguments

Extract from `$ARGUMENTS`:
- `roadmap_path`: Path to the roadmap file
- `chunk_number`: Which chunk to create a prompt for (1-indexed)

If arguments are missing or invalid, respond:
```
Usage: /create_prompt_v2 [roadmap-path] [chunk-number]

Example: /create_prompt_v2 ai_docs/roadmaps/2024-12-16-user-auth.md 1

Please provide the roadmap path and chunk number.
```

### Step 2: Read Roadmap

Read the roadmap file and extract:
- Feature name
- Vision
- Background
- Affected areas
- Testing strategy
- Constraints
- Total chunk count
- The specific chunk's details (purpose, depends on, produces, key considerations)

If the chunk number doesn't exist, respond with available chunks.

### Step 3: Balanced Elicitation

**Only ask about what the roadmap doesn't cover.** Do NOT re-ask about vision, trigger, affected areas, or constraints—these are inherited.

Ask 2-3 questions at a time about:

1. **Edge cases**: "What should happen when [X boundary condition]?"
2. **Ambiguities**: "The roadmap says [Y]—does that mean [A] or [B]?"
3. **Scope boundaries**: "Should [Z] be in this chunk or deferred to chunk N?"
4. **Specific requirements**: "What exactly should [behavior] look like?"

**Skip elicitation if** the roadmap's key considerations are detailed enough and no ambiguities exist. Proceed directly to output.

**Challenge vague answers.** "It should just work" is not an edge case handler.

### Step 4: Generate Output

---

## Output

Write to `ai_docs/prompts/YYYY-MM-DD-NN-chunk-name.md` (NN = zero-padded chunk number):

```markdown
---
date: [ISO date]
status: draft
parent_roadmap: [roadmap path]
chunk: [N]
chunk_name: [Chunk Name]
target_command: create_plan
---

# [Chunk Name]

## Inherited Context

> **Feature**: [Feature name from roadmap]
> **Roadmap**: [roadmap path]
> **Chunk**: [N] of [total]
> **Depends on**: [dependencies from roadmap]
> **Produces**: [what this enables from roadmap]

## Goal

[Chunk-specific goal derived from roadmap's purpose statement]

## Background

[Relevant context from roadmap—vision, trigger, what exists. Keep brief; link to roadmap for full context.]

## Requirements

### Must Have
- [Specific requirement for this chunk]
- [Another requirement]

### Out of Scope (handled by other chunks)
- [Item explicitly deferred to chunk N]
- [Another deferred item]

## Affected Areas

[Narrowed from roadmap to what THIS chunk touches]

- **Users/Personas**: [subset relevant to this chunk]
- **Systems/Components**: [specific components this chunk modifies]
- **Data**: [data this chunk creates, modifies, reads]

## Edge Cases

- [Edge case identified during elicitation]
- [Another edge case]

## Success Criteria

### Automated Verification
- [ ] [Specific test or type check]
- [ ] [Another automated check]

### Manual Verification
- [ ] [Specific user flow to test]
- [ ] [Another manual check]

## Open Questions for Planning

- [Remaining unknown that create_plan should investigate]

---

**To execute**: `/create_plan ai_docs/prompts/YYYY-MM-DD-NN-chunk-name.md`
```

### Quick Summary

```
Created: `ai_docs/prompts/YYYY-MM-DD-NN-chunk-name.md`

Summary:
- Chunk [N] of [total]: [chunk name]
- [One-line goal]
- [Key edge case or consideration]

Ready to plan? Run:
/create_plan ai_docs/prompts/YYYY-MM-DD-NN-chunk-name.md

Next chunk:
/create_prompt_v2 [roadmap-path] [N+1]
```

---

## What NOT to Do

- **Don't re-ask roadmap-level questions** — vision, trigger, constraints are inherited
- **Don't accept vague edge case handling** — "it should just work" needs specifics
- **Don't skip scope boundaries** — clarity on what's in vs deferred prevents scope creep
- **Don't research the codebase** — that's `create_plan`'s job
- **Don't suggest implementation approaches** — that's planning
- **Don't change roadmap decisions** — if the roadmap is wrong, update it first
