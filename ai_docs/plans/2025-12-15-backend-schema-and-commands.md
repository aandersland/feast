# Backend Schema & Commands Implementation Plan

## Overview

Build the complete SQLite database schema (9 tables) and ~30 Tauri commands to support all Feast app features: Recipes, Meal Planning, Shopping Lists, and Quick Lists. This replaces the current minimal `items` table with a full data model that aligns with the existing frontend TypeScript types.

## Current State

The backend has minimal infrastructure:
- 1 table: `items` (unused by the app)
- 4 commands: `greet`, `get_items`, `create_item`, `delete_item`
- Migration infrastructure via sqlx
- Connection pooling with `get_db_pool()`
- `AppError` enum for error handling

The frontend has complete TypeScript types in `src/lib/types/` that the backend schema must align with.

**Key Code References**:
- DB pattern: `src-tauri/src/db/items.rs:9-65`
- Command pattern: `src-tauri/src/commands/items.rs:1-22`
- Registration: `src-tauri/src/lib.rs:41-46`
- Frontend types: `src/lib/types/recipe.ts`, `mealPlan.ts`, `shoppingList.ts`

## Desired End State

- 9 database tables with proper relationships and constraints
- ~30 Tauri commands covering all CRUD operations
- Unit conversion utility for shopping list aggregation
- TypeScript IPC wrappers for all commands
- Comprehensive Rust unit tests for DB operations and commands
- All migrations idempotent and tested

## What We're NOT Doing

- Recipe import from URL (deferred to Phase 5 of roadmap)
- Offline support / sync
- Frontend store integration (Phase 3 of roadmap)
- Frontend component changes
- Image upload/management UI (just store path)
- Nutrition tracking in database (keep as optional JSON or defer)

## Integration Map

| Type | Location | File:Line | Action |
|------|----------|-----------|--------|
| DB modules | `src-tauri/src/db/mod.rs` | Line 3-6 | Add `pub mod recipes;` etc. |
| Command modules | `src-tauri/src/commands/mod.rs` | Line 3,7 | Add `pub mod recipes;` and re-exports |
| Command registration | `src-tauri/src/lib.rs` | Line 41-46 | Add commands to `generate_handler![]` |
| Error variants | `src-tauri/src/error/mod.rs` | Line 6-19 | Add `Conversion` variant |
| TypeScript types | `src/lib/types/` | Various | Add backend-specific types if needed |
| IPC wrappers | `src/lib/tauri/commands.ts` | Line 1-18 | Add wrapper functions |
| IPC exports | `src/lib/tauri/index.ts` | Line 1 | Add new exports |

## Implementation Approach

Feature-vertical: implement each domain completely (schema → db operations → commands → IPC wrappers → tests) before moving to the next. This allows testing each feature end-to-end.

**Dependency order**: Recipes → Meal Plans → Unit Conversion → Shopping Lists → Quick Lists → Manual Items

---

## Phase 1: Foundation & Recipes

### Goal

Create the recipes and ingredients schema, implement CRUD commands, and establish patterns for all subsequent phases.

### Integration Points

**Depends on**: None (foundation)
**Produces for next phase**: `recipes` and `ingredients` tables, Recipe CRUD commands

**Wiring required**:
- [x] Add `pub mod recipes;` and `pub mod ingredients;` to `src-tauri/src/db/mod.rs`
- [x] Add `pub mod recipes;` and `pub mod ingredients;` to `src-tauri/src/commands/mod.rs`
- [x] Add recipe/ingredient commands to `generate_handler![]` in `src-tauri/src/lib.rs`
- [x] Add IPC wrappers to `src/lib/tauri/commands.ts`
- [x] Export new functions from `src/lib/tauri/index.ts`

### Changes

#### Migration: `20250115000001_recipes_schema.sql`

**File**: `src-tauri/migrations/20250115000001_recipes_schema.sql`

```sql
-- Recipes schema

-- Ingredients table (normalized for aggregation)
CREATE TABLE IF NOT EXISTS ingredients (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL UNIQUE,
    category TEXT NOT NULL DEFAULT 'Other',
    default_unit TEXT
);

CREATE INDEX IF NOT EXISTS idx_ingredients_name ON ingredients(name);
CREATE INDEX IF NOT EXISTS idx_ingredients_category ON ingredients(category);

-- Recipes table
CREATE TABLE IF NOT EXISTS recipes (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT NOT NULL DEFAULT '',
    prep_time INTEGER NOT NULL DEFAULT 0,
    cook_time INTEGER NOT NULL DEFAULT 0,
    servings INTEGER NOT NULL DEFAULT 1,
    image_path TEXT,
    source_url TEXT,
    notes TEXT,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE INDEX IF NOT EXISTS idx_recipes_name ON recipes(name);
CREATE INDEX IF NOT EXISTS idx_recipes_created_at ON recipes(created_at);

-- Recipe ingredients junction table
CREATE TABLE IF NOT EXISTS recipe_ingredients (
    id TEXT PRIMARY KEY,
    recipe_id TEXT NOT NULL,
    ingredient_id TEXT NOT NULL,
    quantity REAL NOT NULL DEFAULT 0,
    unit TEXT NOT NULL DEFAULT '',
    notes TEXT,
    display_order INTEGER NOT NULL DEFAULT 0,
    FOREIGN KEY (recipe_id) REFERENCES recipes(id) ON DELETE CASCADE,
    FOREIGN KEY (ingredient_id) REFERENCES ingredients(id) ON DELETE RESTRICT
);

CREATE INDEX IF NOT EXISTS idx_recipe_ingredients_recipe ON recipe_ingredients(recipe_id);
CREATE INDEX IF NOT EXISTS idx_recipe_ingredients_ingredient ON recipe_ingredients(ingredient_id);

-- Tags table for recipe categorization
CREATE TABLE IF NOT EXISTS tags (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL UNIQUE
);

-- Recipe tags junction table
CREATE TABLE IF NOT EXISTS recipe_tags (
    recipe_id TEXT NOT NULL,
    tag_id TEXT NOT NULL,
    PRIMARY KEY (recipe_id, tag_id),
    FOREIGN KEY (recipe_id) REFERENCES recipes(id) ON DELETE CASCADE,
    FOREIGN KEY (tag_id) REFERENCES tags(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_recipe_tags_recipe ON recipe_tags(recipe_id);
CREATE INDEX IF NOT EXISTS idx_recipe_tags_tag ON recipe_tags(tag_id);
```

#### DB Module: `src-tauri/src/db/ingredients.rs`

**File**: `src-tauri/src/db/ingredients.rs`

```rust
//! Ingredient database operations

use crate::db::pool::get_db_pool;
use crate::error::AppError;
use serde::Serialize;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct Ingredient {
    pub id: String,
    pub name: String,
    pub category: String,
    pub default_unit: Option<String>,
}

/// Get all ingredients
pub async fn get_all_ingredients() -> Result<Vec<Ingredient>, AppError> {
    let pool = get_db_pool();

    sqlx::query_as::<_, Ingredient>(
        "SELECT id, name, category, default_unit FROM ingredients ORDER BY name"
    )
    .fetch_all(pool)
    .await
    .map_err(|e| AppError::Database(e.to_string()))
}

/// Create a new ingredient
pub async fn create_ingredient(name: &str, category: &str, default_unit: Option<&str>) -> Result<Ingredient, AppError> {
    let pool = get_db_pool();
    let id = Uuid::new_v4().to_string();
    let normalized_name = name.trim().to_lowercase();

    sqlx::query(
        "INSERT INTO ingredients (id, name, category, default_unit) VALUES (?, ?, ?, ?)"
    )
    .bind(&id)
    .bind(&normalized_name)
    .bind(category)
    .bind(default_unit)
    .execute(pool)
    .await
    .map_err(|e| AppError::Database(e.to_string()))?;

    Ok(Ingredient {
        id,
        name: normalized_name,
        category: category.to_string(),
        default_unit: default_unit.map(String::from),
    })
}

/// Get or create an ingredient by name (for recipe creation)
pub async fn get_or_create_ingredient(name: &str, category: &str, default_unit: Option<&str>) -> Result<Ingredient, AppError> {
    let pool = get_db_pool();
    let normalized_name = name.trim().to_lowercase();

    // Try to find existing
    let existing = sqlx::query_as::<_, Ingredient>(
        "SELECT id, name, category, default_unit FROM ingredients WHERE name = ?"
    )
    .bind(&normalized_name)
    .fetch_optional(pool)
    .await
    .map_err(|e| AppError::Database(e.to_string()))?;

    if let Some(ingredient) = existing {
        return Ok(ingredient);
    }

    // Create new
    create_ingredient(&normalized_name, category, default_unit).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::pool::init_db_for_test;

    #[tokio::test]
    async fn test_create_ingredient() {
        init_db_for_test().await;

        let ingredient = create_ingredient("Chicken Breast", "Meat & Seafood", Some("lb"))
            .await
            .unwrap();

        assert_eq!(ingredient.name, "chicken breast"); // normalized
        assert_eq!(ingredient.category, "Meat & Seafood");
        assert_eq!(ingredient.default_unit, Some("lb".to_string()));
    }

    #[tokio::test]
    async fn test_get_or_create_ingredient() {
        init_db_for_test().await;

        let ing1 = get_or_create_ingredient("Tomato", "Produce", None).await.unwrap();
        let ing2 = get_or_create_ingredient("TOMATO", "Produce", None).await.unwrap();

        assert_eq!(ing1.id, ing2.id); // Same ingredient returned
    }
}
```

