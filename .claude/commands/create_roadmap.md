---
description: Decompose a feature into implementable chunks with strategic decisions upfront
model: opus
---

# Create Roadmap

Elicit high-level feature requirements and decompose into implementable chunks. Focus on WHAT, WHY, and HOW IT BREAKS DOWN—individual prompts handle chunk details.

## Initial Response

```
I'll help you create a roadmap for feature implementation.

What feature or capability do you want to build? (Even a rough idea is fine—we'll structure it together.)
```

---

## Elicitation Process

Ask questions conversationally, 2-3 at a time. Skip questions already answered. Challenge vague answers. This is higher-level than `create_prompt`—focus on the whole, not the parts.

### Phase 1: Vision & Context

1. **End state**: What does success look like when this is complete?
2. **Trigger**: Why now? (User request, tech debt, new capability, competitive need)
3. **Baseline**: What exists today that this builds on or replaces?

### Phase 2: Scope & Impact

4. **Users/Personas**: Who benefits from this? Who interacts with it?
5. **Systems/Components**: What parts of the codebase will this touch?
6. **Data**: What data is created, modified, or read?

### Phase 3: Decomposition

7. **Natural chunks**: How does this break into 2-6 logical pieces?
   - Guide toward pieces that can be implemented and tested independently
   - Each chunk should deliver tangible value or enable the next chunk
   - Challenge chunks that are too vague ("improve performance") or too granular ("add button")

8. **Dependencies**: What order must these go in?
   - What must exist before each chunk can start?
   - What does each chunk produce that others need?

### Phase 4: Testing Strategy

9. **Overall approach**: How will the complete feature be verified?
   - Unit tests for logic
   - Integration tests for component interactions
   - E2E tests for user flows
   - Manual testing checkpoints

10. **Testing boundaries**: Where are the natural testing seams between chunks?

### Phase 5: Constraints

11. **Timeline/priority**: Any time pressure or sequencing with other work?
12. **Technical constraints**: Required technologies, compatibility needs, performance targets?
13. **Open questions**: What's unclear that individual prompts should investigate?

---

## Output

Once sufficient clarity is reached, generate:

### 1. Roadmap File

Write to `ai_docs/roadmaps/YYYY-MM-DD-feature-name.md`:

```markdown
---
date: [ISO date]
status: draft
feature: [Feature Name]
chunks: [N]
---

# [Feature Name]

## Vision

[2-3 sentences: what success looks like when complete]

## Background

[What triggered this, what exists today, why now]

## Affected Areas

- **Users/Personas**: [who benefits, who interacts]
- **Systems/Components**: [what parts of codebase]
- **Data**: [what data is created, modified, read]

## Testing Strategy

[Overall approach: unit, integration, E2E, manual checkpoints]

## Constraints

[Timeline, technical requirements, compatibility needs]

---

## Chunks

### 1. [Chunk Name]

**Purpose**: [One line—what this delivers]

**Depends on**: [None | Chunk N]

**Produces**: [What downstream chunks or users need from this]

**Key considerations**:
- [Guided hint for prompt creation]
- [Another consideration]

---

### 2. [Chunk Name]

**Purpose**: [One line—what this delivers]

**Depends on**: [Chunk 1 | None]

**Produces**: [What downstream chunks or users need from this]

**Key considerations**:
- [Guided hint for prompt creation]
- [Another consideration]

---

[Continue for all chunks...]

---

## Next Steps

- [ ] `/create_prompt_v2 ai_docs/roadmaps/YYYY-MM-DD-feature-name.md 1` → [Chunk 1 name]
- [ ] `/create_prompt_v2 ai_docs/roadmaps/YYYY-MM-DD-feature-name.md 2` → [Chunk 2 name]
[Continue for all chunks...]
```

### 2. Quick Summary

```
Created: `ai_docs/roadmaps/YYYY-MM-DD-feature-name.md`

Summary:
- [One-line vision]
- [N chunks identified]
- [Key dependency or sequencing note]

Ready to create prompts? Start with:
/create_prompt_v2 ai_docs/roadmaps/YYYY-MM-DD-feature-name.md 1
```

---

## What NOT to Do

- **Don't get into implementation details** — that's `create_plan`'s job
- **Don't define specific file changes** — that's planning
- **Don't accept vague chunks** — "make it better" is not a chunk
- **Don't skip dependencies analysis** — order matters
- **Don't accept "we'll figure out testing later"** — testing strategy is required
- **Don't create more than 6 chunks** — if you need more, the feature is too big; break it into multiple roadmaps
