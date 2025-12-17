//! Ingredient command handlers

use crate::db::ingredients::{self, Ingredient};
use tauri::command;

/// Get all ingredients
#[command]
pub async fn get_ingredients() -> Result<Vec<Ingredient>, String> {
    ingredients::get_all_ingredients()
        .await
        .map_err(|e| e.into())
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