#### DB Module: `src-tauri/src/db/recipes.rs`

**File**: `src-tauri/src/db/recipes.rs`

```rust
//! Recipe database operations

use crate::db::ingredients::{get_or_create_ingredient, Ingredient};
use crate::db::pool::get_db_pool;
use crate::error::AppError;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

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
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct RecipeIngredientRow {
    pub id: String,
    pub recipe_id: String,
    pub ingredient_id: String,
    pub name: String,
    pub quantity: f64,
    pub unit: String,
    pub notes: Option<String>,
    pub display_order: i64,
}

/// Full recipe with ingredients for API response
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Recipe {
    pub id: String,
    pub name: String,
    pub description: String,
    pub prep_time: i64,
    pub cook_time: i64,
    pub servings: i64,
    pub image_path: Option<String>,
    pub source_url: Option<String>,
    pub notes: Option<String>,
    pub tags: Vec<String>,
    pub ingredients: Vec<RecipeIngredient>,
    pub instructions: Vec<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RecipeIngredient {
    pub id: String,
    pub name: String,
    pub quantity: f64,
    pub unit: String,
    pub notes: Option<String>,
}

/// Input for creating/updating a recipe
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecipeInput {
    pub name: String,
    pub description: String,
    pub prep_time: i64,
    pub cook_time: i64,
    pub servings: i64,
    pub image_path: Option<String>,
    pub source_url: Option<String>,
    pub notes: Option<String>,
    pub tags: Vec<String>,
    pub ingredients: Vec<IngredientInput>,
    pub instructions: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IngredientInput {
    pub name: String,
    pub quantity: f64,
    pub unit: String,
    pub category: Option<String>,
    pub notes: Option<String>,
}

/// Get all recipes (without ingredients for list view)
pub async fn get_all_recipes() -> Result<Vec<RecipeRow>, AppError> {
    let pool = get_db_pool();

    sqlx::query_as::<_, RecipeRow>(
        "SELECT id, name, description, prep_time, cook_time, servings,
                image_path, source_url, notes, created_at, updated_at
         FROM recipes ORDER BY created_at DESC"
    )
    .fetch_all(pool)
    .await
    .map_err(|e| AppError::Database(e.to_string()))
}

/// Get a single recipe with all details
pub async fn get_recipe_by_id(id: &str) -> Result<Recipe, AppError> {
    let pool = get_db_pool();

    // Get recipe row
    let row = sqlx::query_as::<_, RecipeRow>(
        "SELECT id, name, description, prep_time, cook_time, servings,
                image_path, source_url, notes, created_at, updated_at
         FROM recipes WHERE id = ?"
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
         ORDER BY ri.display_order"
    )
    .bind(id)
    .fetch_all(pool)
    .await
    .map_err(|e| AppError::Database(e.to_string()))?;

    // Get tags
    let tags: Vec<String> = sqlx::query_scalar(
        "SELECT t.name FROM tags t
         JOIN recipe_tags rt ON t.id = rt.tag_id
         WHERE rt.recipe_id = ?"
    )
    .bind(id)
    .fetch_all(pool)
    .await
    .map_err(|e| AppError::Database(e.to_string()))?;

    // Get instructions (stored as JSON in notes for now, or separate table later)
    // For simplicity, we'll store instructions as newline-separated in a field
    // TODO: Consider separate instructions table if ordering/editing needed
    let instructions: Vec<String> = vec![]; // Placeholder - implement based on storage decision

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
        ingredients: ingredients.into_iter().map(|i| RecipeIngredient {
            id: i.id,
            name: i.name,
            quantity: i.quantity,
            unit: i.unit,
            notes: i.notes,
        }).collect(),
        instructions,
        created_at: row.created_at,
        updated_at: row.updated_at,
    })
}

/// Create a new recipe with ingredients
pub async fn create_recipe(input: RecipeInput) -> Result<Recipe, AppError> {
    let pool = get_db_pool();
    let recipe_id = Uuid::new_v4().to_string();

    // Insert recipe
    sqlx::query(
        "INSERT INTO recipes (id, name, description, prep_time, cook_time, servings,
                              image_path, source_url, notes)
         VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)"
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
    .execute(pool)
    .await
    .map_err(|e| AppError::Database(e.to_string()))?;

    // Insert ingredients
    for (order, ing_input) in input.ingredients.iter().enumerate() {
        let category = ing_input.category.as_deref().unwrap_or("Other");
        let ingredient = get_or_create_ingredient(&ing_input.name, category, None).await?;

        let ri_id = Uuid::new_v4().to_string();
        sqlx::query(
            "INSERT INTO recipe_ingredients (id, recipe_id, ingredient_id, quantity, unit, notes, display_order)
             VALUES (?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(&ri_id)
        .bind(&recipe_id)
        .bind(&ingredient.id)
        .bind(ing_input.quantity)
        .bind(&ing_input.unit)
        .bind(&ing_input.notes)
        .bind(order as i64)
        .execute(pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;
    }

    // Insert tags
    for tag_name in &input.tags {
        let tag_id = get_or_create_tag(tag_name).await?;
        sqlx::query("INSERT OR IGNORE INTO recipe_tags (recipe_id, tag_id) VALUES (?, ?)")
            .bind(&recipe_id)
            .bind(&tag_id)
            .execute(pool)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;
    }

    get_recipe_by_id(&recipe_id).await
}

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

    // Update recipe
    sqlx::query(
        "UPDATE recipes SET name = ?, description = ?, prep_time = ?, cook_time = ?,
                           servings = ?, image_path = ?, source_url = ?, notes = ?,
                           updated_at = datetime('now')
         WHERE id = ?"
    )
    .bind(&input.name)
    .bind(&input.description)
    .bind(input.prep_time)
    .bind(input.cook_time)
    .bind(input.servings)
    .bind(&input.image_path)
    .bind(&input.source_url)
    .bind(&input.notes)
    .bind(id)
    .execute(pool)
    .await
    .map_err(|e| AppError::Database(e.to_string()))?;

    // Delete existing ingredients and re-insert (simpler than diffing)
    sqlx::query("DELETE FROM recipe_ingredients WHERE recipe_id = ?")
        .bind(id)
        .execute(pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

    // Re-insert ingredients
    for (order, ing_input) in input.ingredients.iter().enumerate() {
        let category = ing_input.category.as_deref().unwrap_or("Other");
        let ingredient = get_or_create_ingredient(&ing_input.name, category, None).await?;

        let ri_id = Uuid::new_v4().to_string();
        sqlx::query(
            "INSERT INTO recipe_ingredients (id, recipe_id, ingredient_id, quantity, unit, notes, display_order)
             VALUES (?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(&ri_id)
        .bind(id)
        .bind(&ingredient.id)
        .bind(ing_input.quantity)
        .bind(&ing_input.unit)
        .bind(&ing_input.notes)
        .bind(order as i64)
        .execute(pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;
    }

    // Update tags
    sqlx::query("DELETE FROM recipe_tags WHERE recipe_id = ?")
        .bind(id)
        .execute(pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

    for tag_name in &input.tags {
        let tag_id = get_or_create_tag(tag_name).await?;
        sqlx::query("INSERT OR IGNORE INTO recipe_tags (recipe_id, tag_id) VALUES (?, ?)")
            .bind(id)
            .bind(&tag_id)
            .execute(pool)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;
    }

    get_recipe_by_id(id).await
}

/// Delete a recipe
pub async fn delete_recipe(id: &str) -> Result<(), AppError> {
    let pool = get_db_pool();

    let result = sqlx::query("DELETE FROM recipes WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound(format!("Recipe with id {id} not found")));
    }

    Ok(())
}

/// Helper: get or create a tag
async fn get_or_create_tag(name: &str) -> Result<String, AppError> {
    let pool = get_db_pool();
    let normalized = name.trim().to_lowercase();

    let existing: Option<String> = sqlx::query_scalar("SELECT id FROM tags WHERE name = ?")
        .bind(&normalized)
        .fetch_optional(pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

    if let Some(id) = existing {
        return Ok(id);
    }

    let id = Uuid::new_v4().to_string();
    sqlx::query("INSERT INTO tags (id, name) VALUES (?, ?)")
        .bind(&id)
        .bind(&normalized)
        .execute(pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

    Ok(id)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::pool::init_db_for_test;

    #[tokio::test]
    async fn test_create_and_get_recipe() {
        init_db_for_test().await;

        let input = RecipeInput {
            name: "Test Recipe".to_string(),
            description: "A test recipe".to_string(),
            prep_time: 10,
            cook_time: 20,
            servings: 4,
            image_path: None,
            source_url: None,
            notes: None,
            tags: vec!["dinner".to_string(), "easy".to_string()],
            ingredients: vec![
                IngredientInput {
                    name: "Chicken".to_string(),
                    quantity: 1.0,
                    unit: "lb".to_string(),
                    category: Some("Meat & Seafood".to_string()),
                    notes: None,
                },
            ],
            instructions: vec![],
        };

        let recipe = create_recipe(input).await.unwrap();

        assert_eq!(recipe.name, "Test Recipe");
        assert_eq!(recipe.servings, 4);
        assert_eq!(recipe.ingredients.len(), 1);
        assert_eq!(recipe.tags.len(), 2);

        // Fetch and verify
        let fetched = get_recipe_by_id(&recipe.id).await.unwrap();
        assert_eq!(fetched.name, recipe.name);
    }

    #[tokio::test]
    async fn test_delete_recipe_cascades() {
        init_db_for_test().await;

        let input = RecipeInput {
            name: "To Delete".to_string(),
            description: "".to_string(),
            prep_time: 0,
            cook_time: 0,
            servings: 1,
            image_path: None,
            source_url: None,
            notes: None,
            tags: vec![],
            ingredients: vec![
                IngredientInput {
                    name: "Ingredient".to_string(),
                    quantity: 1.0,
                    unit: "cup".to_string(),
                    category: None,
                    notes: None,
                },
            ],
            instructions: vec![],
        };

        let recipe = create_recipe(input).await.unwrap();
        delete_recipe(&recipe.id).await.unwrap();

        let result = get_recipe_by_id(&recipe.id).await;
        assert!(result.is_err());
    }
}
```

