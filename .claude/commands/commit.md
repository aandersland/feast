---
description: Create git commits with user approval and no Claude attribution
model: opus
---

# Commit Changes

Create git commits for changes made during this session.

## Process

### 1. Review Changes

```bash
git status
git diff
git diff --cached
```

Consider:
- What was accomplished in this session?
- Should changes be one commit or multiple logical commits?
- Which files belong together?

### 2. Plan Commits

- Identify file groupings
- Draft clear, descriptive commit messages
- Use imperative mood ("Add feature" not "Added feature")
- Focus on why, not just what

### 3. Present Plan

```markdown
I plan to create [N] commit(s):

**Commit 1**: [message]
- `file1.ts`
- `file2.ts`

**Commit 2**: [message]
- `file3.ts`

Shall I proceed?
```

If unsure about groupings, present options:
```markdown
These changes could be grouped as:

**Option A** — Single commit:
- [message covering all changes]

**Option B** — Separate commits:
- Commit 1: [focused message]
- Commit 2: [focused message]

Which approach do you prefer?
```

### 4. Execute on Confirmation

```bash
git add [specific files]
git commit -m "[message]"
```

### 5. Show Result

```bash
git log --oneline -n [number of commits created]
```

---

## Commit Message Guidelines

**Format**:
```
<type>: <short summary>

[optional body with more detail]
```

**Types** (optional, adapt to project conventions):
- `feat`: New feature
- `fix`: Bug fix
- `refactor`: Code change that neither fixes nor adds
- `docs`: Documentation only
- `test`: Adding or updating tests
- `chore`: Maintenance tasks

**Good messages**:
- `Add user authentication endpoint`
- `Fix race condition in cache invalidation`
- `Refactor payment processing for clarity`

**Avoid**:
- `Update files`
- `Fix bug`
- `WIP`

---

## What NOT to Do

- **NEVER add co-author information or Claude attribution**
- **NEVER include "Generated with Claude" messages**
- **NEVER add "Co-Authored-By" lines**
- **NEVER use `git add -A` or `git add .`** — always specify files
- **Don't commit without presenting plan first**
- **Don't combine unrelated changes** — keep commits focused

Commits should be authored solely by the user, written as if the user wrote them.
