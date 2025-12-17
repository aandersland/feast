//! Shopping list command handlers

use crate::db::shopping_lists::{self, *};
use tauri::command;

#[command]
pub async fn get_shopping_lists(week_start: String) -> Result<Vec<ShoppingListWithItems>, String> {
    shopping_lists::get_shopping_lists(&week_start)
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn create_shopping_list(input: ShoppingListInput) -> Result<ShoppingList, String> {
    shopping_lists::create_shopping_list(input)
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn delete_shopping_list(id: String) -> Result<(), String> {
    shopping_lists::delete_shopping_list(&id)
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn add_shopping_item(input: ShoppingItemInput) -> Result<ShoppingListItem, String> {
    shopping_lists::add_shopping_item(input)
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn update_shopping_item(
    id: String,
    quantity: Option<f64>,
    is_checked: Option<bool>,
) -> Result<ShoppingListItem, String> {
    shopping_lists::update_shopping_item(&id, quantity, is_checked)
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn soft_delete_shopping_item(id: String) -> Result<(), String> {
    shopping_lists::soft_delete_shopping_item(&id)
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn restore_shopping_item(id: String) -> Result<ShoppingListItem, String> {
    shopping_lists::restore_shopping_item(&id)
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn move_shopping_item(
    id: String,
    to_list_id: String,
) -> Result<ShoppingListItem, String> {
    shopping_lists::move_shopping_item(&id, &to_list_id)
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn get_aggregated_shopping_list(
    start_date: String,
    end_date: String,
) -> Result<Vec<AggregatedShoppingItem>, String> {
    shopping_lists::get_aggregated_shopping_list(&start_date, &end_date)
        .await
        .map_err(|e| e.to_string())
}
