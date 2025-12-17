# Import Command and Frontend Integration Implementation Plan

## Overview

Wire the parser, HTTP client, and database together into a Tauri command (`import_recipe_from_url`), then connect the frontend `ImportRecipe` component to call it with proper loading states, error handling, and user feedback. This completes the Recipe URL Import feature.

## Current State

The backend pieces exist but are not connected:
- Parser: `parse_recipe_from_html()` at `src-tauri/src/parser/mod.rs:51`
- HTTP client: `fetch_url()` at `src-tauri/src/http/mod.rs:124`
- Recipe creation: `create_recipe()` at `src-tauri/src/db/recipes.rs:199`
- `RecipeRow` has `source_url` field at `src-tauri/src/db/recipes.rs:20`

Frontend has stub implementation:
- `ImportRecipe.svelte:11-19` uses setTimeout simulation
- `Recipes.svelte:118-121` shows an alert

**Key Discoveries**:
- Commands follow thin wrapper pattern delegating to db layer (`src-tauri/src/commands/recipes.rs:7-10`)
- Error handling converts `AppError` to JSON string via `From` impl (`src-tauri/src/error/mod.rs:52-56`)
- Frontend uses typed `invoke()` wrappers (`src/lib/tauri/commands.ts`)
- Toast notifications via `toastStore.success()`/`toastStore.error()` (`src/lib/stores/toast.ts:27-33`)
- `ParseError` variants map to user-friendly messages (`src-tauri/src/parser/mod.rs:10-23`)
- `FetchError` variants map to user-friendly messages (`src-tauri/src/http/mod.rs:21-58`)

## Desired End State

1. User pastes URL in ImportRecipe modal and clicks "Import Recipe"
2. Backend validates URL, checks for duplicates, fetches HTML, parses JSON-LD, creates recipe
3. On success: modal closes, success toast appears, recipe list refreshes with new recipe
4. On error: error toast with user-friendly message, modal stays open for retry
5. Cancel button aborts in-progress import

Verification: Import a recipe from AllRecipes.com or similar site with JSON-LD markup.

## What We're NOT Doing

