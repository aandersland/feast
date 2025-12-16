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
