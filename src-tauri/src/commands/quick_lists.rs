//! Quick list command handlers

use crate::db::quick_lists::{self, *};
use crate::db::shopping_lists::ShoppingListItem;
use tauri::command;

#[command]
pub async fn get_quick_lists() -> Result<Vec<QuickListWithItems>, String> {
    quick_lists::get_quick_lists()
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn create_quick_list(name: String) -> Result<QuickList, String> {
    quick_lists::create_quick_list(&name)
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn update_quick_list(id: String, name: String) -> Result<QuickList, String> {
    quick_lists::update_quick_list(&id, &name)
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn delete_quick_list(id: String) -> Result<(), String> {
    quick_lists::delete_quick_list(&id)
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn add_quick_list_item(
    quick_list_id: String,
    input: QuickListItemInput,
) -> Result<QuickListItem, String> {
    quick_lists::add_quick_list_item(&quick_list_id, input)
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn update_quick_list_item(
    id: String,
    input: QuickListItemInput,
) -> Result<QuickListItem, String> {
    quick_lists::update_quick_list_item(&id, input)
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn remove_quick_list_item(id: String) -> Result<(), String> {
    quick_lists::remove_quick_list_item(&id)
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn add_quick_list_to_shopping(
    quick_list_id: String,
    shopping_list_id: String,
) -> Result<Vec<ShoppingListItem>, String> {
    quick_lists::add_quick_list_to_shopping(&quick_list_id, &shopping_list_id)
        .await
        .map_err(|e| e.to_string())
}
