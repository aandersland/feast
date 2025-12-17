# Instructions Storage Migration Implementation Plan

## Overview

Add database storage for recipe instructions by adding an `instructions TEXT` column to the recipes table that stores a JSON array of strings. Update CRUD operations to persist and load instructions.

## Current State

The `Recipe` struct has `instructions: Vec<String>` but `get_recipe_by_id` returns an empty vector (hardcoded placeholder). The `RecipeInput` accepts instructions but they're never saved.

**Key Discoveries**:
- Migration naming: `YYYYMMDDHHMMSS_description.sql` (e.g., `20250115000001_recipes_schema.sql`)
- Recipes table defined at `migrations/20250115000001_recipes_schema.sql:15-27`
- `get_recipe_by_id` returns `vec![]` at `src-tauri/src/db/recipes.rs:153`
- `create_recipe` INSERT at `src-tauri/src/db/recipes.rs:188-191` missing instructions
- `RecipeInput.instructions` exists at line 83, `Recipe.instructions` at line 54

## Desired End State

- New `instructions TEXT DEFAULT '[]'` column in recipes table
- `create_recipe` saves instructions as JSON string
- `update_recipe` saves instructions as JSON string
- `get_recipe_by_id` loads and parses instructions from JSON
- Existing recipes continue working with empty instructions

Verification: `cargo test recipes` passes with instruction CRUD tests.

## What We're NOT Doing

- JSON-LD parsing (Chunk 1)
- HTTP fetching (Chunk 2)
- Import command (Chunk 4)
- UI for editing instructions manually
- RecipeForm updates for manual instruction entry

## Integration Map

| Type | Location | Notes |
|------|----------|-------|
| Entry point | `migrations/20251216000001_add_instructions.sql` (new) | New migration file |
| Registration | N/A | sqlx runs migrations automatically on startup |
| Exports | N/A | No new exports needed |
| Consumers | `src-tauri/src/db/recipes.rs` | Update 3 functions |
| Events | N/A | None required |

## Implementation Approach

This is a straightforward database migration with code updates:
1. Add migration to add column with default value
2. Update INSERT/UPDATE queries to include instructions as JSON
3. Update SELECT to fetch instructions and parse JSON
4. Add tests to verify persistence

---

## Phase 1: Database Migration

### Goal
Add the `instructions` column to the recipes table with a default empty JSON array.

### Integration Points

**Depends on**: None
**Produces for next phase**: New database column available for CRUD operations

**Wiring required**:
- [x] Create migration file `src-tauri/migrations/20251216000001_add_instructions.sql`

### Changes

#### Migration File

**File**: `src-tauri/migrations/20251216000001_add_instructions.sql` (new)

**Change**: Add instructions column to recipes table

```sql
-- Add instructions column to recipes table
-- Stores a JSON array of instruction strings

ALTER TABLE recipes ADD COLUMN instructions TEXT NOT NULL DEFAULT '[]';
```

### Success Criteria

#### Automated Verification
- [x] `cargo build -p feast` compiles (sqlx checks migrations)
- [x] Migration applies without errors

#### Integration Verification
- [x] Column exists in database after migration runs

#### Manual Verification
- [ ] Run app, verify existing recipes still load

**Checkpoint**: Run the app to apply migration before proceeding to Phase 2.

---

## Phase 2: CRUD Operations Update

### Goal
Update `create_recipe`, `update_recipe`, and `get_recipe_by_id` to persist and load instructions as JSON.

### Integration Points

**Consumes from Phase 1**: `instructions` column in recipes table
**Produces for next phase**: Working instruction persistence

**Wiring required**:
- [x] Update `create_recipe` INSERT query at `src-tauri/src/db/recipes.rs:188`
- [x] Update `update_recipe` UPDATE query at `src-tauri/src/db/recipes.rs:258`
- [x] Update `get_recipe_by_id` to load and parse instructions at `src-tauri/src/db/recipes.rs:115`

