//! Shopping list command handlers

use crate::correlation::ensure_correlation_id;
use crate::db::shopping_lists::{self, *};
use crate::logging::redact;
use std::time::Instant;
use tauri::command;

#[command]
pub async fn get_shopping_lists(week_start: String, correlation_id: Option<String>) -> Result<Vec<ShoppingListWithItems>, String> {
    let cid = ensure_correlation_id(correlation_id);
    let start = Instant::now();
    log::debug!("[cid:{}] get_shopping_lists called, week_start={}", cid, week_start);

    let result = shopping_lists::get_shopping_lists(&week_start)
        .await
        .map_err(|e| e.to_string());

    let elapsed = start.elapsed();
    match &result {
        Ok(lists) => log::info!(
            "[cid:{}] get_shopping_lists succeeded in {:?}, {}",
            cid, elapsed, redact::format_count(lists.len(), "list")
        ),
        Err(e) => log::error!("[cid:{}] get_shopping_lists failed in {:?}: {}", cid, elapsed, e),
    }
    result
}

#[command]
pub async fn create_shopping_list(input: ShoppingListInput, correlation_id: Option<String>) -> Result<ShoppingList, String> {
    let cid = ensure_correlation_id(correlation_id);
    let start = Instant::now();
    log::debug!(
        "[cid:{}] create_shopping_list called, {}, week_start={}",
        cid,
        redact::redact_string(Some(&input.name), "name"),
        input.week_start
    );

    let result = shopping_lists::create_shopping_list(input)
        .await
        .map_err(|e| e.to_string());

    let elapsed = start.elapsed();
    match &result {
        Ok(list) => log::info!("[cid:{}] create_shopping_list succeeded in {:?}, id={}", cid, elapsed, list.id),
        Err(e) => log::error!("[cid:{}] create_shopping_list failed in {:?}: {}", cid, elapsed, e),
    }
    result
}

#[command]
pub async fn delete_shopping_list(id: String, correlation_id: Option<String>) -> Result<(), String> {
    let cid = ensure_correlation_id(correlation_id);
    let start = Instant::now();
    log::debug!("[cid:{}] delete_shopping_list called, id={}", cid, id);

    let result = shopping_lists::delete_shopping_list(&id)
        .await
        .map_err(|e| e.to_string());

    let elapsed = start.elapsed();
    match &result {
        Ok(()) => log::info!("[cid:{}] delete_shopping_list succeeded in {:?}, id={}", cid, elapsed, id),
        Err(e) => log::error!("[cid:{}] delete_shopping_list failed in {:?}: {}", cid, elapsed, e),
    }
    result
}

#[command]
pub async fn add_shopping_item(input: ShoppingItemInput, correlation_id: Option<String>) -> Result<ShoppingListItem, String> {
    let cid = ensure_correlation_id(correlation_id);
    let start = Instant::now();
    log::debug!(
        "[cid:{}] add_shopping_item called, {}, list_id={}",
        cid,
        redact::redact_string(Some(&input.name), "name"),
        input.list_id
    );

    let result = shopping_lists::add_shopping_item(input)
        .await
        .map_err(|e| e.to_string());

    let elapsed = start.elapsed();
    match &result {
        Ok(item) => log::info!("[cid:{}] add_shopping_item succeeded in {:?}, id={}", cid, elapsed, item.id),
        Err(e) => log::error!("[cid:{}] add_shopping_item failed in {:?}: {}", cid, elapsed, e),
    }
    result
}

#[command]
pub async fn update_shopping_item(
    id: String,
    quantity: Option<f64>,
    is_checked: Option<bool>,
    correlation_id: Option<String>,
) -> Result<ShoppingListItem, String> {
    let cid = ensure_correlation_id(correlation_id);
    let start = Instant::now();
    log::debug!(
        "[cid:{}] update_shopping_item called, id={}, quantity={:?}, is_checked={:?}",
        cid, id, quantity, is_checked
    );

    let result = shopping_lists::update_shopping_item(&id, quantity, is_checked)
        .await
        .map_err(|e| e.to_string());

    let elapsed = start.elapsed();
    match &result {
        Ok(item) => log::info!("[cid:{}] update_shopping_item succeeded in {:?}, id={}", cid, elapsed, item.id),
        Err(e) => log::error!("[cid:{}] update_shopping_item failed in {:?}: {}", cid, elapsed, e),
    }
    result
}

#[command]
pub async fn soft_delete_shopping_item(id: String, correlation_id: Option<String>) -> Result<(), String> {
    let cid = ensure_correlation_id(correlation_id);
    let start = Instant::now();
    log::debug!("[cid:{}] soft_delete_shopping_item called, id={}", cid, id);

    let result = shopping_lists::soft_delete_shopping_item(&id)
        .await
        .map_err(|e| e.to_string());

    let elapsed = start.elapsed();
    match &result {
        Ok(()) => log::info!("[cid:{}] soft_delete_shopping_item succeeded in {:?}, id={}", cid, elapsed, id),
        Err(e) => log::error!("[cid:{}] soft_delete_shopping_item failed in {:?}: {}", cid, elapsed, e),
    }
    result
}

#[command]
pub async fn restore_shopping_item(id: String, correlation_id: Option<String>) -> Result<ShoppingListItem, String> {
    let cid = ensure_correlation_id(correlation_id);
    let start = Instant::now();
    log::debug!("[cid:{}] restore_shopping_item called, id={}", cid, id);

    let result = shopping_lists::restore_shopping_item(&id)
        .await
        .map_err(|e| e.to_string());

    let elapsed = start.elapsed();
    match &result {
        Ok(item) => log::info!("[cid:{}] restore_shopping_item succeeded in {:?}, id={}", cid, elapsed, item.id),
        Err(e) => log::error!("[cid:{}] restore_shopping_item failed in {:?}: {}", cid, elapsed, e),
    }
    result
}

#[command]
pub async fn move_shopping_item(
    id: String,
    to_list_id: String,
    correlation_id: Option<String>,
) -> Result<ShoppingListItem, String> {
    let cid = ensure_correlation_id(correlation_id);
    let start = Instant::now();
    log::debug!("[cid:{}] move_shopping_item called, id={}, to_list_id={}", cid, id, to_list_id);

    let result = shopping_lists::move_shopping_item(&id, &to_list_id)
        .await
        .map_err(|e| e.to_string());

    let elapsed = start.elapsed();
    match &result {
        Ok(item) => log::info!("[cid:{}] move_shopping_item succeeded in {:?}, id={}", cid, elapsed, item.id),
        Err(e) => log::error!("[cid:{}] move_shopping_item failed in {:?}: {}", cid, elapsed, e),
    }
    result
}

#[command]
pub async fn get_aggregated_shopping_list(
    start_date: String,
    end_date: String,
    correlation_id: Option<String>,
) -> Result<Vec<AggregatedShoppingItem>, String> {
    let cid = ensure_correlation_id(correlation_id);
    let start = Instant::now();
    log::debug!(
        "[cid:{}] get_aggregated_shopping_list called, start_date={}, end_date={}",
        cid, start_date, end_date
    );

    let result = shopping_lists::get_aggregated_shopping_list(&start_date, &end_date)
        .await
        .map_err(|e| e.to_string());

    let elapsed = start.elapsed();
    match &result {
        Ok(items) => log::info!(
            "[cid:{}] get_aggregated_shopping_list succeeded in {:?}, {}",
            cid, elapsed, redact::format_count(items.len(), "item")
        ),
        Err(e) => log::error!("[cid:{}] get_aggregated_shopping_list failed in {:?}: {}", cid, elapsed, e),
    }
    result
}
