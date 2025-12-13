---
description: Template structure for implementation plans (referenced by create_plan)
---

# Plan Template

Use this structure for all implementation plans. All `[brackets]` must be resolved before delivery.

---

````markdown
# [Feature/Task Name] Implementation Plan

## Overview

[2-3 sentences: what we're implementing and why]

## Current State

[What exists now, key constraints, relevant patterns discovered]

**Key Discoveries**:
- [Finding with file:line reference]
- [Pattern to follow]
- [Constraint to work within]

## Desired End State

[Specification of the outcome. How to verify we're done.]

## What We're NOT Doing

[Explicit out-of-scope items to prevent scope creep]

- [Item 1]
- [Item 2]

## Integration Map

| Type | Location | Notes |
|------|----------|-------|
| Entry point | `file:line` | Where new code is invoked |
| Registration | `file:line` | Config/registry updates |
| Exports | `file:line` | Barrel file changes |
| Consumers | `file:line` | Existing code that will use this |
| Events | `file:line` or N/A | Pub/sub wiring |

## Implementation Approach

[High-level strategy. Why this sequence of phases.]

---

## Phase 1: [Descriptive Name]

### Goal
[What this phase accomplishes — one sentence]

### Integration Points

**Depends on**: [Prior phase or existing code]
**Produces for next phase**: [What Phase 2 will consume]

**Wiring required**:
- [ ] [Specific integration task with file:line]

### Changes

#### [Component/File Group]

**File**: `path/to/file.ext`

**Change**: [Summary of modification]

```[language]
// Specific code to add/modify
```

#### [Next Component]

...

### Success Criteria

#### Automated Verification
- [ ] Tests pass: `[test command]`
- [ ] Types check: `[type command]`
- [ ] Lint passes: `[lint command]`

#### Integration Verification
- [ ] New code importable from `[expected location]`
- [ ] Downstream files compile: `[specific files or command]`
- [ ] Integration test: `[command]` (if applicable)

#### Manual Verification
- [ ] [Specific behavior to verify]
- [ ] [Edge case to test]

**Checkpoint**: Pause for manual verification before proceeding to Phase 2.

---

## Phase 2: [Descriptive Name]

### Goal
[One sentence]

### Integration Points

**Consumes from Phase 1**: `[import/interface]` from `[file:line]`
**Produces for next phase**: [What this exposes]

**Wiring required**:
- [ ] Wire `[component]` to `[consumer]` at `[file:line]`
- [ ] Register in `[config/registry]` at `[file:line]`
- [ ] Update barrel export in `[index file]`

### Changes

...

### Success Criteria

#### Automated Verification
- [ ] ...

#### Integration Verification
- [ ] Component registered and discoverable
- [ ] End-to-end import chain works: `[trace from entry to new code]`

#### Manual Verification
- [ ] ...

**Checkpoint**: Pause for manual verification before proceeding.

---

## Phase N: [Final Phase — Often Integration/Wiring]

### Goal
Wire all components together and verify end-to-end.

### Integration Points

**Consumes**: All prior phase outputs
**Produces**: Complete feature, externally accessible

**Wiring required**:
- [ ] [Final registration]
- [ ] [Route/endpoint exposure]
- [ ] [Feature flag enablement]

### Changes

...

### Success Criteria

#### Automated Verification
- [ ] Full test suite: `[command]`
- [ ] E2E tests: `[command]`

#### Integration Verification
- [ ] Feature accessible from expected entry point
- [ ] All registrations verified
- [ ] No orphaned code (everything wired)

#### Manual Verification
- [ ] Feature works end-to-end via UI/API
- [ ] Performance acceptable
- [ ] Error handling works as expected

---

## Testing Strategy

### Unit Tests
- [What to test]
- [Key edge cases]

### Integration Tests
- [Cross-component scenarios]
- [Wiring verification]

### E2E Tests
- [User-facing scenarios]

### Manual Testing Checklist
1. [ ] [Step-by-step verification]
2. [ ] [Edge case to manually test]
3. [ ] [Error condition to verify]

## Rollback Plan

[How to revert if something goes wrong]

**If no complex rollback needed**:
```
Git revert to commit before Phase 1: `git revert --no-commit HEAD~N..HEAD`
```

**If data migration involved**:
- [ ] [Specific rollback migration]
- [ ] [Data restoration steps]

**If feature flags used**:
- [ ] Disable flag at `[location]`

## Migration Notes

[If applicable]

- **Data migration**: [Steps or "None required"]
- **Feature flags**: [Flag name and location or "None"]
- **Backwards compatibility**: [Concerns or "Not applicable"]

## References

- Ticket: `[path or link]`
- Related research: `[path]`
- Similar implementation: `[file:line]`
````

---

## Template Rules

1. **Every phase has Integration Points** — no exceptions
2. **Wiring tasks are checkboxes** — explicit, trackable
3. **Success criteria are categorized** — Automated / Integration / Manual
4. **Checkpoints between phases** — human confirms before proceeding
5. **No placeholders in final plan** — all `[brackets]` resolved before delivery
6. **Rollback plan is concrete** — specific commands or steps, not just "revert"
