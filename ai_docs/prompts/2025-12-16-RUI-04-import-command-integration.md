---
date: 2025-12-16
status: draft
parent_roadmap: ai_docs/roadmaps/2025-12-16-recipe-url-import.md
chunk: 4
chunk_name: Import Command and Frontend Integration
target_command: create_plan
---

# Import Command and Frontend Integration

## Inherited Context

> **Feature**: Recipe URL Import
> **Roadmap**: ai_docs/roadmaps/2025-12-16-recipe-url-import.md
> **Chunk**: 4 of 4
> **Depends on**: Chunks 1 (Parser), 2 (HTTP Client), 3 (Instructions Storage)
> **Produces**: Complete end-to-end recipe import feature

## Goal

Wire the parser, HTTP client, and database together into a Tauri command, then connect the frontend `ImportRecipe` component to call it with proper loading states, error handling, and user feedback.

## Background

The `ImportRecipe.svelte` component exists with a stub implementation that shows an alert. The backend pieces (parser, HTTP client, instructions storage) are ready from previous chunks. This chunk orchestrates them into a working feature.

**Current frontend state:**
- `ImportRecipe.svelte` has URL input and submit button
- `handleImport` function currently just shows an alert
- Modal opens from "Import from URL" button on Recipes page
- Toast system exists via `ToastContainer.svelte`

## Requirements

### Must Have

**Backend:**
- New Tauri command: `import_recipe_from_url(url: String) -> Result<Recipe, String>`
- Command orchestrates: validate URL → check for duplicate → fetch HTML → parse JSON-LD → create recipe
- Check if `source_url` already exists in database before fetching
- Return user-friendly error messages for all failure modes
- Register command in Tauri app builder

**Frontend:**
- Update `ImportRecipe.svelte` to call `import_recipe_from_url` via Tauri invoke
- Add cancel button to abort in-progress import
- Show spinner with "Importing recipe..." during fetch
- On success: close modal, show success toast, refresh recipe list
- On error: show error toast, keep modal open for retry
- Update `recipes.ts` store with import action if needed

**Error Messages (user-friendly):**
- Duplicate URL: "A recipe from this URL has already been imported"
- Invalid URL: "Please enter a valid website URL"
- Network error: "Could not connect to the website"
- Timeout: "The website took too long to respond"
- No JSON-LD: "Could not find recipe data on this page"
- Multiple recipes: "This page contains multiple recipes. Please try a more specific URL"
- Parse error: "The recipe data on this page could not be read"

### Out of Scope (handled by other chunks or future work)

- Parser implementation (Chunk 1)
- HTTP client implementation (Chunk 2)
- Instructions storage (Chunk 3)
- Preview before save
- Editing imported recipe before save
- Retry logic (user can manually retry)

## Affected Areas

- **Users/Personas**: Home cooks importing recipes from the web
- **Systems/Components**:
  - New: `src-tauri/src/commands/import.rs` (or add to `recipes.rs`)
  - Update: `src-tauri/src/main.rs` or `lib.rs` — register command
  - Update: `src/lib/components/recipes/ImportRecipe.svelte`
  - Update: `src/lib/stores/recipes.ts` — add import action
  - Update: `src/lib/tauri/` — add invoke wrapper if pattern exists
- **Data**: Creates new recipe in database from external URL

## Edge Cases

### Duplicate Detection
- Exact URL match: block import
- Same URL with different query params (e.g., `?utm_source=...`): treat as different (don't normalize)
- Same URL with/without trailing slash: treat as different (exact match)

### Cancel Behavior
- User clicks cancel during fetch: abort request, close modal, no toast
- User clicks cancel during parse (unlikely to be slow): same behavior
- Request completes right as user cancels: cancel takes precedence, don't save

### Concurrent Imports
- User opens modal, starts import, closes modal, opens again: previous request should be cancelled
- Prevent double-submit: disable button while loading

### Empty/Whitespace URL
- Trim whitespace before validation
- Empty after trim: show validation error inline, don't call backend

### Recipe List Refresh
- After successful import, recipe list should show the new recipe
- Use existing `recipeStore.load()` or add the new recipe directly to store

### Modal State
- Success: close modal immediately, then show toast
- Error: show toast, keep modal open with URL still filled in
- Cancel: close modal, no toast

## Success Criteria

### Automated Verification
- [ ] `cargo test` passes for import command
- [ ] `cargo clippy` passes with no warnings
- [ ] `pnpm check` passes (TypeScript/Svelte)
- [ ] `pnpm test` passes for frontend
- [ ] Integration test: mock HTML → full import flow → recipe in DB
- [ ] Frontend test: mock Tauri invoke, verify loading/error states

### Manual Verification
- [ ] Import recipe from AllRecipes.com (or similar major site)
- [ ] Import recipe from a food blog with JSON-LD
- [ ] Try importing same URL twice — see duplicate error
- [ ] Try importing non-recipe URL — see appropriate error
- [ ] Cancel mid-import — verify no recipe saved
- [ ] Verify imported recipe appears in list and can be viewed

## Open Questions for Planning

- Should the import command live in `commands/import.rs` or be added to `commands/recipes.rs`?
- Does a Tauri invoke wrapper pattern exist in `src/lib/tauri/` to follow?
- How to properly implement request cancellation in Tauri async commands?

---

**To execute**: `/create_plan ai_docs/prompts/2025-12-16-04-import-command-integration.md`
