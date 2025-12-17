---
date: 2025-12-16
status: draft
parent_roadmap: ai_docs/roadmaps/2025-12-16-recipe-url-import.md
chunk: 3
chunk_name: Instructions Storage Migration
target_command: create_plan
---

# Instructions Storage Migration

## Inherited Context

> **Feature**: Recipe URL Import
> **Roadmap**: ai_docs/roadmaps/2025-12-16-recipe-url-import.md
> **Chunk**: 3 of 4
> **Depends on**: None
> **Produces**: Database capability to persist recipe instructions, used by Chunk 4 (Import Command)

## Goal

Add database storage for recipe instructions so imported recipes can save their step-by-step instructions. Currently the `Recipe` struct has an `instructions: Vec<String>` field but it always returns empty.

## Background

The existing schema has a `recipes` table but no storage for instructions. The `Recipe` struct in `db/recipes.rs` includes `instructions: Vec<String>` but `get_recipe_by_id` returns an empty vector (placeholder). This chunk adds the actual persistence layer.

**Approach**: Add a JSON column (`instructions TEXT`) to the recipes table storing a JSON array of strings. This is simpler than a separate table for ordered text steps.

## Requirements

### Must Have

- New migration adding `instructions TEXT` column to recipes table (default to empty JSON array `'[]'`)
- Update `create_recipe` to save instructions as JSON
- Update `update_recipe` to save instructions as JSON
- Update `get_recipe_by_id` to load and parse instructions from JSON
- Handle JSON parse errors gracefully (return empty array if malformed)
- Existing recipes continue to work (empty instructions)
- Unit tests for instruction CRUD operations

### Out of Scope (handled by other chunks)

- JSON-LD parsing (Chunk 1)
- HTTP fetching (Chunk 2)
- Import command (Chunk 4)
- UI for editing instructions manually (future work)
- RecipeForm updates for manual instruction entry (future work)

## Affected Areas

- **Systems/Components**:
  - New migration file: `src-tauri/migrations/YYYYMMDDHHMMSS_add_instructions.sql`
  - `src-tauri/src/db/recipes.rs` — update CRUD operations
- **Data**: Adds `instructions` column to `recipes` table, stores JSON array of strings

## Edge Cases

### Migration
- Existing recipes get default value `'[]'` (empty JSON array)
- Migration must be idempotent-safe (handle re-runs gracefully)

### JSON Storage
- Empty instructions: store as `'[]'`
- Instructions with special characters: JSON encoding handles escaping
- Very long instructions: no explicit limit (SQLite TEXT can handle large content)

### JSON Parsing on Read
- Valid JSON array: `["Step 1", "Step 2"]` → `vec!["Step 1", "Step 2"]`
- Empty string in DB: treat as `[]`
- NULL in DB: treat as `[]`
- Malformed JSON: log warning, return `[]` (don't fail the whole recipe fetch)
- Non-array JSON (e.g., `{}`): treat as `[]`

### RecipeInput Compatibility
- `RecipeInput` already has `instructions: Vec<String>` field
- Existing callers passing empty vec should continue to work
- No breaking changes to the struct

## Success Criteria

### Automated Verification
- [ ] `cargo test` passes for recipes module
- [ ] `cargo clippy` passes with no warnings
- [ ] Migration applies cleanly to existing database
- [ ] Test: create recipe with instructions, fetch it, verify instructions returned
- [ ] Test: update recipe instructions, verify changes persisted
- [ ] Test: existing recipe (no instructions) returns empty vec
- [ ] Test: malformed JSON in DB returns empty vec (no panic)

### Manual Verification
- [ ] Run migration against dev database with existing recipes
- [ ] Verify existing recipes still load correctly
- [ ] Create a recipe via existing UI, confirm it saves (with empty instructions)

## Open Questions for Planning

- What timestamp format should be used for the migration filename? (Check existing migrations)
- Should sqlx offline mode be updated after adding the migration?

---

**To execute**: `/create_plan ai_docs/prompts/2025-12-16-03-instructions-storage.md`