### Changes

#### Update RecipeRow struct

**File**: `src-tauri/src/db/recipes.rs`

**Change**: Add instructions field to RecipeRow struct (around line 12)

```rust
#[derive(Debug, Clone, Serialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct RecipeRow {
    pub id: String,
    pub name: String,
    pub description: String,
    pub prep_time: i64,
    pub cook_time: i64,
    pub servings: i64,
    pub image_path: Option<String>,
    pub source_url: Option<String>,
    pub notes: Option<String>,
    pub instructions: String,  // JSON array stored as TEXT
    pub created_at: String,
    pub updated_at: String,
}
```

#### Add helper function for parsing instructions

**File**: `src-tauri/src/db/recipes.rs`

**Change**: Add helper function after the struct definitions (around line 95)

```rust
/// Parse instructions JSON string into Vec<String>
/// Returns empty vec on parse failure (graceful degradation)
fn parse_instructions(json_str: &str) -> Vec<String> {
    if json_str.is_empty() {
        return vec![];
    }
    serde_json::from_str(json_str).unwrap_or_else(|e| {
        log::warn!("Failed to parse instructions JSON: {}", e);
        vec![]
    })
}

/// Serialize instructions Vec<String> to JSON string
fn serialize_instructions(instructions: &[String]) -> String {
    serde_json::to_string(instructions).unwrap_or_else(|_| "[]".to_string())
}
```

#### Update get_all_recipes query

**File**: `src-tauri/src/db/recipes.rs`

**Change**: Add instructions to SELECT in `get_all_recipes` (around line 100)

```rust
/// Get all recipes (without ingredients for list view)
pub async fn get_all_recipes() -> Result<Vec<RecipeRow>, AppError> {
    let pool = get_db_pool();

    sqlx::query_as::<_, RecipeRow>(
        "SELECT id, name, description, prep_time, cook_time, servings,
                image_path, source_url, notes, instructions, created_at, updated_at
         FROM recipes ORDER BY created_at DESC",
    )
    .fetch_all(pool)
    .await
    .map_err(|e| AppError::Database(e.to_string()))
}
```

#### Update get_recipe_by_id query and parsing

**File**: `src-tauri/src/db/recipes.rs`

**Change**: Update SELECT query and use parsed instructions (around line 115)

```rust
/// Get a single recipe with all details
pub async fn get_recipe_by_id(id: &str) -> Result<Recipe, AppError> {
    let pool = get_db_pool();

    // Get recipe row
    let row = sqlx::query_as::<_, RecipeRow>(
        "SELECT id, name, description, prep_time, cook_time, servings,
                image_path, source_url, notes, instructions, created_at, updated_at
         FROM recipes WHERE id = ?",
    )
    .bind(id)
    .fetch_optional(pool)
    .await
    .map_err(|e| AppError::Database(e.to_string()))?
    .ok_or_else(|| AppError::NotFound(format!("Recipe with id {id} not found")))?;

    // Get ingredients
    let ingredients = sqlx::query_as::<_, RecipeIngredientRow>(
        "SELECT ri.id, ri.recipe_id, ri.ingredient_id, i.name, ri.quantity, ri.unit, ri.notes, ri.display_order
         FROM recipe_ingredients ri
         JOIN ingredients i ON ri.ingredient_id = i.id
         WHERE ri.recipe_id = ?
         ORDER BY ri.display_order",
    )
    .bind(id)
    .fetch_all(pool)
    .await
    .map_err(|e| AppError::Database(e.to_string()))?;

    // Get tags
    let tags: Vec<String> = sqlx::query_scalar(
        "SELECT t.name FROM tags t
         JOIN recipe_tags rt ON t.id = rt.tag_id
         WHERE rt.recipe_id = ?",
    )
    .bind(id)
    .fetch_all(pool)
    .await
    .map_err(|e| AppError::Database(e.to_string()))?;

    // Parse instructions from JSON
    let instructions = parse_instructions(&row.instructions);

    Ok(Recipe {
        id: row.id,
        name: row.name,
        description: row.description,
        prep_time: row.prep_time,
        cook_time: row.cook_time,
        servings: row.servings,
        image_path: row.image_path,
        source_url: row.source_url,
        notes: row.notes,
        tags,
        ingredients: ingredients
            .into_iter()
            .map(|i| RecipeIngredient {
                id: i.id,
                name: i.name,
                quantity: i.quantity,
                unit: i.unit,
                notes: i.notes,
            })
            .collect(),
        instructions,
        created_at: row.created_at,
        updated_at: row.updated_at,
    })
}
```

