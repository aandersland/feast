//! Manual shopping item command handlers

use crate::db::manual_items::{self, ManualItem, ManualItemInput};
use tauri::command;

/// Get manual items for a week
#[command]
pub async fn get_manual_items(week_start: String) -> Result<Vec<ManualItem>, String> {
    manual_items::get_manual_items(&week_start)
        .await
        .map_err(|e| e.to_string())
}

/// Create a manual item
#[command]
pub async fn create_manual_item(input: ManualItemInput) -> Result<ManualItem, String> {
    manual_items::create_manual_item(input)
        .await
        .map_err(|e| e.to_string())
}

/// Update a manual item
#[command]
pub async fn update_manual_item(
    id: String,
    quantity: Option<f64>,
    is_checked: Option<bool>,
) -> Result<ManualItem, String> {
    manual_items::update_manual_item(&id, quantity, is_checked)
        .await
        .map_err(|e| e.to_string())
}

/// Delete a manual item
#[command]
pub async fn delete_manual_item(id: String) -> Result<(), String> {
    manual_items::delete_manual_item(&id)
        .await
        .map_err(|e| e.to_string())
}