- Preview before save
- Editing imported recipe before save
- Retry logic (user can manually retry)
- URL normalization for duplicate detection (exact match only)
- Request cancellation in Rust (Tauri doesn't support this cleanly)

## Integration Map

| Type | Location | Notes |
|------|----------|-------|
| Entry point | `src-tauri/src/commands/recipes.rs` | New `import_recipe_from_url` command |
| Registration | `src-tauri/src/commands/mod.rs:23` | Add to `pub use recipes::{...}` |
| Registration | `src-tauri/src/lib.rs:17` | Import new command |
| Registration | `src-tauri/src/lib.rs:61` | Add to `generate_handler![]` |
| Exports | `src/lib/tauri/commands.ts` | Add `importRecipeFromUrl` wrapper |
| Exports | `src/lib/tauri/index.ts:11` | Re-export new function |
| Consumers | `src/lib/components/recipes/ImportRecipe.svelte` | Replace stub with real call |
| Consumers | `src/lib/components/Recipes.svelte:118-121` | Update `handleImport` |
| Events | N/A | None required |

## Implementation Approach

Three phases in dependency order:
1. Backend command that orchestrates all pieces and returns user-friendly errors
2. Frontend TypeScript wrapper following existing patterns
3. Component wiring with loading/error states and toast feedback

---

## Phase 1: Backend Import Command

### Goal
Create `import_recipe_from_url` Tauri command that orchestrates validation, duplicate check, fetch, parse, and create.

### Integration Points

**Depends on**: Existing parser (`src-tauri/src/parser`), HTTP client (`src-tauri/src/http`), recipe DB (`src-tauri/src/db/recipes.rs`)
**Produces for next phase**: Registered Tauri command callable via `invoke("import_recipe_from_url", { url })`

**Wiring required**:
- [x] Add `import_recipe_from_url` function in `src-tauri/src/commands/recipes.rs`
- [x] Export from `src-tauri/src/commands/mod.rs:23`
- [x] Import in `src-tauri/src/lib.rs:17`
- [x] Register in handler at `src-tauri/src/lib.rs:61`

### Changes

#### Import Command Function

**File**: `src-tauri/src/commands/recipes.rs`

**Change**: Add new command after existing recipe commands (after line 36)

```rust
use crate::http::{self, FetchError};
use crate::parser::{self, ParseError, ParsedRecipe};

/// Import a recipe from a URL
#[command]
pub async fn import_recipe_from_url(url: String) -> Result<Recipe, String> {
    // Validate URL format
    let url = url.trim();
    if url.is_empty() {
        return Err("Please enter a valid website URL".to_string());
    }

    // Check for duplicate source_url
    if recipe_exists_by_source_url(url).await? {
        return Err("A recipe from this URL has already been imported".to_string());
    }

    // Fetch HTML
    let html = http::fetch_url(url).await.map_err(|e| match e {
        FetchError::InvalidUrl(_) => "Please enter a valid website URL".to_string(),
        FetchError::InvalidUrlScheme => "Please enter a valid website URL".to_string(),
        FetchError::ConnectionFailed(_) => "Could not connect to the website".to_string(),
        FetchError::Timeout(_) => "The website took too long to respond".to_string(),
        FetchError::TooManyRedirects(_) => "Could not connect to the website".to_string(),
        FetchError::HttpError { status, .. } => {
            format!("The website returned an error (HTTP {})", status)
        }
        FetchError::InvalidContentType(_) => {
            "This URL does not appear to be a recipe page".to_string()
        }
        FetchError::ResponseTooLarge(_) => "The page is too large to process".to_string(),
        FetchError::ReadError(_) => "Could not read the website response".to_string(),
    })?;

    // Parse JSON-LD
    let parsed = parser::parse_recipe_from_html(&html).map_err(|e| match e {
        ParseError::NoJsonLdFound => "Could not find recipe data on this page".to_string(),
        ParseError::NoRecipeFound => "Could not find recipe data on this page".to_string(),
        ParseError::MultipleRecipesFound => {
            "This page contains multiple recipes. Please try a more specific URL".to_string()
        }
        ParseError::MalformedRecipe(msg) => {
            format!("The recipe data on this page could not be read: {}", msg)
        }
    })?;

    // Convert ParsedRecipe to RecipeInput
    let input = parsed_to_input(parsed, url);

    // Create recipe
    recipes::create_recipe(input).await.map_err(|e| e.into())
}

/// Check if a recipe with this source_url already exists
async fn recipe_exists_by_source_url(url: &str) -> Result<bool, String> {
    use crate::db::pool::get_db_pool;

    let pool = get_db_pool();
    let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM recipes WHERE source_url = ?")
        .bind(url)
        .fetch_one(pool)
        .await
        .map_err(|e| format!("Database error: {}", e))?;

    Ok(count > 0)
}

/// Convert ParsedRecipe to RecipeInput
fn parsed_to_input(parsed: ParsedRecipe, source_url: &str) -> RecipeInput {
    RecipeInput {
        name: parsed.name,
        description: parsed.description,
        prep_time: parsed.prep_time,
        cook_time: parsed.cook_time,
        servings: parsed.servings,
        image_path: parsed.image_url,
        source_url: Some(source_url.to_string()),
        notes: None,
        tags: vec![],
        ingredients: parsed
            .ingredients
            .into_iter()
            .map(|i| IngredientInput {
                name: i.name,
                quantity: i.quantity,
                unit: i.unit,
                category: None,
                notes: None,
            })
            .collect(),
        instructions: parsed.instructions,
    }
}
```

#### Update Command Imports

**File**: `src-tauri/src/commands/recipes.rs`

**Change**: Add imports at top of file (after line 4)

```rust
use crate::db::recipes::{self, IngredientInput, Recipe, RecipeInput, RecipeRow};
use crate::http::{self, FetchError};
use crate::parser::{self, ParseError, ParsedRecipe};
use tauri::command;
```

#### Export from mod.rs

**File**: `src-tauri/src/commands/mod.rs`

**Change**: Update line 23 to include new export

```rust
pub use recipes::{create_recipe, delete_recipe, get_recipe, get_recipes, import_recipe_from_url, update_recipe};
```

#### Import in lib.rs

**File**: `src-tauri/src/lib.rs`

**Change**: Update lines 12-21 to include import

```rust
use commands::{
    add_quick_list_item, add_quick_list_to_shopping, add_shopping_item, create_ingredient,
    create_item, create_manual_item, create_meal_plan, create_quick_list, create_recipe,
    create_shopping_list, delete_item, delete_manual_item, delete_meal_plan, delete_quick_list,
    delete_recipe, delete_shopping_list, get_aggregated_shopping_list, get_ingredients, get_items,
    get_manual_items, get_meal_plans, get_or_create_ingredient, get_quick_lists, get_recipe,
    get_recipes, get_shopping_lists, greet, import_recipe_from_url, move_shopping_item,
    remove_quick_list_item, restore_shopping_item, soft_delete_shopping_item, update_manual_item,
    update_meal_plan, update_quick_list, update_quick_list_item, update_recipe,
    update_shopping_item,
};
```

#### Register in Handler

**File**: `src-tauri/src/lib.rs`

**Change**: Add to generate_handler around line 61 (after delete_recipe)

```rust
            // Recipe commands
            get_recipes,
            get_recipe,
            create_recipe,
            update_recipe,
            delete_recipe,
            import_recipe_from_url,
```

### Success Criteria

#### Automated Verification
- [x] Tests pass: `pnpm test:rust`
- [x] Lint passes: `cd src-tauri && cargo clippy`
- [x] Build succeeds: `cd src-tauri && cargo check`

#### Integration Verification
- [x] Command registered: `cargo check` compiles with new handler
- [x] Import chain works: parser → http → db all accessible from command

#### Manual Verification
- [ ] N/A (no UI yet)

**Checkpoint**: Verify `cargo check` passes before proceeding to Phase 2.

---

## Phase 2: Frontend Tauri Wrapper

### Goal
Add TypeScript invoke wrapper for `import_recipe_from_url` command.

### Integration Points

**Consumes from Phase 1**: Registered Tauri command `import_recipe_from_url`
**Produces for next phase**: `importRecipeFromUrl()` function exported from `$lib/tauri`

**Wiring required**:
- [x] Add `importRecipeFromUrl` function in `src/lib/tauri/commands.ts`
- [x] Export from `src/lib/tauri/index.ts`

### Changes

#### Add Wrapper Function

**File**: `src/lib/tauri/commands.ts`

**Change**: Add after deleteRecipe (after line 76)

```typescript
export async function importRecipeFromUrl(url: string): Promise<Recipe> {
  return invoke<Recipe>("import_recipe_from_url", { url });
}
```

#### Export from Index

**File**: `src/lib/tauri/index.ts`

**Change**: Add to exports (around line 11)

```typescript
export {
  greet,
  getItems,
  createItem,
  deleteItem,
  // Recipe exports
  getRecipes,
  getRecipe,
  createRecipe,
  updateRecipe,
  deleteRecipe,
  importRecipeFromUrl,
  // ... rest unchanged
```

### Success Criteria

#### Automated Verification
- [x] Types check: `pnpm check`
- [N/A] Lint passes: `pnpm lint` (eslint not configured in project)

#### Integration Verification
- [x] Function importable: `import { importRecipeFromUrl } from "$lib/tauri"`

#### Manual Verification
- [ ] N/A (no UI integration yet)

**Checkpoint**: Verify `pnpm check` passes before proceeding to Phase 3.

---

## Phase 3: Component Integration

### Goal
Wire `ImportRecipe.svelte` to call backend with loading/error states and toast feedback.

### Integration Points

**Consumes from Phase 2**: `importRecipeFromUrl` from `$lib/tauri`
**Produces**: Complete working import feature

**Wiring required**:
- [x] Update `ImportRecipe.svelte` to use real Tauri call
- [x] Update `Recipes.svelte` `handleImport` to handle async result
- [x] Add toast notifications for success/error

### Changes

#### Update ImportRecipe Component

**File**: `src/lib/components/recipes/ImportRecipe.svelte`

**Change**: Replace entire file

```svelte
<script lang="ts">
  import { importRecipeFromUrl } from "$lib/tauri";
  import { toastStore } from "$lib/stores";
  import type { Recipe } from "$lib/types";

  interface Props {
    onSuccess: (recipe: Recipe) => void;
    onCancel: () => void;
  }

  let { onSuccess, onCancel }: Props = $props();
  let url = $state("");
  let isLoading = $state(false);
  let error = $state<string | null>(null);

  async function handleSubmit() {
    const trimmedUrl = url.trim();
    if (!trimmedUrl) {
      error = "Please enter a URL";
      return;
    }

    error = null;
    isLoading = true;

    try {
      const recipe = await importRecipeFromUrl(trimmedUrl);
      toastStore.success(`Imported "${recipe.name}"`);
      onSuccess(recipe);
    } catch (e) {
      const message = e instanceof Error ? e.message : String(e);
      error = message;
      toastStore.error(message);
    } finally {
      isLoading = false;
    }
  }

  function handleCancel() {
    if (!isLoading) {
      onCancel();
    }
  }
</script>

<div>
  <p class="text-gray-500 mb-6">
    Paste a link to a recipe from any website. We'll extract the ingredients, instructions, and more.
  </p>

  <form onsubmit={(e) => { e.preventDefault(); handleSubmit(); }} class="space-y-4">
    <div>
      <label class="block text-sm font-medium text-gray-700 mb-1">Recipe URL</label>
      <input
        type="url"
        bind:value={url}
        disabled={isLoading}
        required
        placeholder="https://www.example.com/recipe/..."
        class="w-full px-4 py-3 border border-gray-300 rounded-lg focus:ring-2 focus:ring-emerald-500 focus:border-emerald-500 disabled:bg-gray-100 disabled:cursor-not-allowed"
      />
      {#if error}
        <p class="mt-2 text-sm text-red-600">{error}</p>
      {/if}
    </div>

    {#if isLoading}
      <div class="flex items-center justify-center py-8">
        <div class="animate-spin rounded-full h-8 w-8 border-b-2 border-emerald-600"></div>
        <span class="ml-3 text-gray-600">Importing recipe...</span>
      </div>
    {/if}

    <div class="flex justify-end gap-3 pt-4">
      <button
        type="button"
        onclick={handleCancel}
        disabled={isLoading}
        class="px-4 py-2 text-gray-700 hover:bg-gray-100 rounded-lg transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
      >
        Cancel
      </button>
      <button
        type="submit"
        disabled={isLoading || !url.trim()}
        class="px-6 py-2 bg-emerald-600 text-white rounded-lg hover:bg-emerald-700 transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
      >
        {isLoading ? "Importing..." : "Import Recipe"}
      </button>
    </div>
  </form>
</div>
```

#### Update Recipes.svelte Handler

**File**: `src/lib/components/Recipes.svelte`

**Change**: Update handleImport function (lines 118-121)

```typescript
  function handleImport(recipe: Recipe) {
    // Recipe already created by backend, just add to store
    recipeStore.load(); // Refresh to get the new recipe
    modalView = "none";
  }
```

#### Update ImportRecipe Props in Recipes.svelte

**File**: `src/lib/components/Recipes.svelte`

**Change**: Update component usage (lines 261-264)

```svelte
<Modal isOpen={modalView === "import"} onClose={closeModal} title="Import Recipe">
  {#snippet children()}
    <ImportRecipe onSuccess={handleImport} onCancel={closeModal} />
  {/snippet}
</Modal>
```

### Success Criteria

#### Automated Verification
- [x] Types check: `pnpm check`
- [x] Tests pass: `pnpm test`
- [N/A] Lint passes: `pnpm lint` (eslint not configured in project)
- [x] Rust tests: `pnpm test:rust`

#### Integration Verification
- [x] Full build succeeds: `pnpm check` passed
- [x] Import flow compiles without errors

#### Manual Verification
- [ ] Import recipe from AllRecipes.com or similar site with JSON-LD
- [ ] Import recipe from a food blog with JSON-LD markup
- [ ] Try importing same URL twice — see duplicate error
- [ ] Try importing non-recipe URL — see appropriate error
- [ ] Verify imported recipe appears in list and can be viewed
- [ ] Success toast appears after import
- [ ] Error toast appears on failure, modal stays open

---

## Testing Strategy

### Unit Tests

**Backend** (`src-tauri/src/commands/recipes.rs`):
- `recipe_exists_by_source_url` returns true for existing URL
- `recipe_exists_by_source_url` returns false for new URL
- `parsed_to_input` correctly maps all fields
- Error message mapping for each `FetchError` variant
- Error message mapping for each `ParseError` variant

**Frontend** (`src/lib/components/recipes/ImportRecipe.test.ts`):
- Empty URL shows validation error
- Loading state shown during import
- Success callback called with recipe on success
- Error displayed on failure
- Cancel disabled during loading

### Integration Tests

**Backend**:
- Full import flow with mock HTML → recipe created in DB
- Duplicate detection blocks second import of same URL

### Manual Testing Checklist

1. [ ] Open Import modal from Recipes page
2. [ ] Paste valid recipe URL (e.g., from AllRecipes)
3. [ ] Click Import, verify spinner shows
4. [ ] Verify success toast and modal closes
5. [ ] Verify new recipe appears in list
6. [ ] Open recipe detail, verify all fields populated
7. [ ] Try importing same URL again, verify duplicate error
8. [ ] Try invalid URL, verify error message
9. [ ] Try URL with no recipe data, verify error message

## Rollback Plan

No data migration involved. Simple git revert:

```bash
git revert --no-commit HEAD~N..HEAD
```

Where N is the number of commits for this feature.

## Migration Notes

- **Data migration**: None required
- **Feature flags**: None
- **Backwards compatibility**: Not applicable (new feature)

## References

- Ticket: `ai_docs/prompts/2025-12-16-RUI-04-import-command-integration.md`
- Parent roadmap: `ai_docs/roadmaps/2025-12-16-recipe-url-import.md`
- Similar implementation: `src-tauri/src/commands/recipes.rs:19-22` (create_recipe pattern)