#### Update create_recipe INSERT

**File**: `src-tauri/src/db/recipes.rs`

**Change**: Add instructions to INSERT query (around line 188)

```rust
/// Create a new recipe with ingredients
pub async fn create_recipe(input: RecipeInput) -> Result<Recipe, AppError> {
    let pool = get_db_pool();
    let recipe_id = Uuid::new_v4().to_string();

    // Serialize instructions to JSON
    let instructions_json = serialize_instructions(&input.instructions);

    // Insert recipe
    sqlx::query(
        "INSERT INTO recipes (id, name, description, prep_time, cook_time, servings,
                              image_path, source_url, notes, instructions)
         VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
    )
    .bind(&recipe_id)
    .bind(&input.name)
    .bind(&input.description)
    .bind(input.prep_time)
    .bind(input.cook_time)
    .bind(input.servings)
    .bind(&input.image_path)
    .bind(&input.source_url)
    .bind(&input.notes)
    .bind(&instructions_json)
    .execute(pool)
    .await
    .map_err(|e| AppError::Database(e.to_string()))?;

    // ... rest of function unchanged (ingredients, tags, return)
```

#### Update update_recipe UPDATE

**File**: `src-tauri/src/db/recipes.rs`

**Change**: Add instructions to UPDATE query (around line 258)

```rust
/// Update an existing recipe
pub async fn update_recipe(id: &str, input: RecipeInput) -> Result<Recipe, AppError> {
    let pool = get_db_pool();

    // Verify recipe exists
    let existing = sqlx::query("SELECT id FROM recipes WHERE id = ?")
        .bind(id)
        .fetch_optional(pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

    if existing.is_none() {
        return Err(AppError::NotFound(format!("Recipe with id {id} not found")));
    }

    // Serialize instructions to JSON
    let instructions_json = serialize_instructions(&input.instructions);

    // Update recipe
    sqlx::query(
        "UPDATE recipes SET name = ?, description = ?, prep_time = ?, cook_time = ?,
                           servings = ?, image_path = ?, source_url = ?, notes = ?,
                           instructions = ?, updated_at = datetime('now')
         WHERE id = ?",
    )
    .bind(&input.name)
    .bind(&input.description)
    .bind(input.prep_time)
    .bind(input.cook_time)
    .bind(input.servings)
    .bind(&input.image_path)
    .bind(&input.source_url)
    .bind(&input.notes)
    .bind(&instructions_json)
    .bind(id)
    .execute(pool)
    .await
    .map_err(|e| AppError::Database(e.to_string()))?;

    // ... rest of function unchanged (delete/re-insert ingredients, tags, return)
```

### Success Criteria

#### Automated Verification
- [x] `cargo check -p feast` compiles without errors
- [x] `cargo clippy -p feast` passes with no warnings

#### Integration Verification
- [x] Creating a recipe with instructions saves them to DB
- [x] Fetching a recipe returns saved instructions
- [x] Updating a recipe updates instructions

#### Manual Verification
- [x] None required for Phase 2

**Checkpoint**: Run `cargo check` before proceeding to Phase 3.

---

## Phase 3: Tests

