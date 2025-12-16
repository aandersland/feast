//! Recipe database operations

use crate::db::ingredients::get_or_create_ingredient;
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
         FROM recipes ORDER BY created_at DESC",
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

/// Create a new recipe with ingredients
pub async fn create_recipe(input: RecipeInput) -> Result<Recipe, AppError> {
    let pool = get_db_pool();
    let recipe_id = Uuid::new_v4().to_string();

    // Insert recipe
    sqlx::query(
        "INSERT INTO recipes (id, name, description, prep_time, cook_time, servings,
                              image_path, source_url, notes)
         VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)",
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
             VALUES (?, ?, ?, ?, ?, ?, ?)",
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
             VALUES (?, ?, ?, ?, ?, ?, ?)",
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
            ingredients: vec![IngredientInput {
                name: "Chicken".to_string(),
                quantity: 1.0,
                unit: "lb".to_string(),
                category: Some("Meat & Seafood".to_string()),
                notes: None,
            }],
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
            ingredients: vec![IngredientInput {
                name: "Ingredient".to_string(),
                quantity: 1.0,
                unit: "cup".to_string(),
                category: None,
                notes: None,
            }],
            instructions: vec![],
        };

        let recipe = create_recipe(input).await.unwrap();
        delete_recipe(&recipe.id).await.unwrap();

        let result = get_recipe_by_id(&recipe.id).await;
        assert!(result.is_err());
    }
}
