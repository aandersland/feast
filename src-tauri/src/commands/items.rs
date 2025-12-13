//! Item command handlers

use crate::db::items::{self, Item};
use tauri::command;

/// Get all items
#[command]
pub async fn get_items() -> Result<Vec<Item>, String> {
    items::get_all_items().await.map_err(|e| e.into())
}

/// Create a new item
#[command]
pub async fn create_item(name: String) -> Result<Item, String> {
    items::create_new_item(&name).await.map_err(|e| e.into())
}

/// Delete an item
#[command]
pub async fn delete_item(id: i64) -> Result<(), String> {
    items::delete_item_by_id(id).await.map_err(|e| e.into())
}
