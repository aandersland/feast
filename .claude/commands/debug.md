---
description: Debug issues by investigating logs, database state, and git history
model: opus
---

# Debug

Investigate issues during manual testing or implementation. Read-only exploration of logs, database, and git state — no file editing.

## Quick Start

```
I'll help debug the issue.

Describe what's happening:
- What were you trying to do?
- What went wrong?
- Any error messages?

I'll investigate logs, database, and recent changes.
```

If a plan/ticket file is provided, read it first to understand context.

---

## Investigation Approach

### Step 1: Understand the Problem

After user describes the issue:

1. **Read provided context** (plan or ticket)
   - What phase/step are they on?
   - Expected vs actual behavior

2. **Quick state check**
   ```bash
   git status
   git log --oneline -5
   ```

### Step 2: Parallel Investigation

Execute focused investigation tasks:

**Task 1 — Logs**
- Find recent logs in application log directory
- Search for errors around the problem timeframe
- Note stack traces, repeated errors

**Task 2 — Database** (if applicable)
- Connect to database
- Check schema and recent data
- Look for stuck states or anomalies

**Task 3 — Git/Files**
- Check uncommitted changes
- Review recent commits
- Verify expected files exist

### Step 3: Present Findings

```markdown
## Debug Report

### Problem Summary
[Clear statement based on evidence]

### Evidence

**Logs**:
- [Error with timestamp]
- [Pattern observed]

**Database**:
```sql
[Relevant query and result]
```

**Git/Files**:
- [Recent changes]
- [File state]

### Likely Cause
[Explanation based on evidence — not speculation]

### Suggested Fix
[Specific action to try]

### Outside My Reach
[Things user needs to check: browser console, external services, etc.]
```

---

## Quick Reference

### Finding Logs

```bash
# List log files (adapt path to project)
ls -lt ~/path/to/app/logs/

# Search for errors
grep -i "error\|exception\|failed" [logfile] | tail -50

# Follow recent logs
tail -100 [logfile]
```

### Database Inspection

```bash
# SQLite (adapt path to project)
sqlite3 ~/path/to/app/database.db

# Common inspection queries
.tables
.schema [table]
SELECT * FROM [table] ORDER BY created_at DESC LIMIT 10;
SELECT * FROM [table] WHERE status = 'error';
```

### Git State

```bash
# Current state
git status
git branch -v

# Recent history
git log --oneline -10
git log --oneline --since="1 hour ago"

# What changed
git diff
git diff --cached
git diff HEAD~3..HEAD --stat
```

### File Verification

```bash
# Check file exists and contents
ls -la [expected_file]
head -50 [expected_file]

# Find files modified recently
find . -type f -mmin -30 -not -path '*/node_modules/*'
```

---

## Common Debug Patterns

| Symptom | First Check |
|---------|-------------|
| "It was working yesterday" | `git log --since="yesterday"`, check for config changes |
| "Works in tests, fails in app" | Environment config differences, missing registrations |
| "Intermittent failures" | Race conditions, connection timeouts in logs |
| "Silent failure" | Check error handling, look for swallowed exceptions |
| "Wrong data displayed" | Database state, recent data migrations |

---

## What NOT to Do

- **Don't edit files** — this is investigation only
- **Don't guess root cause without evidence** — show logs/data supporting conclusion
- **Don't skip git status** — uncommitted changes are often the cause
- **Don't assume logs exist** — verify paths before searching
- **Don't make changes to "test a theory"** — report findings, let user decide
- **Don't ignore "Outside My Reach"** — explicitly state what you can't check