### Goal
Add tests verifying instruction persistence and edge case handling.

### Integration Points

**Consumes from Phase 2**: Working instruction CRUD
**Produces**: Verified instruction storage

**Wiring required**:
- [x] Add tests to existing `#[cfg(test)]` module in `src-tauri/src/db/recipes.rs`

### Changes

#### Add Instruction Tests

**File**: `src-tauri/src/db/recipes.rs` (append to tests module)

**Change**: Add tests for instruction persistence

```rust
// Add these tests to the existing #[cfg(test)] mod tests block

    #[tokio::test]
    async fn test_create_recipe_with_instructions() {
        init_db_for_test().await;

        let input = RecipeInput {
            name: "Recipe With Instructions".to_string(),
            description: "Test recipe".to_string(),
            prep_time: 10,
            cook_time: 20,
            servings: 4,
            image_path: None,
            source_url: None,
            notes: None,
            tags: vec![],
            ingredients: vec![IngredientInput {
                name: "Test Ingredient".to_string(),
                quantity: 1.0,
                unit: "cup".to_string(),
                category: None,
                notes: None,
            }],
            instructions: vec![
                "Preheat oven to 350°F.".to_string(),
                "Mix all ingredients.".to_string(),
                "Bake for 30 minutes.".to_string(),
            ],
        };

        let recipe = create_recipe(input).await.unwrap();

        assert_eq!(recipe.instructions.len(), 3);
        assert_eq!(recipe.instructions[0], "Preheat oven to 350°F.");
        assert_eq!(recipe.instructions[1], "Mix all ingredients.");
        assert_eq!(recipe.instructions[2], "Bake for 30 minutes.");

        // Fetch and verify persistence
        let fetched = get_recipe_by_id(&recipe.id).await.unwrap();
        assert_eq!(fetched.instructions.len(), 3);
        assert_eq!(fetched.instructions[0], "Preheat oven to 350°F.");
    }

    #[tokio::test]
    async fn test_update_recipe_instructions() {
        init_db_for_test().await;

        // Create recipe with initial instructions
        let input = RecipeInput {
            name: "Update Instructions Test".to_string(),
            description: "".to_string(),
            prep_time: 0,
            cook_time: 0,
            servings: 1,
            image_path: None,
            source_url: None,
            notes: None,
            tags: vec![],
            ingredients: vec![IngredientInput {
                name: "Ingredient".to_string(),
                quantity: 1.0,
                unit: "cup".to_string(),
                category: None,
                notes: None,
            }],
            instructions: vec!["Step 1".to_string()],
        };

        let recipe = create_recipe(input).await.unwrap();
        assert_eq!(recipe.instructions.len(), 1);

        // Update with new instructions
        let updated_input = RecipeInput {
            name: "Update Instructions Test".to_string(),
            description: "".to_string(),
            prep_time: 0,
            cook_time: 0,
            servings: 1,
            image_path: None,
            source_url: None,
            notes: None,
            tags: vec![],
            ingredients: vec![IngredientInput {
                name: "Ingredient".to_string(),
                quantity: 1.0,
                unit: "cup".to_string(),
                category: None,
                notes: None,
            }],
            instructions: vec![
                "New Step 1".to_string(),
                "New Step 2".to_string(),
            ],
        };

        let updated = update_recipe(&recipe.id, updated_input).await.unwrap();

        assert_eq!(updated.instructions.len(), 2);
        assert_eq!(updated.instructions[0], "New Step 1");
        assert_eq!(updated.instructions[1], "New Step 2");
    }

    #[tokio::test]
    async fn test_recipe_empty_instructions() {
        init_db_for_test().await;

        let input = RecipeInput {
            name: "No Instructions".to_string(),
            description: "".to_string(),
            prep_time: 0,
            cook_time: 0,
            servings: 1,
            image_path: None,
            source_url: None,
            notes: None,
            tags: vec![],
            ingredients: vec![IngredientInput {
                name: "Ingredient".to_string(),
                quantity: 1.0,
                unit: "cup".to_string(),
                category: None,
                notes: None,
            }],
            instructions: vec![], // Empty instructions
        };

        let recipe = create_recipe(input).await.unwrap();

        assert!(recipe.instructions.is_empty());

        // Fetch and verify
        let fetched = get_recipe_by_id(&recipe.id).await.unwrap();
        assert!(fetched.instructions.is_empty());
    }

    #[test]
    fn test_parse_instructions_valid_json() {
        let json = r#"["Step 1", "Step 2", "Step 3"]"#;
        let result = parse_instructions(json);
        assert_eq!(result.len(), 3);
        assert_eq!(result[0], "Step 1");
    }

    #[test]
    fn test_parse_instructions_empty_string() {
        let result = parse_instructions("");
        assert!(result.is_empty());
    }

    #[test]
    fn test_parse_instructions_empty_array() {
        let result = parse_instructions("[]");
        assert!(result.is_empty());
    }

    #[test]
    fn test_parse_instructions_malformed_json() {
        // Should return empty vec, not panic
        let result = parse_instructions("not valid json");
        assert!(result.is_empty());
    }

    #[test]
    fn test_parse_instructions_non_array_json() {
        // Object instead of array - should return empty vec
        let result = parse_instructions(r#"{"step": "value"}"#);
        assert!(result.is_empty());
    }

    #[test]
    fn test_serialize_instructions() {
        let instructions = vec!["Step 1".to_string(), "Step 2".to_string()];
        let json = serialize_instructions(&instructions);
        assert_eq!(json, r#"["Step 1","Step 2"]"#);
    }

    #[test]
    fn test_serialize_empty_instructions() {
        let instructions: Vec<String> = vec![];
        let json = serialize_instructions(&instructions);
        assert_eq!(json, "[]");
    }
```

