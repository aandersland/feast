---
date: 2025-12-16
status: draft
feature: Recipe URL Import
feature_id: RUI
chunks: 4
---

# Recipe URL Import

## Vision

Users can paste a recipe URL, and the app fetches, parses, and saves the recipe automatically. The feature uses JSON-LD structured data (Schema.org Recipe format) to extract recipe details reliably. After import, the recipe appears in the recipe list ready for use in meal planning.

## Background

The "Import from URL" button exists in the Recipes page but currently only shows an alert. The research document (`ai_docs/research/2025-12-16-recipe-extraction-standards.md`) confirmed that ~75% of recipe websites embed JSON-LD structured data, making reliable extraction feasible without external dependencies.

**Current state:**
- `ImportRecipe.svelte` component exists with stub implementation
- `recipes` table has `source_url` field but no `instructions` storage
- Recipe CRUD operations are functional via Tauri commands
- Instructions are in the Recipe struct but return empty (placeholder)

**Constraints from user:**
- Write custom JSON-LD parser (no external recipe libraries)
- Only support JSON-LD format (no HTML scraping fallback)
- User-Agent header should only contain app name: "feast"
- All errors must be handled and shown via toast
- Comprehensive test coverage required

## Affected Areas

- **Users/Personas**: Home cooks who want to save recipes from the web
- **Systems/Components**:
  - `src-tauri/src/` — New parser module, HTTP client, import command
  - `src/lib/components/recipes/ImportRecipe.svelte` — Wire to backend
  - `src/lib/stores/recipes.ts` — Add import action
  - Database — New migration for instructions storage
- **Data**: Creates new recipes in SQLite from external URL data

## Testing Strategy

- **Unit tests**: JSON-LD parser with HTML fixture files covering edge cases
- **Unit tests**: HTTP client error handling (mocked responses)
- **Unit tests**: Database operations for new instructions storage
- **Integration tests**: Full import flow with fixture data
- **Frontend tests**: Store actions and component behavior
- **Manual checkpoint**: Test against 3-5 real recipe websites

## Constraints

- No external recipe-specific crates (build parser in-house)
- User-Agent: "feast" only (no version, no additional info)
- JSON-LD only — return clear error if not found
- Must handle: network errors, invalid URL, no JSON-LD, malformed recipe data

---

## Chunks

### 1. JSON-LD Recipe Parser Module

**Purpose**: Create a Rust module that extracts and parses Schema.org Recipe data from HTML

**Depends on**: None

**Produces**:
- `src-tauri/src/parser/` module with `parse_recipe_from_html(html: &str) -> Result<ParsedRecipe, ParseError>`
- `ParsedRecipe` struct matching fields needed for `RecipeInput`
- Test fixtures with various JSON-LD structures

**Key considerations**:
- Use `scraper` crate to find `<script type="application/ld+json">` tags
- Handle both single Recipe and array of `@type` (some sites have `["Recipe", "SomethingElse"]`)
- Parse ISO 8601 duration strings (PT30M, PT1H15M) into minutes
- Handle missing optional fields gracefully (description, image, times, etc.)
- Map `recipeIngredient[]` strings to ingredient list (parsing quantity/unit is stretch goal)
- Map `recipeInstructions` to step text (handle both string[] and HowToStep[])
- Return specific error types: NoJsonLdFound, NoRecipeFound, MalformedRecipe
- Test fixtures should include: minimal recipe, full recipe, nested @graph structure, missing fields

---

### 2. HTTP Client for Recipe Fetching

**Purpose**: Add HTTP client capability to fetch HTML content from recipe URLs

**Depends on**: None (can be developed in parallel with Chunk 1)

**Produces**:
- HTTP fetching function in `src-tauri/src/http/` or similar
- URL validation logic
- Error types for network/HTTP failures

**Key considerations**:
- Add `reqwest` crate with blocking or async support (check Tauri async patterns)
- User-Agent header: exactly "feast" — nothing else
- Validate URL format (must be http/https, proper structure)
- Handle: connection errors, timeouts (reasonable default ~30s), HTTP 4xx/5xx
- Consider redirect handling (most sites redirect http→https)
- Return clear error messages suitable for user display
- Tests should mock HTTP responses (don't hit real URLs in tests)

---

### 3. Instructions Storage Migration

**Purpose**: Add database storage for recipe instructions (currently not persisted)

**Depends on**: None (can be developed in parallel)

**Produces**:
- New migration adding instructions storage
- Updated `db/recipes.rs` to save and load instructions
- Tests verifying instruction persistence

**Key considerations**:
- Current `Recipe` struct has `instructions: Vec<String>` but returns empty
- Options: (a) JSON column in recipes table, (b) separate `recipe_instructions` table
- JSON column is simpler for ordered text; table is better if steps need individual editing
- Recommend JSON column (`instructions TEXT` storing JSON array) for simplicity
- Update `create_recipe`, `update_recipe`, `get_recipe_by_id` to handle instructions
- Ensure existing tests still pass
- Add specific tests for instruction CRUD

---

### 4. Import Command and Frontend Integration

**Purpose**: Wire parser, HTTP client, and database together; connect frontend

**Depends on**: Chunks 1, 2, 3

**Produces**:
- Tauri command `import_recipe_from_url(url: String) -> Result<Recipe, String>`
- Updated `ImportRecipe.svelte` calling the real backend
- Updated `recipes.ts` store with import action
- Toast notifications for success and specific error cases

**Key considerations**:
- Command orchestrates: validate URL → fetch HTML → parse JSON-LD → create recipe
- Error messages should be user-friendly:
  - "Could not connect to website" (network error)
  - "Website did not respond" (timeout)
  - "Could not find recipe data on this page" (no JSON-LD/Recipe)
  - "Recipe data was incomplete or invalid" (parse error)
- On success: return full Recipe, frontend shows success toast and closes modal
- On error: frontend shows error toast, keeps modal open for retry
- Check if `source_url` already exists (duplicate prevention) — optional, decide approach
- Frontend tests: mock Tauri invoke, verify loading states and error handling
- Integration test: full flow with fixture HTML served or mocked

---

## Next Steps

- [x] `/create_prompt_v2 ... 1` → `ai_docs/prompts/2025-12-16-RUI-01-jsonld-recipe-parser.md`
- [x] `/create_prompt_v2 ... 2` → `ai_docs/prompts/2025-12-16-RUI-02-http-client.md`
- [x] `/create_prompt_v2 ... 3` → `ai_docs/prompts/2025-12-16-RUI-03-instructions-storage.md`
- [x] `/create_prompt_v2 ... 4` → `ai_docs/prompts/2025-12-16-RUI-04-import-command-integration.md`

**To plan:** `/create_plan ai_docs/prompts/2025-12-16-RUI-01-jsonld-recipe-parser.md`
