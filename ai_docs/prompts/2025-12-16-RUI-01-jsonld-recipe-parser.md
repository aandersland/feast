---
date: 2025-12-16
status: draft
parent_roadmap: ai_docs/roadmaps/2025-12-16-recipe-url-import.md
chunk: 1
chunk_name: JSON-LD Recipe Parser Module
target_command: create_plan
---

# JSON-LD Recipe Parser Module

## Inherited Context

> **Feature**: Recipe URL Import
> **Roadmap**: ai_docs/roadmaps/2025-12-16-recipe-url-import.md
> **Chunk**: 1 of 4
> **Depends on**: None
> **Produces**: Parser module used by Chunk 4 (Import Command) to extract recipes from HTML

## Goal

Create a Rust module that extracts Schema.org Recipe data from HTML containing JSON-LD, parses it into a structured format compatible with `RecipeInput`, and handles the variety of real-world implementations.

## Background

~75% of recipe websites embed JSON-LD structured data following the Schema.org Recipe specification. This data appears in `<script type="application/ld+json">` tags and provides reliable, machine-readable recipe information.

The app already has a `RecipeInput` struct that accepts recipe data for creation. This parser produces a similar structure that can be passed to the existing `create_recipe` function.

**Constraint**: No external recipe-specific crates. Build parser using `scraper` (HTML parsing) and `serde_json` (JSON parsing).

## Requirements

### Must Have

- Extract `<script type="application/ld+json">` content from HTML
- Find Schema.org Recipe objects (handle `@type: "Recipe"` and `@type: ["Recipe", ...]`)
- Handle `@graph` structures (array of objects, find the Recipe within)
- Parse required fields: `name`, `recipeIngredient`, `recipeInstructions`
- Parse optional fields: `description`, `image`, `author`, `prepTime`, `cookTime`, `totalTime`, `recipeYield`, `recipeCategory`, `recipeCuisine`
- Parse ISO 8601 duration strings (PT30M, PT1H15M) into minutes
- Parse ingredient strings into quantity, unit, and name components
- Parse fractions as decimals (1/2 → 0.5, 1 1/2 → 1.5)
- Parse ranges using first number (3-4 → 3)
- Handle unparseable ingredients as name-only (quantity=0, unit="")
- Handle `recipeInstructions` as both `string[]` and `HowToStep[]` formats
- Extract first/primary image URL with URL validation
- Return specific error types for different failure modes
- Unit tests with HTML fixture files

### Out of Scope (handled by other chunks)

- HTTP fetching (Chunk 2)
- Database storage (Chunk 3)
- Tauri command integration (Chunk 4)
- HTML scraping fallback for sites without JSON-LD

## Affected Areas

- **Systems/Components**:
  - New `src-tauri/src/parser/` module (or `src-tauri/src/recipe_parser.rs`)
  - New test fixtures in `src-tauri/tests/fixtures/` or similar
- **Data**: Produces `ParsedRecipe` struct with fields matching `RecipeInput`

## Edge Cases

### JSON-LD Structure Variations
- **Single Recipe object**: `{"@type": "Recipe", ...}`
- **Array @type**: `{"@type": ["Recipe", "SomeOtherType"], ...}`
- **@graph wrapper**: `{"@context": "...", "@graph": [{...}, {"@type": "Recipe", ...}]}`
- **Multiple JSON-LD blocks**: Page has several `<script type="application/ld+json">` tags
- **Multiple Recipe objects**: Error — inform user "Multiple recipes found on page, unable to determine which to import"

### Ingredient Parsing
- Standard: "2 cups flour" → quantity: 2.0, unit: "cups", name: "flour"
- Fractions: "1/2 tsp salt" → quantity: 0.5, unit: "tsp", name: "salt"
- Mixed fractions: "1 1/2 cups sugar" → quantity: 1.5, unit: "cups", name: "sugar"
- Ranges: "3-4 cloves garlic" → quantity: 3.0, unit: "", name: "cloves garlic"
- No quantity: "salt to taste" → quantity: 0.0, unit: "", name: "salt to taste"
- Parentheticals: "1 (15 oz) can beans" → best effort parsing

### Instructions Variations
- String array: `["Step 1", "Step 2"]` → extract as-is
- HowToStep array: `[{"@type": "HowToStep", "text": "Step 1"}]` → extract `text` field
- HowToSection: `[{"@type": "HowToSection", "itemListElement": [...]}]` → flatten steps
- Single string (rare): `"Mix ingredients. Bake."` → split on sentence boundaries or keep as single step

### Image Variations
- String URL: `"https://example.com/image.jpg"` → use directly
- Array of URLs: `["url1.jpg", "url2.jpg"]` → use first
- ImageObject: `{"@type": "ImageObject", "url": "..."}` → extract `url`
- Array of ImageObjects: use first object's URL
- Invalid URL format: skip image, don't fail parse
- No image: leave as None

### Duration Parsing
- `PT30M` → 30 minutes
- `PT1H` → 60 minutes
- `PT1H30M` → 90 minutes
- `PT2H15M` → 135 minutes
- Invalid/missing: default to 0

### Error Cases
- No `<script type="application/ld+json">` found → `NoJsonLdFound`
- JSON-LD found but no Recipe type → `NoRecipeFound`
- Multiple Recipe objects found → `MultipleRecipesFound`
- Recipe missing required `name` field → `MalformedRecipe("missing name")`
- Recipe missing `recipeIngredient` → `MalformedRecipe("missing ingredients")`
- JSON parse error → `MalformedRecipe("invalid JSON")`

## Success Criteria

### Automated Verification
- [ ] `cargo test` passes for parser module
- [ ] `cargo clippy` passes with no warnings
- [ ] Tests cover: minimal recipe, full recipe, @graph structure, HowToStep instructions, various ingredient formats, duration parsing, image extraction, all error cases
- [ ] Test fixtures include at least 5 different HTML structures

### Manual Verification
- [ ] Parser correctly handles sample HTML from 3 real recipe sites (saved as fixtures)
- [ ] Error messages are clear and specific

## Open Questions for Planning

- Should `ParsedRecipe` be identical to `RecipeInput` or a separate struct that converts to it?
- What's the best module structure: single file or `parser/mod.rs` with submodules?
- Should ingredient parsing be a separate submodule for testability?

---

**To execute**: `/create_plan ai_docs/prompts/2025-12-16-01-jsonld-recipe-parser.md`
