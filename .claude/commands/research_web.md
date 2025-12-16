---
description: Research topics from the web and synthesize technical findings
model: opus
---

# Web Research

Conduct comprehensive web research by executing focused searches and synthesizing findings from authoritative sources.

## Quick Start

```
I'm ready to research the web.

What's your question or topic?
```

**With depth option:**
- `/research_web --quick` — Fast overview (3-5 sources)
- `/research_web --thorough` — Deep dive (8-12 sources)

Default depth is **quick** unless specified.

---

## Process

### 1. Interpret the Query

Break down the research question:
- What specific information is needed?
- What type of sources are most relevant?
- What search terms will yield best results?

Create a research plan with TodoWrite.

### 2. Identify Search Strategy

Match the question type to search approach:

| Question Type | Search Focus | Example Sources |
|---------------|--------------|-----------------|
| Standards/Specs | RFCs, W3C, IETF, ISO | schema.org, json-ld.org |
| Libraries/Tools | GitHub, npm, PyPI, docs | Official documentation |
| APIs | Official docs, examples | Developer portals |
| Best Practices | Articles, guides, tutorials | MDN, Stack Overflow |
| Comparisons | Reviews, benchmarks | Blog posts, tech articles |

### 3. Execute Web Searches

Use WebSearch to find relevant sources:
- Run **parallel searches** for different aspects of the question
- Target authoritative domains when possible
- Note publication dates for currency

**Depth configuration:**

| Depth | Searches | Fetches | Use When |
|-------|----------|---------|----------|
| Quick | 2-3 | 2-3 | Overview, quick answer, known topic |
| Thorough | 4-6 | 5-8 | Deep dive, comparison, unfamiliar domain |

### 4. Fetch and Extract

Use WebFetch to get detailed content from promising sources:
- Prioritize official documentation
- Extract specific code examples, specifications, or data
- Note the exact URL and relevant sections

### 5. Synthesize Findings

- Connect information across sources
- Identify consensus vs. conflicting information
- Note gaps in available information
- Highlight practical recommendations

**Complete all searches and fetches before synthesizing.**

### 6. Write Research Document

Save to `ai_docs/research/YYYY-MM-DD-topic-name.md`:

```markdown
---
date: [ISO datetime with timezone]
researcher: claude
topic: "[question]"
depth: [quick|thorough]
sources_searched: [N]
sources_cited: [N]
tags: [web-research, relevant-tags]
status: complete
---

# Web Research: [Topic]

## Question
[Original query]

## Summary
[2-3 sentence high-level findings]

## Key Findings

### [Category 1]
- Finding with source reference [1]
- Connection to other findings

### [Category 2]
...

## Options/Approaches
[If comparing solutions]

| Option | Pros | Cons | Best For |
|--------|------|------|----------|
| Option A | ... | ... | ... |
| Option B | ... | ... | ... |

## Recommendations
[Based on findings, practical guidance]

## Sources
1. [Title](URL) — What this source provided
2. [Title](URL) — What this source provided
...

## Open Questions
[Areas needing further investigation, if any]
```

### 7. Present and Iterate

- Summarize key findings
- Highlight actionable recommendations
- Offer to answer follow-ups

**For follow-up questions:**
- Append to same document
- Add `## Follow-up: [topic]` section
- Update frontmatter with `last_updated` field

---

## Search Tips by Topic

### Standards and Specifications
```
Search: "[topic] specification official"
Search: "[topic] RFC" OR "[topic] W3C"
Search: "site:schema.org [topic]"
```

### Libraries and Packages
```
Search: "[topic] library [language]"
Search: "[topic] npm package" OR "[topic] pypi"
Search: "site:github.com [topic] stars:>100"
```

### API Documentation
```
Search: "[service] API documentation"
Search: "[service] developer docs"
Search: "[service] API examples [language]"
```

### Comparisons
```
Search: "[option A] vs [option B]"
Search: "[topic] comparison [year]"
Search: "[topic] benchmark performance"
```

---

## Handling Limited Results

If searches find little relevant information:

```markdown
## Web Research: [Topic]

### Searches Performed
- Query 1: [search terms] — [N] results
- Query 2: [search terms] — [N] results

### Result
Limited information available on this topic.

### What Was Found
- [Any partial findings]

### Possible Reasons
- Topic may be too new
- May use different terminology
- May be niche/specialized domain

### Suggested Next Steps
- Try alternative search terms: [suggestions]
- Check specific domains: [suggestions]
- Clarify what aspect you need
```

---

## What NOT to Do

- **Don't synthesize before completing all searches** — gather everything first
- **Don't skip source attribution** — every claim needs a linked source
- **Don't include outdated info without noting dates** — currency matters
- **Don't present speculation as fact** — distinguish findings from inference
- **Don't ignore conflicting information** — note disagreements between sources
- **Don't over-rely on a single source** — cross-reference when possible
- **Don't include URLs without context** — describe what each source provides

---

## Example Research Flow

**Question:** "How would I extract recipe information from websites? What options are available?"

1. **Interpret:** User wants to know about recipe data extraction — standards, libraries, approaches
2. **Strategy:** Search for standards (schema.org), libraries, and best practices
3. **Searches:**
   - "recipe schema structured data"
   - "recipe extraction library python"
   - "web scraping recipe data best practices"
4. **Fetch:** schema.org Recipe spec, top library docs, tutorial articles
5. **Synthesize:** Connect Schema.org standard with extraction libraries
6. **Write:** Document with options comparison, recommendations
7. **Present:** Summary with actionable next steps
