# Claude Code Skills

Skills are reusable behaviors that Claude auto-discovers and invokes when appropriate contexts are encountered. Unlike agents (which you explicitly invoke) or slash commands (which you trigger), skills activate automatically when their trigger conditions are met.

## Available Skills

### 1. write-to-doc
**Activates when:** Creating or updating formal documentation

**Purpose:** Ensures consistent structure, appropriate detail, and proper file locations for all project documentation.

**Handles:**
- Feature specifications
- Architectural decision records (ADRs)
- Review findings
- Implementation plans
- Interface definitions
- Data models
- Knowledge base articles

**Benefits:**
- Consistent documentation structure
- Proper use of templates
- Correct file locations
- Appropriate detail level
- Clear formatting and style

**Location:** `.claude/skills/write-to-doc/`

---

### 2. generate-mermaid
**Activates when:** Creating visual diagrams

**Purpose:** Ensures proper Mermaid syntax and diagram clarity for all visual documentation.

**Handles:**
- Flowcharts (process flows, decision trees)
- Sequence diagrams (API interactions, component communication)
- ERDs (database schemas, data models)
- State diagrams (state machines, lifecycle flows)
- User journeys (UX flows, emotional journeys)
- Class diagrams (OOP structure, relationships)

**Benefits:**
- Correct Mermaid syntax
- Appropriate diagram type selection
- Clear, readable layouts
- Consistent styling
- Proper file organization

**Location:** `.claude/skills/generate-mermaid/`

---

### 3. run-tests
**Activates when:** Running tests or analyzing test results

**Purpose:** Ensures tests are run appropriately and results are interpreted correctly.

**Handles:**
- Running appropriate test levels (unit, integration, E2E)
- Interpreting test output
- Analyzing failures
- Understanding coverage reports
- Debugging test issues
- Test maintenance

**Benefits:**
- Run right tests at right time
- Correctly interpret results
- Systematic failure investigation
- Proper coverage analysis
- Fix tests effectively

**Location:** `.claude/skills/run-tests/`

---

### 4. structured-review
**Activates when:** Conducting any type of review

**Purpose:** Ensures systematic analysis and actionable findings with consistent format.

**Handles:**
- Code reviews
- Specification reviews
- Architecture reviews
- Security reviews
- UX reviews
- Performance reviews

**Benefits:**
- Consistent review format
- Systematic methodology
- Actionable findings
- Balanced feedback
- Appropriate severity levels
- Clear recommendations

**Location:** `.claude/skills/structured-review/`

---

## How Skills Work

### Auto-Discovery
Claude Code automatically discovers skills in `.claude/skills/*/SKILL.md` files.

### Activation
Skills activate when their trigger conditions are met. The skill's `description` in the frontmatter specifies when it should activate.

### Behavior
Once activated, the skill provides specialized instructions that guide Claude's behavior for that specific context.

### Transparency
Skills work transparently - you don't need to explicitly invoke them. They automatically enhance Claude's capabilities in relevant contexts.

## Skill Structure

Each skill follows this directory structure:

```
.claude/skills/[skill-name]/
├── SKILL.md              # Main skill definition (required)
├── scripts/              # Executable code (optional)
├── references/           # Documentation, schemas (optional)
└── assets/               # Templates, resources (optional)
```

### SKILL.md Format

```markdown
---
name: skill-name
description: Specific trigger conditions when this skill should activate
---

# Skill Name

[Detailed instructions that Claude follows when skill is active]

## When This Skill Activates
[Specific scenarios]

## Principles
[Core principles to follow]

## Workflow
[Step-by-step process]

## Examples
[Concrete examples]

## Constraints
[Limitations and boundaries]
```

## When Each Skill Activates

### write-to-doc
```
✓ Creating feature spec
✓ Writing ADR
✓ Documenting API
✓ Writing review report
✗ Writing code comments
✗ Casual conversation
```

### generate-mermaid
```
✓ Creating flowchart
✓ Documenting architecture
✓ Designing database schema
✓ Mapping user journey
✗ Writing prose
✗ Simple explanations
```

### run-tests
```
✓ After code changes
✓ Investigating test failure
✓ Checking coverage
✓ Before committing
✗ Just reading code
✗ Planning features
```

