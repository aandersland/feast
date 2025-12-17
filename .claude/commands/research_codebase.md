---
description: Research codebase comprehensively using parallel research tasks and synthesize findings
model: opus
---

# Research Codebase

Conduct comprehensive codebase research by executing focused research tasks and synthesizing findings.

## Quick Start

```
I'm ready to research the codebase.

What's your question or area of interest?
```

---

## Process

### 1. Read Mentioned Files First

If the user references files (tickets, docs, JSON):
- Read them **fully** — no truncation
- Read in main context **before** executing research tasks

### 2. Decompose the Question

Break down the query:
- What components/patterns need investigation?
- What directories are relevant?
- What connections might exist?

Create a research plan with TodoWrite.

### 3. Execute Research Tasks

Execute appropriate research tasks based on question type:

| Question Type | Approach | Focus |
|---------------|----------|-------|
| Where are files? | codebase-finder patterns | locations-only |
| How is X done elsewhere? | codebase-finder patterns | with-examples |
| How does code work? | codebase-analyzer patterns | Deep trace |
| Where does code wire in? | integration-finder patterns | Registrations |

**Complete all research tasks before synthesizing.**

### 4. Synthesize Findings

- Prioritize live codebase over documentation
- Connect findings across components
- Include file:line references
- Highlight patterns and architectural decisions

### 5. Write Research Document

Save to `ai_docs/research/YYYY-MM-DD-topic-name.md`:

```markdown
---
date: [ISO datetime with timezone]
researcher: [name]
git_commit: [hash]
branch: [branch]
repository: [repo]
topic: "[question]"
tags: [research, relevant-components]
status: complete
---

# Research: [Topic]

## Question
[Original query]

## Summary
[High-level findings — 2-3 sentences]

## Detailed Findings

### [Area 1]
- Finding with reference (`file:line`)
- Connection to other components

### [Area 2]
...

## Code References
- `path/file.py:123` — Description
- `path/file.ts:45-67` — Description

## Architecture Insights
[Patterns, conventions, design decisions discovered]

## Open Questions
[Areas needing further investigation, if any]
```

### 6. Present & Iterate

- Summarize key findings
- Include navigation references
- Offer to answer follow-ups

**For follow-up questions**:
- Append to same document
- Add `## Follow-up: [topic]` section
- Update frontmatter with `last_updated` field

---

## Research Task Reference

### File Discovery (codebase-finder, locations-only)
- Find all files related to a feature area
- Categorize: implementation, tests, config, types
- Output organized file lists

### Pattern Extraction (codebase-finder, with-examples)
- Find similar implementations
- Extract code snippets showing how X is done
- Note variations and contexts

### Implementation Analysis (codebase-analyzer)
- Trace data flow through specific code
- Document entry points, transformations, outputs
- Include file:line references for every claim

### Integration Discovery (integration-finder)
- Find registration patterns
- Identify barrel exports
- Map configuration and wiring points

### Test Discovery (test-finder)
- Find test files, utilities, fixtures, and mocks
- Identify testing framework and configuration
- Use `overview` mode for high-level landscape, `detailed` for specific file locations

### Test Analysis (test-analyzer)
- Analyze what code paths are tested vs untested
- Document test patterns, setup/teardown, mocking strategies
- Use `coverage` mode for what's tested, `quality` mode for test patterns and structure

---

## Handling No Results

If research tasks find nothing relevant:

```markdown
## Research: [Topic]

### Search Performed
- Searched for: [keywords/patterns]
- Locations checked: [directories]

### Result
No matching files or patterns found.

### Possible Reasons
- [Feature may not exist yet]
- [May use different naming]
- [May be in unexpected location]

### Suggested Next Steps
- [Try alternative search terms]
- [Check specific directory]
- [Clarify what you're looking for]
```

---

## What NOT to Do

- **Don't start research before reading provided files** — context first
- **Don't synthesize before completing all research** — gather everything first
- **Don't skip file:line references** — every claim needs evidence
- **Don't include speculation** — document what exists, not what should
- **Don't leave open questions unacknowledged** — list them explicitly
- **Don't duplicate agent selection tables** — reference README for quick lookup
