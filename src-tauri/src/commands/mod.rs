//! Tauri command handlers

pub mod ingredients;
pub mod items;
pub mod logging;
pub mod manual_items;
pub mod meal_plans;
pub mod quick_lists;
pub mod recipes;
pub mod shopping_lists;

use crate::correlation::ensure_correlation_id;
use crate::logging::redact;
use std::time::Instant;
use tauri::command;

pub use ingredients::{create_ingredient, get_ingredients, get_or_create_ingredient};
pub use items::{create_item, delete_item, get_items};
pub use manual_items::{
    create_manual_item, delete_manual_item, get_manual_items, update_manual_item,
};
pub use meal_plans::{create_meal_plan, delete_meal_plan, get_meal_plans, update_meal_plan};
pub use quick_lists::{
    add_quick_list_item, add_quick_list_to_shopping, create_quick_list, delete_quick_list,
    get_quick_lists, remove_quick_list_item, update_quick_list, update_quick_list_item,
};
pub use recipes::{create_recipe, delete_recipe, get_recipe, get_recipes, import_recipe_from_url, update_recipe};
pub use shopping_lists::{
    add_shopping_item, create_shopping_list, delete_shopping_list, get_aggregated_shopping_list,
    get_shopping_lists, move_shopping_item, restore_shopping_item, soft_delete_shopping_item,
    update_shopping_item,
};
pub use logging::log_from_frontend;

/// Greet a user by name
#[command]
#[must_use]
pub fn greet(name: &str, correlation_id: Option<String>) -> String {
    let cid = ensure_correlation_id(correlation_id);
    let start = Instant::now();
    log::debug!("[cid:{}] greet called, {}", cid, redact::redact_string(Some(name), "name"));

    let result = format!("Hello, {name}! Welcome to feast.");

    let elapsed = start.elapsed();
    log::info!("[cid:{}] greet succeeded in {:?}", cid, elapsed);
    result
}