### Success Criteria

#### Automated Verification
- [x] `cargo test recipes` passes all tests
- [x] `cargo clippy -p feast` passes with no warnings
- [x] `cargo fmt --check` passes

#### Integration Verification
- [x] Tests cover: create with instructions, update instructions, empty instructions
- [x] Tests cover: JSON parsing edge cases (empty, malformed, non-array)

#### Manual Verification
- [ ] Run app with existing database, verify recipes still load
- [ ] Create a recipe via UI, verify it saves (with empty instructions)

**Checkpoint**: Run `cargo test recipes` and verify all tests pass.

---

## Testing Strategy

### Unit Tests (Synchronous)
- `parse_instructions` with valid JSON, empty string, empty array, malformed JSON, non-array JSON
- `serialize_instructions` with instructions and empty vec

### Integration Tests (Async with DB)
- Create recipe with instructions, verify persistence
- Update recipe instructions, verify changes
- Create recipe with empty instructions, verify empty vec returned

### Manual Testing Checklist
1. [ ] `cargo test recipes` — all tests pass
2. [ ] Run app against dev database with existing recipes
3. [ ] Existing recipes load correctly (empty instructions)
4. [ ] Create new recipe via UI — saves successfully

## Rollback Plan

```sql
-- Rollback migration (if needed)
ALTER TABLE recipes DROP COLUMN instructions;
```

Or git revert to commit before Phase 1.

## Migration Notes

- **Data migration**: Existing recipes get default `'[]'` (empty array)
- **Feature flags**: None
- **Backwards compatibility**: Existing code continues working; instructions simply become non-empty when available

## References

- Ticket: `ai_docs/prompts/2025-12-16-RUI-03-instructions-storage.md`
- Parent roadmap: `ai_docs/roadmaps/2025-12-16-recipe-url-import.md`
- Existing schema: `src-tauri/migrations/20250115000001_recipes_schema.sql`
- Recipe CRUD: `src-tauri/src/db/recipes.rs`
