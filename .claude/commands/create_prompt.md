---
description: Generate well-structured prompts for create_plan through guided elicitation
model: opus
---

# Create Prompt

Elicit and structure feature requirements into plannable specifications. Focus on WHAT and WHY—`create_plan` handles HOW.

## Initial Response

```
I'll help you create a prompt for implementation planning.

What's the feature or task? (Even a rough idea is fine—we'll refine it together.)
```

---

## Elicitation Process

Ask questions conversationally, 2-3 at a time. Skip questions already answered. Challenge vague answers.

### Phase 1: Core Understanding

1. **Goal**: What problem does this solve or capability does it add?
2. **Trigger**: What prompted this? (Bug, user request, tech debt, new feature)
3. **Baseline**: What exists today?
4. **Affected areas**: Who/what is impacted?

### Phase 2: Scope & Boundaries

5. **In scope**: What specifically must this deliver?
6. **Out of scope**: What are you explicitly NOT doing?
7. **Dependencies**: What must exist before this can start?

### Phase 3: Vertical Slice Analysis

For features touching multiple layers (UI → API → Service → Database):

8. **Layer inventory**: Which layers does this touch?
9. **Testing reality check**: For each layer—can it be tested independently? What blocks manual testing?
10. **Decomposition preference**:

| Approach | When to Use | Testing Reality |
|----------|-------------|-----------------|
| **Back-to-front** | Clear data model, complex logic | Unit tests early; manual testing late |
| **Front-to-back** | UI-driven, unclear data needs | Can prototype UI with mocks; real integration late |
| **Thin vertical slice** | Feature can narrow to one flow | Full manual testing early on narrow scope |
| **Planner decides** | Unclear tradeoffs | Note: may hit testing gaps mid-implementation |

### Phase 4: Success Criteria

11. **Done state**: How will you know this is complete?
12. **Verification approach**: Automated tests? Manual flows?

### Phase 5: Context & Constraints

13. **Timeline/priority**: Any time pressure?
14. **Technical constraints**: Required technologies, compatibility needs?
15. **Open questions**: What should the planner investigate?

---

## Output

Once sufficient clarity is reached, generate:

### 1. Prompt File

Write to `ai_docs/prompts/YYYY-MM-DD-description.md`:

```markdown
---
date: [ISO date]
status: draft
target_command: create_plan
---

# [Feature/Task Name]

## Goal

[1-2 sentences: what problem this solves]

## Background

[What triggered this, what exists today]

## Requirements

### Must Have
- [Specific requirement]

### Out of Scope
- [Explicitly excluded item]

## Affected Areas

- **Users/Personas**: [who is affected]
- **Systems/Components**: [what parts of codebase]
- **Data**: [what data is created, modified, read]

## Vertical Slice Analysis

**Layers involved**: [UI, API, Service, Database, etc.]

**Decomposition approach**: [chosen approach]

**Rationale**: [why this approach]

## Success Criteria

### Automated Verification
- [ ] [Specific test or check]

### Manual Verification
- [ ] [Specific user flow to test]

## Open Questions for Planning

- [Question requiring codebase investigation]

## Constraints

- [Timeline, technical, or other constraints]

---

**To execute**: `/create_plan ai_docs/prompts/YYYY-MM-DD-description.md`
```

### 2. Quick Summary

```
Created: `ai_docs/prompts/YYYY-MM-DD-description.md`

Summary:
- [One-line goal]
- [Key complexity or risk]
- [Recommended decomposition and why]

Ready to plan? Run:
/create_plan ai_docs/prompts/YYYY-MM-DD-description.md
```

---

## What NOT to Do

- **Don't research the codebase** — that's `create_plan`'s job
- **Don't suggest implementation approaches** — that's planning
- **Don't accept vague requirements** — "better UX" is not a requirement
- **Don't skip vertical slice analysis** for multi-layer features
- **Don't accept "we'll figure it out"** for testing strategy