### structured-review
```
✓ Reviewing code
✓ Evaluating spec
✓ Assessing architecture
✓ Conducting security audit
✗ Implementing features
✗ Writing documentation
```

## Creating Custom Skills

### 1. Identify Reusable Behavior
Look for tasks that:
- Occur frequently
- Have established patterns
- Benefit from consistency
- Require specialized knowledge

### 2. Create Skill Directory
```bash
mkdir -p .claude/skills/[skill-name]/{scripts,references,assets}
```

### 3. Write SKILL.md
```markdown
---
name: your-skill-name
description: Be very specific about when this activates. Example: "Automatically invoked when deploying applications or managing infrastructure"
---

# Your Skill Name

Clear instructions for the behavior.

## When This Skill Activates
[Specific trigger scenarios]

## Workflow
[Step-by-step process]

## Examples
[Concrete examples]
```

### 4. Add Supporting Files (Optional)
- `scripts/` - Automation scripts
- `references/` - Documentation, schemas, patterns
- `assets/` - Templates, examples

### 5. Test the Skill
Trigger the skill's activation conditions and verify it enhances Claude's behavior appropriately.

## Skill Best Practices

### Specific Triggers
**Good:** "Automatically invoked when writing feature specifications or API documentation"
**Bad:** "Invoked when writing stuff"

### Clear Instructions
- Be explicit about what to do
- Provide step-by-step workflows
- Include examples
- Note constraints

### Focused Scope
Each skill should handle one clear area of responsibility:
- write-to-doc = documentation
- generate-mermaid = diagrams
- run-tests = testing
- structured-review = reviews

### Discoverable
Skills are automatically discovered by Claude Code, so make the description clear about when they activate.

## Skill vs Agent vs Command

### Skill
- **Activation:** Automatic when context matches
- **Purpose:** Enhance behavior in specific contexts
- **Example:** write-to-doc activates when writing docs

### Agent
- **Activation:** Explicit invocation
- **Purpose:** Specialized worker for complex tasks
- **Example:** "Use the ideation agent to explore options"

### Command
- **Activation:** User types slash command
- **Purpose:** Entry point to workflows
- **Example:** `/spec feature-name`

## Example Workflows with Skills

### Writing a Feature Spec
```
User: /spec user authentication system
  ↓
spec-writer agent invoked
  ↓
write-to-doc skill auto-activates
  ↓
Spec written with proper structure, template, location
```

### Creating a Diagram
```
User: /diagram sequence authentication flow
  ↓
mermaid-diagrammer agent invoked
  ↓
generate-mermaid skill auto-activates
  ↓
Diagram created with proper syntax, layout, location
```

### Running Tests
```
User: Run tests for the auth module
  ↓
run-tests skill auto-activates
  ↓
Tests run, results interpreted, issues identified
```

### Conducting Review
```
User: /review implementation auth module
  ↓
review-orchestrator invokes reviewers
  ↓
structured-review skill auto-activates
  ↓
Review conducted systematically with consistent format
```

## Tips

### Let Skills Work
Don't micromanage - trust skills to handle their domains:
- Don't specify template locations (write-to-doc handles it)
- Don't dictate Mermaid syntax (generate-mermaid handles it)
- Don't manually structure reviews (structured-review handles it)

### Combine with Agents
Skills enhance agents' work:
- spec-writer agent + write-to-doc skill = great specs
- mermaid-diagrammer agent + generate-mermaid skill = clear diagrams
- review agents + structured-review skill = thorough reviews

### Iterate and Improve
Skills can evolve:
- Add new sections to templates
- Refine workflows
- Add more examples
- Update based on usage

## Future Skills to Consider

Skills to potentially add as needs emerge:

- **deploy** - Deployment and infrastructure management
- **migrate** - Database and data migration procedures
- **optimize** - Performance optimization approaches
- **refactor** - Code refactoring patterns
- **debug-systematic** - Systematic debugging methodology
- **api-design** - API design principles and patterns

Add skills when you identify frequently repeated behaviors that benefit from consistency.

## References

- Claude Code Skills documentation
- Individual skill files for detailed instructions
- Project templates in `docs/` and `ai_docs/`

## Notes

- Skills are project-specific and stored in `.claude/skills/`
- Skills activate automatically based on context
- Skills can reference each other
- Skills can load supporting files from their directory
- Skills work alongside agents and commands seamlessly