#### Commands Module: `src-tauri/src/commands/recipes.rs`

**File**: `src-tauri/src/commands/recipes.rs`

```rust
//! Recipe command handlers

use crate::db::recipes::{self, Recipe, RecipeInput, RecipeRow};
use tauri::command;

/// Get all recipes (list view)
#[command]
pub async fn get_recipes() -> Result<Vec<RecipeRow>, String> {
    recipes::get_all_recipes().await.map_err(|e| e.into())
}

/// Get a single recipe with full details
#[command]
pub async fn get_recipe(id: String) -> Result<Recipe, String> {
    recipes::get_recipe_by_id(&id).await.map_err(|e| e.into())
}

/// Create a new recipe
#[command]
pub async fn create_recipe(input: RecipeInput) -> Result<Recipe, String> {
    recipes::create_recipe(input).await.map_err(|e| e.into())
}

/// Update an existing recipe
#[command]
pub async fn update_recipe(id: String, input: RecipeInput) -> Result<Recipe, String> {
    recipes::update_recipe(&id, input).await.map_err(|e| e.into())
}

/// Delete a recipe
#[command]
pub async fn delete_recipe(id: String) -> Result<(), String> {
    recipes::delete_recipe(&id).await.map_err(|e| e.into())
}
```

#### Commands Module: `src-tauri/src/commands/ingredients.rs`

**File**: `src-tauri/src/commands/ingredients.rs`

```rust
//! Ingredient command handlers

use crate::db::ingredients::{self, Ingredient};
use tauri::command;

/// Get all ingredients
#[command]
pub async fn get_ingredients() -> Result<Vec<Ingredient>, String> {
    ingredients::get_all_ingredients().await.map_err(|e| e.into())
}

/// Create a new ingredient
#[command]
pub async fn create_ingredient(
    name: String,
    category: String,
    default_unit: Option<String>,
) -> Result<Ingredient, String> {
    ingredients::create_ingredient(&name, &category, default_unit.as_deref())
        .await
        .map_err(|e| e.into())
}

/// Get or create an ingredient
#[command]
pub async fn get_or_create_ingredient(
    name: String,
    category: String,
    default_unit: Option<String>,
) -> Result<Ingredient, String> {
    ingredients::get_or_create_ingredient(&name, &category, default_unit.as_deref())
        .await
        .map_err(|e| e.into())
}
```

#### Update: `src-tauri/src/db/mod.rs`

**File**: `src-tauri/src/db/mod.rs`

```rust
//! Database module

pub mod ingredients;
pub mod items;
pub mod pool;
pub mod recipes;

pub use pool::{get_db_pool, init_db};
```

#### Update: `src-tauri/src/commands/mod.rs`

**File**: `src-tauri/src/commands/mod.rs`

```rust
//! Tauri command handlers

pub mod ingredients;
pub mod items;
pub mod recipes;

use tauri::command;

pub use ingredients::{create_ingredient, get_ingredients, get_or_create_ingredient};
pub use items::{create_item, delete_item, get_items};
pub use recipes::{create_recipe, delete_recipe, get_recipe, get_recipes, update_recipe};

/// Greet a user by name
#[command]
#[must_use]
pub fn greet(name: &str) -> String {
    format!("Hello, {name}! Welcome to feast.")
}
```

#### Update: `src-tauri/src/lib.rs` (command registration)

Add to the `generate_handler![]` macro:

```rust
.invoke_handler(tauri::generate_handler![
    greet,
    get_items,
    create_item,
    delete_item,
    // Recipe commands
    get_recipes,
    get_recipe,
    create_recipe,
    update_recipe,
    delete_recipe,
    // Ingredient commands
    get_ingredients,
    create_ingredient,
    get_or_create_ingredient,
])
```

#### Update: `src-tauri/Cargo.toml`

Add uuid dependency:

```toml
[dependencies]
# ... existing deps ...
uuid = { version = "1.0", features = ["v4"] }
```

#### IPC Wrappers: `src/lib/tauri/commands.ts`

Add to existing file:

```typescript
import type { Recipe, Ingredient } from "$lib/types";

// Recipe types for backend
export interface RecipeInput {
  name: string;
  description: string;
  prepTime: number;
  cookTime: number;
  servings: number;
  imagePath?: string;
  sourceUrl?: string;
  notes?: string;
  tags: string[];
  ingredients: IngredientInput[];
  instructions: string[];
}

export interface IngredientInput {
  name: string;
  quantity: number;
  unit: string;
  category?: string;
  notes?: string;
}

export interface RecipeRow {
  id: string;
  name: string;
  description: string;
  prepTime: number;
  cookTime: number;
  servings: number;
  imagePath?: string;
  sourceUrl?: string;
  notes?: string;
  createdAt: string;
  updatedAt: string;
}

// Recipe commands
export async function getRecipes(): Promise<RecipeRow[]> {
  return invoke<RecipeRow[]>("get_recipes");
}

export async function getRecipe(id: string): Promise<Recipe> {
  return invoke<Recipe>("get_recipe", { id });
}

export async function createRecipe(input: RecipeInput): Promise<Recipe> {
  return invoke<Recipe>("create_recipe", { input });
}

export async function updateRecipe(id: string, input: RecipeInput): Promise<Recipe> {
  return invoke<Recipe>("update_recipe", { id, input });
}

export async function deleteRecipe(id: string): Promise<void> {
  return invoke<void>("delete_recipe", { id });
}

// Ingredient commands
export async function getIngredients(): Promise<Ingredient[]> {
  return invoke<Ingredient[]>("get_ingredients");
}

export async function createIngredient(
  name: string,
  category: string,
  defaultUnit?: string
): Promise<Ingredient> {
  return invoke<Ingredient>("create_ingredient", { name, category, defaultUnit });
}

export async function getOrCreateIngredient(
  name: string,
  category: string,
  defaultUnit?: string
): Promise<Ingredient> {
  return invoke<Ingredient>("get_or_create_ingredient", { name, category, defaultUnit });
}
```

#### Update: `src/lib/tauri/index.ts`

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
  // Ingredient exports
  getIngredients,
  createIngredient,
  getOrCreateIngredient,
} from "./commands";

export type { RecipeInput, IngredientInput, RecipeRow } from "./commands";
```

### Success Criteria

#### Automated Verification
- [x] Migration runs: `pnpm test:rust` passes
- [x] Rust tests pass: `cargo test` in src-tauri
- [x] Clippy clean: `cargo clippy` no warnings
- [x] Types compile: `pnpm check` passes

#### Integration Verification
- [x] Commands registered in `lib.rs`
- [x] Modules exported in `mod.rs` files
- [x] IPC wrappers exported in `index.ts`

#### Manual Verification
- [ ] Can create recipe via Tauri command in dev tools
- [ ] Can fetch recipe with ingredients
- [ ] Delete cascades to recipe_ingredients

**Checkpoint**: Pause for verification before Phase 2.

---

## Phase 2: Meal Plans

### Goal

Create meal plans schema and CRUD commands that reference recipes.

### Integration Points

**Depends on**: Phase 1 (recipes table must exist)
**Produces for next phase**: `meal_plans` table for shopping list aggregation

**Wiring required**:
- [x] Add `pub mod meal_plans;` to `src-tauri/src/db/mod.rs`
- [x] Add `pub mod meal_plans;` to `src-tauri/src/commands/mod.rs`
- [x] Add meal plan commands to `generate_handler![]`
- [x] Add IPC wrappers and exports

### Changes

#### Migration: `20250115000002_meal_plans_schema.sql`

**File**: `src-tauri/migrations/20250115000002_meal_plans_schema.sql`

```sql
-- Meal plans schema

CREATE TABLE IF NOT EXISTS meal_plans (
    id TEXT PRIMARY KEY,
    date TEXT NOT NULL,
    meal_type TEXT NOT NULL CHECK (meal_type IN ('breakfast', 'lunch', 'dinner', 'snack')),
    recipe_id TEXT NOT NULL,
    servings INTEGER NOT NULL DEFAULT 1,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    FOREIGN KEY (recipe_id) REFERENCES recipes(id) ON DELETE CASCADE,
    UNIQUE (date, meal_type, recipe_id)
);

CREATE INDEX IF NOT EXISTS idx_meal_plans_date ON meal_plans(date);
CREATE INDEX IF NOT EXISTS idx_meal_plans_recipe ON meal_plans(recipe_id);
```

#### DB Module: `src-tauri/src/db/meal_plans.rs`

**File**: `src-tauri/src/db/meal_plans.rs`

```rust
//! Meal plan database operations

use crate::db::pool::get_db_pool;
use crate::error::AppError;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct MealPlan {
    pub id: String,
    pub date: String,
    pub meal_type: String,
    pub recipe_id: String,
    pub servings: i64,
    pub created_at: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MealPlanInput {
    pub date: String,
    pub meal_type: String,
    pub recipe_id: String,
    pub servings: i64,
}

/// Get meal plans for a date range
pub async fn get_meal_plans(start_date: &str, end_date: &str) -> Result<Vec<MealPlan>, AppError> {
    let pool = get_db_pool();

    sqlx::query_as::<_, MealPlan>(
        "SELECT id, date, meal_type, recipe_id, servings, created_at
         FROM meal_plans
         WHERE date >= ? AND date <= ?
         ORDER BY date,
           CASE meal_type
             WHEN 'breakfast' THEN 1
             WHEN 'lunch' THEN 2
             WHEN 'dinner' THEN 3
             WHEN 'snack' THEN 4
           END"
    )
    .bind(start_date)
    .bind(end_date)
    .fetch_all(pool)
    .await
    .map_err(|e| AppError::Database(e.to_string()))
}

/// Create a meal plan entry
pub async fn create_meal_plan(input: MealPlanInput) -> Result<MealPlan, AppError> {
    let pool = get_db_pool();
    let id = Uuid::new_v4().to_string();

    // Validate meal_type
    if !["breakfast", "lunch", "dinner", "snack"].contains(&input.meal_type.as_str()) {
        return Err(AppError::Validation(format!(
            "Invalid meal type: {}. Must be breakfast, lunch, dinner, or snack",
            input.meal_type
        )));
    }

    sqlx::query(
        "INSERT INTO meal_plans (id, date, meal_type, recipe_id, servings)
         VALUES (?, ?, ?, ?, ?)"
    )
    .bind(&id)
    .bind(&input.date)
    .bind(&input.meal_type)
    .bind(&input.recipe_id)
    .bind(input.servings)
    .execute(pool)
    .await
    .map_err(|e| AppError::Database(e.to_string()))?;

    sqlx::query_as::<_, MealPlan>(
        "SELECT id, date, meal_type, recipe_id, servings, created_at
         FROM meal_plans WHERE id = ?"
    )
    .bind(&id)
    .fetch_one(pool)
    .await
    .map_err(|e| AppError::Database(e.to_string()))
}

/// Update meal plan servings
pub async fn update_meal_plan(id: &str, servings: i64) -> Result<MealPlan, AppError> {
    let pool = get_db_pool();

    let result = sqlx::query("UPDATE meal_plans SET servings = ? WHERE id = ?")
        .bind(servings)
        .bind(id)
        .execute(pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound(format!("Meal plan with id {id} not found")));
    }

    sqlx::query_as::<_, MealPlan>(
        "SELECT id, date, meal_type, recipe_id, servings, created_at
         FROM meal_plans WHERE id = ?"
    )
    .bind(id)
    .fetch_one(pool)
    .await
    .map_err(|e| AppError::Database(e.to_string()))
}

/// Delete a meal plan entry
pub async fn delete_meal_plan(id: &str) -> Result<(), AppError> {
    let pool = get_db_pool();

    let result = sqlx::query("DELETE FROM meal_plans WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound(format!("Meal plan with id {id} not found")));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::pool::init_db_for_test;
    use crate::db::recipes::{create_recipe, IngredientInput, RecipeInput};

    async fn create_test_recipe() -> String {
        let input = RecipeInput {
            name: "Test Recipe".to_string(),
            description: "".to_string(),
            prep_time: 0,
            cook_time: 0,
            servings: 4,
            image_path: None,
            source_url: None,
            notes: None,
            tags: vec![],
            ingredients: vec![],
            instructions: vec![],
        };
        create_recipe(input).await.unwrap().id
    }

    #[tokio::test]
    async fn test_create_meal_plan() {
        init_db_for_test().await;
        let recipe_id = create_test_recipe().await;

        let input = MealPlanInput {
            date: "2025-01-15".to_string(),
            meal_type: "dinner".to_string(),
            recipe_id,
            servings: 2,
        };

        let plan = create_meal_plan(input).await.unwrap();

        assert_eq!(plan.date, "2025-01-15");
        assert_eq!(plan.meal_type, "dinner");
        assert_eq!(plan.servings, 2);
    }

    #[tokio::test]
    async fn test_get_meal_plans_by_date_range() {
        init_db_for_test().await;
        let recipe_id = create_test_recipe().await;

        // Create plans for multiple days
        for date in ["2025-01-13", "2025-01-14", "2025-01-15", "2025-01-20"] {
            let input = MealPlanInput {
                date: date.to_string(),
                meal_type: "dinner".to_string(),
                recipe_id: recipe_id.clone(),
                servings: 2,
            };
            create_meal_plan(input).await.unwrap();
        }

        let plans = get_meal_plans("2025-01-13", "2025-01-15").await.unwrap();
        assert_eq!(plans.len(), 3); // Only dates in range
    }
}
```

#### Commands Module: `src-tauri/src/commands/meal_plans.rs`

**File**: `src-tauri/src/commands/meal_plans.rs`

```rust
//! Meal plan command handlers

use crate::db::meal_plans::{self, MealPlan, MealPlanInput};
use tauri::command;

/// Get meal plans for a date range
#[command]
pub async fn get_meal_plans(start_date: String, end_date: String) -> Result<Vec<MealPlan>, String> {
    meal_plans::get_meal_plans(&start_date, &end_date)
        .await
        .map_err(|e| e.into())
}

/// Create a meal plan entry
#[command]
pub async fn create_meal_plan(input: MealPlanInput) -> Result<MealPlan, String> {
    meal_plans::create_meal_plan(input).await.map_err(|e| e.into())
}

/// Update meal plan servings
#[command]
pub async fn update_meal_plan(id: String, servings: i64) -> Result<MealPlan, String> {
    meal_plans::update_meal_plan(&id, servings)
        .await
        .map_err(|e| e.into())
}

/// Delete a meal plan entry
#[command]
pub async fn delete_meal_plan(id: String) -> Result<(), String> {
    meal_plans::delete_meal_plan(&id).await.map_err(|e| e.into())
}
```

#### IPC Wrappers

Add to `src/lib/tauri/commands.ts`:

```typescript
import type { MealPlan } from "$lib/types";

export interface MealPlanInput {
  date: string;
  mealType: string;
  recipeId: string;
  servings: number;
}

// Meal plan commands
export async function getMealPlans(startDate: string, endDate: string): Promise<MealPlan[]> {
  return invoke<MealPlan[]>("get_meal_plans", { startDate, endDate });
}

export async function createMealPlan(input: MealPlanInput): Promise<MealPlan> {
  return invoke<MealPlan>("create_meal_plan", { input });
}

export async function updateMealPlan(id: string, servings: number): Promise<MealPlan> {
  return invoke<MealPlan>("update_meal_plan", { id, servings });
}

export async function deleteMealPlan(id: string): Promise<void> {
  return invoke<void>("delete_meal_plan", { id });
}
```

### Success Criteria

#### Automated Verification
- [x] Migration runs successfully
- [x] Rust tests pass
- [x] Types compile

#### Manual Verification
- [ ] Can create meal plan referencing a recipe
- [ ] Can fetch meal plans by date range
- [ ] Deleting recipe cascades to meal plans

**Checkpoint**: Pause for verification before Phase 3.

---

## Phase 3: Unit Conversion Utility

### Goal

Create a Rust module for unit conversion to support shopping list aggregation.

### Integration Points

**Depends on**: None (utility module)
**Produces for next phase**: `convert_units` function for aggregation

**Wiring required**:
- [x] Add `pub mod units;` to `src-tauri/src/db/mod.rs` or create new `utils` module

### Changes

#### Utility Module: `src-tauri/src/utils/units.rs`

**File**: `src-tauri/src/utils/mod.rs`

```rust
//! Utility modules

pub mod units;
```

**File**: `src-tauri/src/utils/units.rs`

```rust
//! Unit conversion for ingredient aggregation

use std::collections::HashMap;

/// Unit categories for grouping compatible units
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UnitCategory {
    Volume,
    Weight,
    Count,
    Other,
}

/// Get the category for a unit
pub fn get_unit_category(unit: &str) -> UnitCategory {
    let unit_lower = unit.to_lowercase();
    match unit_lower.as_str() {
        // Volume units
        "cup" | "cups" | "c" |
        "tablespoon" | "tablespoons" | "tbsp" | "tbs" |
        "teaspoon" | "teaspoons" | "tsp" |
        "ml" | "milliliter" | "milliliters" |
        "l" | "liter" | "liters" |
        "fl oz" | "fluid ounce" | "fluid ounces" |
        "pint" | "pints" | "pt" |
        "quart" | "quarts" | "qt" |
        "gallon" | "gallons" | "gal" => UnitCategory::Volume,

        // Weight units
        "g" | "gram" | "grams" |
        "kg" | "kilogram" | "kilograms" |
        "oz" | "ounce" | "ounces" |
        "lb" | "lbs" | "pound" | "pounds" => UnitCategory::Weight,

        // Count units
        "" | "whole" | "piece" | "pieces" |
        "clove" | "cloves" |
        "slice" | "slices" |
        "can" | "cans" |
        "bunch" | "bunches" |
        "head" | "heads" |
        "stalk" | "stalks" |
        "sprig" | "sprigs" => UnitCategory::Count,

        _ => UnitCategory::Other,
    }
}

/// Conversion factors to a base unit within each category
/// Volume: base = ml
/// Weight: base = g
fn get_conversion_factor(unit: &str) -> Option<f64> {
    let unit_lower = unit.to_lowercase();
    match unit_lower.as_str() {
        // Volume to ml
        "ml" | "milliliter" | "milliliters" => Some(1.0),
        "l" | "liter" | "liters" => Some(1000.0),
        "tsp" | "teaspoon" | "teaspoons" => Some(4.929),
        "tbsp" | "tbs" | "tablespoon" | "tablespoons" => Some(14.787),
        "fl oz" | "fluid ounce" | "fluid ounces" => Some(29.574),
        "cup" | "cups" | "c" => Some(236.588),
        "pint" | "pints" | "pt" => Some(473.176),
        "quart" | "quarts" | "qt" => Some(946.353),
        "gallon" | "gallons" | "gal" => Some(3785.41),

        // Weight to g
        "g" | "gram" | "grams" => Some(1.0),
        "kg" | "kilogram" | "kilograms" => Some(1000.0),
        "oz" | "ounce" | "ounces" => Some(28.3495),
        "lb" | "lbs" | "pound" | "pounds" => Some(453.592),

        _ => None,
    }
}

/// Normalize a unit to its base form for display
pub fn normalize_unit(unit: &str) -> String {
    let unit_lower = unit.to_lowercase();
    match unit_lower.as_str() {
        "c" => "cup".to_string(),
        "cups" => "cup".to_string(),
        "tbs" | "tablespoons" => "tbsp".to_string(),
        "teaspoons" => "tsp".to_string(),
        "milliliters" | "milliliter" => "ml".to_string(),
        "liters" | "liter" => "L".to_string(),
        "fluid ounces" | "fluid ounce" | "fl oz" => "fl oz".to_string(),
        "pints" | "pt" => "pint".to_string(),
        "quarts" | "qt" => "quart".to_string(),
        "gallons" | "gal" => "gallon".to_string(),
        "grams" | "gram" => "g".to_string(),
        "kilograms" | "kilogram" => "kg".to_string(),
        "ounces" | "ounce" => "oz".to_string(),
        "pounds" | "pound" | "lbs" => "lb".to_string(),
        "pieces" | "piece" => "".to_string(),
        _ => unit_lower,
    }
}

/// Convert a quantity from one unit to another
/// Returns None if units are incompatible
pub fn convert_quantity(quantity: f64, from_unit: &str, to_unit: &str) -> Option<f64> {
    let from_cat = get_unit_category(from_unit);
    let to_cat = get_unit_category(to_unit);

    // Must be same category
    if from_cat != to_cat {
        return None;
    }

    // Count units don't convert
    if from_cat == UnitCategory::Count || from_cat == UnitCategory::Other {
        if normalize_unit(from_unit) == normalize_unit(to_unit) {
            return Some(quantity);
        }
        return None;
    }

    let from_factor = get_conversion_factor(from_unit)?;
    let to_factor = get_conversion_factor(to_unit)?;

    // Convert: from_unit -> base -> to_unit
    Some(quantity * from_factor / to_factor)
}

/// Result of aggregating quantities
#[derive(Debug, Clone)]
pub struct AggregatedQuantity {
    pub quantity: f64,
    pub unit: String,
    pub is_converted: bool,
}

/// Aggregate multiple quantities of the same ingredient
/// Returns the best unit and total quantity, or separate entries if incompatible
pub fn aggregate_quantities(items: &[(f64, String)]) -> Vec<AggregatedQuantity> {
    if items.is_empty() {
        return vec![];
    }

    // Group by unit category
    let mut by_category: HashMap<UnitCategory, Vec<(f64, String)>> = HashMap::new();

    for (qty, unit) in items {
        let cat = get_unit_category(unit);
        by_category.entry(cat).or_default().push((*qty, unit.clone()));
    }

    let mut results = vec![];

    for (category, group) in by_category {
        if category == UnitCategory::Count || category == UnitCategory::Other {
            // For count/other, group by normalized unit
            let mut by_unit: HashMap<String, f64> = HashMap::new();
            for (qty, unit) in group {
                let normalized = normalize_unit(&unit);
                *by_unit.entry(normalized).or_default() += qty;
            }
            for (unit, qty) in by_unit {
                results.push(AggregatedQuantity {
                    quantity: qty,
                    unit,
                    is_converted: false,
                });
            }
        } else {
            // For volume/weight, convert to most common unit
            let target_unit = find_best_target_unit(&group);
            let mut total = 0.0;
            let mut any_converted = false;

            for (qty, unit) in &group {
                if let Some(converted) = convert_quantity(*qty, unit, &target_unit) {
                    total += converted;
                    if normalize_unit(unit) != normalize_unit(&target_unit) {
                        any_converted = true;
                    }
                }
            }

            results.push(AggregatedQuantity {
                quantity: total,
                unit: normalize_unit(&target_unit),
                is_converted: any_converted,
            });
        }
    }

    results
}

/// Find the most common unit in a group (for choosing target unit)
fn find_best_target_unit(items: &[(f64, String)]) -> String {
    let mut counts: HashMap<String, usize> = HashMap::new();
    for (_, unit) in items {
        let normalized = normalize_unit(unit);
        *counts.entry(normalized).or_default() += 1;
    }

    counts
        .into_iter()
        .max_by_key(|(_, count)| *count)
        .map(|(unit, _)| unit)
        .unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unit_categories() {
        assert_eq!(get_unit_category("cup"), UnitCategory::Volume);
        assert_eq!(get_unit_category("CUPS"), UnitCategory::Volume);
        assert_eq!(get_unit_category("lb"), UnitCategory::Weight);
        assert_eq!(get_unit_category(""), UnitCategory::Count);
        assert_eq!(get_unit_category("pinch"), UnitCategory::Other);
    }

    #[test]
    fn test_convert_volume() {
        // 2 cups to tbsp
        let result = convert_quantity(2.0, "cup", "tbsp").unwrap();
        assert!((result - 32.0).abs() < 0.1); // 2 cups ≈ 32 tbsp

        // 1 liter to cups
        let result = convert_quantity(1.0, "L", "cup").unwrap();
        assert!((result - 4.227).abs() < 0.01);
    }

    #[test]
    fn test_convert_weight() {
        // 1 lb to oz
        let result = convert_quantity(1.0, "lb", "oz").unwrap();
        assert!((result - 16.0).abs() < 0.01);

        // 1 kg to lb
        let result = convert_quantity(1.0, "kg", "lb").unwrap();
        assert!((result - 2.205).abs() < 0.01);
    }

    #[test]
    fn test_incompatible_units() {
        // Can't convert volume to weight
        assert!(convert_quantity(1.0, "cup", "lb").is_none());
    }

    #[test]
    fn test_aggregate_same_unit() {
        let items = vec![
            (1.0, "cup".to_string()),
            (0.5, "cup".to_string()),
        ];
        let result = aggregate_quantities(&items);
        assert_eq!(result.len(), 1);
        assert!((result[0].quantity - 1.5).abs() < 0.001);
        assert_eq!(result[0].unit, "cup");
    }

    #[test]
    fn test_aggregate_different_volume_units() {
        let items = vec![
            (1.0, "cup".to_string()),
            (2.0, "tbsp".to_string()),
        ];
        let result = aggregate_quantities(&items);
        assert_eq!(result.len(), 1);
        // Result should be in cups (most common), 1 cup + 2 tbsp ≈ 1.125 cups
        assert!(result[0].is_converted);
    }

    #[test]
    fn test_aggregate_incompatible() {
        let items = vec![
            (1.0, "cup".to_string()),
            (2.0, "lb".to_string()),
        ];
        let result = aggregate_quantities(&items);
        assert_eq!(result.len(), 2); // Can't combine, separate entries
    }
}
```

#### Update: `src-tauri/src/lib.rs`

Add module declaration:

```rust
pub mod commands;
pub mod db;
pub mod error;
pub mod utils;
```

### Success Criteria

#### Automated Verification
- [x] All unit conversion tests pass
- [x] Clippy clean

#### Manual Verification
- [ ] Converting 2 cups to tbsp gives ~32 tbsp
- [ ] Incompatible units return separate entries

**Checkpoint**: Pause for verification before Phase 4.

---

## Phase 4: Shopping Lists

### Goal

Create shopping lists schema with CRUD and aggregation from meal plans.

### Integration Points

**Depends on**: Phase 2 (meal_plans), Phase 3 (unit conversion)
**Produces for next phase**: Shopping list functionality for frontend

**Wiring required**:
- [x] Add `pub mod shopping_lists;` to db and commands modules
- [x] Register all shopping list commands

### Changes

#### Migration: `20250115000003_shopping_lists_schema.sql`

**File**: `src-tauri/migrations/20250115000003_shopping_lists_schema.sql`

```sql
-- Shopping lists schema

CREATE TABLE IF NOT EXISTS shopping_lists (
    id TEXT PRIMARY KEY,
    week_start TEXT NOT NULL,
    name TEXT NOT NULL,
    list_type TEXT NOT NULL DEFAULT 'weekly' CHECK (list_type IN ('weekly', 'midweek', 'custom')),
    created_at TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE INDEX IF NOT EXISTS idx_shopping_lists_week ON shopping_lists(week_start);

CREATE TABLE IF NOT EXISTS shopping_list_items (
    id TEXT PRIMARY KEY,
    list_id TEXT NOT NULL,
    ingredient_id TEXT,
    name TEXT NOT NULL,
    quantity REAL NOT NULL DEFAULT 0,
    unit TEXT NOT NULL DEFAULT '',
    category TEXT NOT NULL DEFAULT 'Other',
    is_checked INTEGER NOT NULL DEFAULT 0,
    is_deleted INTEGER NOT NULL DEFAULT 0,
    deleted_at TEXT,
    moved_to_list_id TEXT,
    source_recipe_ids TEXT,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    FOREIGN KEY (list_id) REFERENCES shopping_lists(id) ON DELETE CASCADE,
    FOREIGN KEY (ingredient_id) REFERENCES ingredients(id) ON DELETE SET NULL,
    FOREIGN KEY (moved_to_list_id) REFERENCES shopping_lists(id) ON DELETE SET NULL
);

CREATE INDEX IF NOT EXISTS idx_shopping_list_items_list ON shopping_list_items(list_id);
CREATE INDEX IF NOT EXISTS idx_shopping_list_items_ingredient ON shopping_list_items(ingredient_id);
```

#### DB Module: `src-tauri/src/db/shopping_lists.rs`

**File**: `src-tauri/src/db/shopping_lists.rs`

```rust
//! Shopping list database operations

use crate::db::pool::get_db_pool;
use crate::error::AppError;
use crate::utils::units::{aggregate_quantities, AggregatedQuantity};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct ShoppingList {
    pub id: String,
    pub week_start: String,
    pub name: String,
    pub list_type: String,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct ShoppingListItem {
    pub id: String,
    pub list_id: String,
    pub ingredient_id: Option<String>,
    pub name: String,
    pub quantity: f64,
    pub unit: String,
    pub category: String,
    pub is_checked: bool,
    pub is_deleted: bool,
    pub deleted_at: Option<String>,
    pub moved_to_list_id: Option<String>,
    pub source_recipe_ids: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ShoppingListWithItems {
    #[serde(flatten)]
    pub list: ShoppingList,
    pub items: Vec<ShoppingListItem>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShoppingListInput {
    pub week_start: String,
    pub name: String,
    pub list_type: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShoppingItemInput {
    pub list_id: String,
    pub name: String,
    pub quantity: f64,
    pub unit: String,
    pub category: String,
}

/// Aggregated shopping item (from meal plans)
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AggregatedShoppingItem {
    pub name: String,
    pub quantity: f64,
    pub unit: String,
    pub category: String,
    pub source_recipe_ids: Vec<String>,
    pub is_converted: bool,
}

/// Get shopping lists for a week
pub async fn get_shopping_lists(week_start: &str) -> Result<Vec<ShoppingListWithItems>, AppError> {
    let pool = get_db_pool();

    let lists = sqlx::query_as::<_, ShoppingList>(
        "SELECT id, week_start, name, list_type, created_at
         FROM shopping_lists WHERE week_start = ?
         ORDER BY created_at"
    )
    .bind(week_start)
    .fetch_all(pool)
    .await
    .map_err(|e| AppError::Database(e.to_string()))?;

    let mut result = vec![];
    for list in lists {
        let items = get_list_items(&list.id).await?;
        result.push(ShoppingListWithItems { list, items });
    }

    Ok(result)
}

async fn get_list_items(list_id: &str) -> Result<Vec<ShoppingListItem>, AppError> {
    let pool = get_db_pool();

    sqlx::query_as::<_, ShoppingListItem>(
        "SELECT id, list_id, ingredient_id, name, quantity, unit, category,
                is_checked, is_deleted, deleted_at, moved_to_list_id,
                source_recipe_ids, created_at
         FROM shopping_list_items WHERE list_id = ?
         ORDER BY category, name"
    )
    .bind(list_id)
    .fetch_all(pool)
    .await
    .map_err(|e| AppError::Database(e.to_string()))
}

/// Create a shopping list
pub async fn create_shopping_list(input: ShoppingListInput) -> Result<ShoppingList, AppError> {
    let pool = get_db_pool();
    let id = Uuid::new_v4().to_string();
    let list_type = input.list_type.unwrap_or_else(|| "custom".to_string());

    sqlx::query(
        "INSERT INTO shopping_lists (id, week_start, name, list_type) VALUES (?, ?, ?, ?)"
    )
    .bind(&id)
    .bind(&input.week_start)
    .bind(&input.name)
    .bind(&list_type)
    .execute(pool)
    .await
    .map_err(|e| AppError::Database(e.to_string()))?;

    sqlx::query_as::<_, ShoppingList>(
        "SELECT id, week_start, name, list_type, created_at FROM shopping_lists WHERE id = ?"
    )
    .bind(&id)
    .fetch_one(pool)
    .await
    .map_err(|e| AppError::Database(e.to_string()))
}

/// Delete a shopping list
pub async fn delete_shopping_list(id: &str) -> Result<(), AppError> {
    let pool = get_db_pool();

    let result = sqlx::query("DELETE FROM shopping_lists WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound(format!("Shopping list with id {id} not found")));
    }

    Ok(())
}

/// Add an item to a shopping list
pub async fn add_shopping_item(input: ShoppingItemInput) -> Result<ShoppingListItem, AppError> {
    let pool = get_db_pool();
    let id = Uuid::new_v4().to_string();

    sqlx::query(
        "INSERT INTO shopping_list_items (id, list_id, name, quantity, unit, category)
         VALUES (?, ?, ?, ?, ?, ?)"
    )
    .bind(&id)
    .bind(&input.list_id)
    .bind(&input.name)
    .bind(input.quantity)
    .bind(&input.unit)
    .bind(&input.category)
    .execute(pool)
    .await
    .map_err(|e| AppError::Database(e.to_string()))?;

    sqlx::query_as::<_, ShoppingListItem>(
        "SELECT id, list_id, ingredient_id, name, quantity, unit, category,
                is_checked, is_deleted, deleted_at, moved_to_list_id,
                source_recipe_ids, created_at
         FROM shopping_list_items WHERE id = ?"
    )
    .bind(&id)
    .fetch_one(pool)
    .await
    .map_err(|e| AppError::Database(e.to_string()))
}

/// Update a shopping item
pub async fn update_shopping_item(
    id: &str,
    quantity: Option<f64>,
    is_checked: Option<bool>,
) -> Result<ShoppingListItem, AppError> {
    let pool = get_db_pool();

    if let Some(qty) = quantity {
        sqlx::query("UPDATE shopping_list_items SET quantity = ? WHERE id = ?")
            .bind(qty)
            .bind(id)
            .execute(pool)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;
    }

    if let Some(checked) = is_checked {
        sqlx::query("UPDATE shopping_list_items SET is_checked = ? WHERE id = ?")
            .bind(checked)
            .bind(id)
            .execute(pool)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;
    }

    sqlx::query_as::<_, ShoppingListItem>(
        "SELECT id, list_id, ingredient_id, name, quantity, unit, category,
                is_checked, is_deleted, deleted_at, moved_to_list_id,
                source_recipe_ids, created_at
         FROM shopping_list_items WHERE id = ?"
    )
    .bind(id)
    .fetch_one(pool)
    .await
    .map_err(|e| AppError::Database(e.to_string()))
}

/// Soft delete an item
pub async fn soft_delete_shopping_item(id: &str) -> Result<(), AppError> {
    let pool = get_db_pool();

    let result = sqlx::query(
        "UPDATE shopping_list_items SET is_deleted = 1, deleted_at = datetime('now') WHERE id = ?"
    )
    .bind(id)
    .execute(pool)
    .await
    .map_err(|e| AppError::Database(e.to_string()))?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound(format!("Shopping item with id {id} not found")));
    }

    Ok(())
}

/// Restore a soft-deleted item
pub async fn restore_shopping_item(id: &str) -> Result<ShoppingListItem, AppError> {
    let pool = get_db_pool();

    let result = sqlx::query(
        "UPDATE shopping_list_items SET is_deleted = 0, deleted_at = NULL WHERE id = ?"
    )
    .bind(id)
    .execute(pool)
    .await
    .map_err(|e| AppError::Database(e.to_string()))?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound(format!("Shopping item with id {id} not found")));
    }

    sqlx::query_as::<_, ShoppingListItem>(
        "SELECT id, list_id, ingredient_id, name, quantity, unit, category,
                is_checked, is_deleted, deleted_at, moved_to_list_id,
                source_recipe_ids, created_at
         FROM shopping_list_items WHERE id = ?"
    )
    .bind(id)
    .fetch_one(pool)
    .await
    .map_err(|e| AppError::Database(e.to_string()))
}

/// Move an item to another list
pub async fn move_shopping_item(id: &str, to_list_id: &str) -> Result<ShoppingListItem, AppError> {
    let pool = get_db_pool();

    sqlx::query(
        "UPDATE shopping_list_items SET list_id = ?, moved_to_list_id = ? WHERE id = ?"
    )
    .bind(to_list_id)
    .bind(to_list_id)
    .bind(id)
    .execute(pool)
    .await
    .map_err(|e| AppError::Database(e.to_string()))?;

    sqlx::query_as::<_, ShoppingListItem>(
        "SELECT id, list_id, ingredient_id, name, quantity, unit, category,
                is_checked, is_deleted, deleted_at, moved_to_list_id,
                source_recipe_ids, created_at
         FROM shopping_list_items WHERE id = ?"
    )
    .bind(id)
    .fetch_one(pool)
    .await
    .map_err(|e| AppError::Database(e.to_string()))
}

/// Get aggregated shopping list from meal plans for a date range
pub async fn get_aggregated_shopping_list(
    start_date: &str,
    end_date: &str,
) -> Result<Vec<AggregatedShoppingItem>, AppError> {
    let pool = get_db_pool();

    // Get all ingredients from meal plans in date range
    #[derive(FromRow)]
    struct RawItem {
        name: String,
        quantity: f64,
        unit: String,
        category: String,
        recipe_id: String,
        servings_multiplier: f64,
    }

    let items = sqlx::query_as::<_, RawItem>(
        "SELECT
            i.name,
            ri.quantity,
            ri.unit,
            i.category,
            mp.recipe_id,
            CAST(mp.servings AS REAL) / CAST(r.servings AS REAL) as servings_multiplier
         FROM meal_plans mp
         JOIN recipes r ON mp.recipe_id = r.id
         JOIN recipe_ingredients ri ON r.id = ri.recipe_id
         JOIN ingredients i ON ri.ingredient_id = i.id
         WHERE mp.date >= ? AND mp.date <= ?"
    )
    .bind(start_date)
    .bind(end_date)
    .fetch_all(pool)
    .await
    .map_err(|e| AppError::Database(e.to_string()))?;

    // Group by ingredient name (lowercase) and aggregate
    use std::collections::HashMap;

    struct GroupedItem {
        category: String,
        quantities: Vec<(f64, String)>,
        recipe_ids: Vec<String>,
    }

    let mut grouped: HashMap<String, GroupedItem> = HashMap::new();

    for item in items {
        let key = item.name.to_lowercase();
        let adjusted_qty = item.quantity * item.servings_multiplier;

        grouped
            .entry(key.clone())
            .or_insert_with(|| GroupedItem {
                category: item.category.clone(),
                quantities: vec![],
                recipe_ids: vec![],
            })
            .quantities.push((adjusted_qty, item.unit));

        let entry = grouped.get_mut(&key).unwrap();
        if !entry.recipe_ids.contains(&item.recipe_id) {
            entry.recipe_ids.push(item.recipe_id.clone());
        }
    }

    // Aggregate quantities using unit conversion
    let mut result = vec![];
    for (name, group) in grouped {
        let aggregated = aggregate_quantities(&group.quantities);

        for agg in aggregated {
            result.push(AggregatedShoppingItem {
                name: name.clone(),
                quantity: agg.quantity,
                unit: agg.unit,
                category: group.category.clone(),
                source_recipe_ids: group.recipe_ids.clone(),
                is_converted: agg.is_converted,
            });
        }
    }

    // Sort by category then name
    result.sort_by(|a, b| {
        a.category.cmp(&b.category).then_with(|| a.name.cmp(&b.name))
    });

    Ok(result)
}
```

#### Commands Module: `src-tauri/src/commands/shopping_lists.rs`

**File**: `src-tauri/src/commands/shopping_lists.rs`

```rust
//! Shopping list command handlers

use crate::db::shopping_lists::{self, *};
use tauri::command;

#[command]
pub async fn get_shopping_lists(week_start: String) -> Result<Vec<ShoppingListWithItems>, String> {
    shopping_lists::get_shopping_lists(&week_start)
        .await
        .map_err(|e| e.into())
}

#[command]
pub async fn create_shopping_list(input: ShoppingListInput) -> Result<ShoppingList, String> {
    shopping_lists::create_shopping_list(input)
        .await
        .map_err(|e| e.into())
}

#[command]
pub async fn delete_shopping_list(id: String) -> Result<(), String> {
    shopping_lists::delete_shopping_list(&id)
        .await
        .map_err(|e| e.into())
}

#[command]
pub async fn add_shopping_item(input: ShoppingItemInput) -> Result<ShoppingListItem, String> {
    shopping_lists::add_shopping_item(input)
        .await
        .map_err(|e| e.into())
}

#[command]
pub async fn update_shopping_item(
    id: String,
    quantity: Option<f64>,
    is_checked: Option<bool>,
) -> Result<ShoppingListItem, String> {
    shopping_lists::update_shopping_item(&id, quantity, is_checked)
        .await
        .map_err(|e| e.into())
}

#[command]
pub async fn soft_delete_shopping_item(id: String) -> Result<(), String> {
    shopping_lists::soft_delete_shopping_item(&id)
        .await
        .map_err(|e| e.into())
}

#[command]
pub async fn restore_shopping_item(id: String) -> Result<ShoppingListItem, String> {
    shopping_lists::restore_shopping_item(&id)
        .await
        .map_err(|e| e.into())
}

#[command]
pub async fn move_shopping_item(id: String, to_list_id: String) -> Result<ShoppingListItem, String> {
    shopping_lists::move_shopping_item(&id, &to_list_id)
        .await
        .map_err(|e| e.into())
}

#[command]
pub async fn get_aggregated_shopping_list(
    start_date: String,
    end_date: String,
) -> Result<Vec<AggregatedShoppingItem>, String> {
    shopping_lists::get_aggregated_shopping_list(&start_date, &end_date)
        .await
        .map_err(|e| e.into())
}
```

### Success Criteria

#### Automated Verification
- [x] Migration runs
- [x] All shopping list tests pass
- [x] Aggregation with unit conversion works

#### Manual Verification
- [ ] Can create shopping lists
- [ ] Can add/update/delete items
- [ ] Aggregated list shows combined quantities

**Checkpoint**: Pause for verification before Phase 5.

---

## Phase 5: Quick Lists

### Goal

Create quick lists schema and CRUD with ability to copy to shopping lists.

### Integration Points

**Depends on**: Phase 4 (shopping_lists for copy feature)
**Produces**: Quick list functionality

### Changes

#### Migration: `20250115000004_quick_lists_schema.sql`

**File**: `src-tauri/migrations/20250115000004_quick_lists_schema.sql`

```sql
-- Quick lists schema

CREATE TABLE IF NOT EXISTS quick_lists (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE TABLE IF NOT EXISTS quick_list_items (
    id TEXT PRIMARY KEY,
    quick_list_id TEXT NOT NULL,
    name TEXT NOT NULL,
    quantity REAL NOT NULL DEFAULT 1,
    unit TEXT NOT NULL DEFAULT '',
    category TEXT NOT NULL DEFAULT 'Other',
    FOREIGN KEY (quick_list_id) REFERENCES quick_lists(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_quick_list_items_list ON quick_list_items(quick_list_id);
```

#### DB Module: `src-tauri/src/db/quick_lists.rs`

```rust
//! Quick list database operations

use crate::db::pool::get_db_pool;
use crate::db::shopping_lists::{add_shopping_item, ShoppingItemInput};
use crate::error::AppError;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct QuickList {
    pub id: String,
    pub name: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct QuickListItem {
    pub id: String,
    pub quick_list_id: String,
    pub name: String,
    pub quantity: f64,
    pub unit: String,
    pub category: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QuickListWithItems {
    #[serde(flatten)]
    pub list: QuickList,
    pub items: Vec<QuickListItem>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuickListItemInput {
    pub name: String,
    pub quantity: f64,
    pub unit: String,
    pub category: String,
}

/// Get all quick lists with items
pub async fn get_quick_lists() -> Result<Vec<QuickListWithItems>, AppError> {
    let pool = get_db_pool();

    let lists = sqlx::query_as::<_, QuickList>(
        "SELECT id, name, created_at, updated_at FROM quick_lists ORDER BY name"
    )
    .fetch_all(pool)
    .await
    .map_err(|e| AppError::Database(e.to_string()))?;

    let mut result = vec![];
    for list in lists {
        let items = sqlx::query_as::<_, QuickListItem>(
            "SELECT id, quick_list_id, name, quantity, unit, category
             FROM quick_list_items WHERE quick_list_id = ?
             ORDER BY category, name"
        )
        .bind(&list.id)
        .fetch_all(pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        result.push(QuickListWithItems { list, items });
    }

    Ok(result)
}

/// Create a quick list
pub async fn create_quick_list(name: &str) -> Result<QuickList, AppError> {
    let pool = get_db_pool();
    let id = Uuid::new_v4().to_string();

    sqlx::query("INSERT INTO quick_lists (id, name) VALUES (?, ?)")
        .bind(&id)
        .bind(name)
        .execute(pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

    sqlx::query_as::<_, QuickList>(
        "SELECT id, name, created_at, updated_at FROM quick_lists WHERE id = ?"
    )
    .bind(&id)
    .fetch_one(pool)
    .await
    .map_err(|e| AppError::Database(e.to_string()))
}

/// Update (rename) a quick list
pub async fn update_quick_list(id: &str, name: &str) -> Result<QuickList, AppError> {
    let pool = get_db_pool();

    let result = sqlx::query(
        "UPDATE quick_lists SET name = ?, updated_at = datetime('now') WHERE id = ?"
    )
    .bind(name)
    .bind(id)
    .execute(pool)
    .await
    .map_err(|e| AppError::Database(e.to_string()))?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound(format!("Quick list with id {id} not found")));
    }

    sqlx::query_as::<_, QuickList>(
        "SELECT id, name, created_at, updated_at FROM quick_lists WHERE id = ?"
    )
    .bind(id)
    .fetch_one(pool)
    .await
    .map_err(|e| AppError::Database(e.to_string()))
}

/// Delete a quick list
pub async fn delete_quick_list(id: &str) -> Result<(), AppError> {
    let pool = get_db_pool();

    let result = sqlx::query("DELETE FROM quick_lists WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound(format!("Quick list with id {id} not found")));
    }

    Ok(())
}

/// Add item to quick list
pub async fn add_quick_list_item(
    quick_list_id: &str,
    input: QuickListItemInput,
) -> Result<QuickListItem, AppError> {
    let pool = get_db_pool();
    let id = Uuid::new_v4().to_string();

    sqlx::query(
        "INSERT INTO quick_list_items (id, quick_list_id, name, quantity, unit, category)
         VALUES (?, ?, ?, ?, ?, ?)"
    )
    .bind(&id)
    .bind(quick_list_id)
    .bind(&input.name)
    .bind(input.quantity)
    .bind(&input.unit)
    .bind(&input.category)
    .execute(pool)
    .await
    .map_err(|e| AppError::Database(e.to_string()))?;

    // Update parent timestamp
    sqlx::query("UPDATE quick_lists SET updated_at = datetime('now') WHERE id = ?")
        .bind(quick_list_id)
        .execute(pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

    sqlx::query_as::<_, QuickListItem>(
        "SELECT id, quick_list_id, name, quantity, unit, category
         FROM quick_list_items WHERE id = ?"
    )
    .bind(&id)
    .fetch_one(pool)
    .await
    .map_err(|e| AppError::Database(e.to_string()))
}

/// Update quick list item
pub async fn update_quick_list_item(
    id: &str,
    input: QuickListItemInput,
) -> Result<QuickListItem, AppError> {
    let pool = get_db_pool();

    let result = sqlx::query(
        "UPDATE quick_list_items SET name = ?, quantity = ?, unit = ?, category = ? WHERE id = ?"
    )
    .bind(&input.name)
    .bind(input.quantity)
    .bind(&input.unit)
    .bind(&input.category)
    .bind(id)
    .execute(pool)
    .await
    .map_err(|e| AppError::Database(e.to_string()))?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound(format!("Quick list item with id {id} not found")));
    }

    sqlx::query_as::<_, QuickListItem>(
        "SELECT id, quick_list_id, name, quantity, unit, category
         FROM quick_list_items WHERE id = ?"
    )
    .bind(id)
    .fetch_one(pool)
    .await
    .map_err(|e| AppError::Database(e.to_string()))
}

/// Remove item from quick list
pub async fn remove_quick_list_item(id: &str) -> Result<(), AppError> {
    let pool = get_db_pool();

    let result = sqlx::query("DELETE FROM quick_list_items WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound(format!("Quick list item with id {id} not found")));
    }

    Ok(())
}

/// Copy quick list items to a shopping list
pub async fn add_quick_list_to_shopping(
    quick_list_id: &str,
    shopping_list_id: &str,
) -> Result<Vec<crate::db::shopping_lists::ShoppingListItem>, AppError> {
    let pool = get_db_pool();

    let items = sqlx::query_as::<_, QuickListItem>(
        "SELECT id, quick_list_id, name, quantity, unit, category
         FROM quick_list_items WHERE quick_list_id = ?"
    )
    .bind(quick_list_id)
    .fetch_all(pool)
    .await
    .map_err(|e| AppError::Database(e.to_string()))?;

    let mut added = vec![];
    for item in items {
        let input = ShoppingItemInput {
            list_id: shopping_list_id.to_string(),
            name: item.name,
            quantity: item.quantity,
            unit: item.unit,
            category: item.category,
        };
        let shopping_item = add_shopping_item(input).await?;
        added.push(shopping_item);
    }

    Ok(added)
}
```

#### Commands and IPC

Similar pattern to previous phases - add command handlers and IPC wrappers.

### Success Criteria

- [x] Quick list CRUD works
- [x] Can copy quick list to shopping list

**Checkpoint**: Pause for verification before Phase 6.

---

## Phase 6: Manual Items

### Goal

Create manual shopping items (items added directly, not from recipes).

### Changes

#### Migration: `20250115000005_manual_items_schema.sql`

```sql
-- Manual shopping items schema

CREATE TABLE IF NOT EXISTS manual_shopping_items (
    id TEXT PRIMARY KEY,
    week_start TEXT NOT NULL,
    name TEXT NOT NULL,
    quantity REAL NOT NULL DEFAULT 1,
    unit TEXT NOT NULL DEFAULT '',
    category TEXT NOT NULL DEFAULT 'Other',
    is_checked INTEGER NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE INDEX IF NOT EXISTS idx_manual_items_week ON manual_shopping_items(week_start);
```

#### DB Module and Commands

Follow same pattern as previous phases.

### Success Criteria

- [x] Manual item CRUD works
- [x] Items associated with correct week

---

## Final Integration Checklist

After all phases complete:

- [x] All migrations run in order
- [x] `pnpm test:rust` passes (16 tests)
- [x] `cargo clippy` clean
- [x] `pnpm check` passes (0 errors)
- [x] All commands registered in `lib.rs`
- [x] All IPC wrappers exported
- [ ] Can perform full workflow: create recipe → add to meal plan → see aggregated shopping list

---

## Test Helper: `init_db_for_test`

Add to `src-tauri/src/db/pool.rs`:

```rust
#[cfg(test)]
pub async fn init_db_for_test() {
    use std::sync::Once;
    static INIT: Once = Once::new();

    INIT.call_once(|| {
        tokio::runtime::Runtime::new().unwrap().block_on(async {
            // Use in-memory database for tests
            let pool = SqlitePool::connect(":memory:").await.unwrap();

            // Run migrations
            sqlx::migrate!("./migrations")
                .run(&pool)
                .await
                .unwrap();

            // Store pool (you'll need a test-specific global or pass it around)
        });
    });
}
```

Note: The exact implementation depends on how you want to handle test isolation. Consider using a test harness that creates fresh databases per test.
