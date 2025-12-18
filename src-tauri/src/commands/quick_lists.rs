//! Quick list command handlers

use crate::correlation::ensure_correlation_id;
use crate::db::quick_lists::{self, *};
use crate::db::shopping_lists::ShoppingListItem;
use crate::logging::redact;
use std::time::Instant;
use tauri::command;

#[command]
pub async fn get_quick_lists(correlation_id: Option<String>) -> Result<Vec<QuickListWithItems>, String> {
    let cid = ensure_correlation_id(correlation_id);
    let start = Instant::now();
    log::debug!("[cid:{}] get_quick_lists called", cid);

    let result = quick_lists::get_quick_lists()
        .await
        .map_err(|e| e.to_string());

    let elapsed = start.elapsed();
    match &result {
        Ok(lists) => log::info!(
            "[cid:{}] get_quick_lists succeeded in {:?}, {}",
            cid, elapsed, redact::format_count(lists.len(), "list")
        ),
        Err(e) => log::error!("[cid:{}] get_quick_lists failed in {:?}: {}", cid, elapsed, e),
    }
    result
}

#[command]
pub async fn create_quick_list(name: String, correlation_id: Option<String>) -> Result<QuickList, String> {
    let cid = ensure_correlation_id(correlation_id);
    let start = Instant::now();
    log::debug!("[cid:{}] create_quick_list called, {}", cid, redact::redact_string(Some(&name), "name"));

    let result = quick_lists::create_quick_list(&name)
        .await
        .map_err(|e| e.to_string());

    let elapsed = start.elapsed();
    match &result {
        Ok(list) => log::info!("[cid:{}] create_quick_list succeeded in {:?}, id={}", cid, elapsed, list.id),
        Err(e) => log::error!("[cid:{}] create_quick_list failed in {:?}: {}", cid, elapsed, e),
    }
    result
}

#[command]
pub async fn update_quick_list(id: String, name: String, correlation_id: Option<String>) -> Result<QuickList, String> {
    let cid = ensure_correlation_id(correlation_id);
    let start = Instant::now();
    log::debug!("[cid:{}] update_quick_list called, id={}, {}", cid, id, redact::redact_string(Some(&name), "name"));

    let result = quick_lists::update_quick_list(&id, &name)
        .await
        .map_err(|e| e.to_string());

    let elapsed = start.elapsed();
    match &result {
        Ok(list) => log::info!("[cid:{}] update_quick_list succeeded in {:?}, id={}", cid, elapsed, list.id),
        Err(e) => log::error!("[cid:{}] update_quick_list failed in {:?}: {}", cid, elapsed, e),
    }
    result
}

#[command]
pub async fn delete_quick_list(id: String, correlation_id: Option<String>) -> Result<(), String> {
    let cid = ensure_correlation_id(correlation_id);
    let start = Instant::now();
    log::debug!("[cid:{}] delete_quick_list called, id={}", cid, id);

    let result = quick_lists::delete_quick_list(&id)
        .await
        .map_err(|e| e.to_string());

    let elapsed = start.elapsed();
    match &result {
        Ok(()) => log::info!("[cid:{}] delete_quick_list succeeded in {:?}, id={}", cid, elapsed, id),
        Err(e) => log::error!("[cid:{}] delete_quick_list failed in {:?}: {}", cid, elapsed, e),
    }
    result
}

#[command]
pub async fn add_quick_list_item(
    quick_list_id: String,
    input: QuickListItemInput,
    correlation_id: Option<String>,
) -> Result<QuickListItem, String> {
    let cid = ensure_correlation_id(correlation_id);
    let start = Instant::now();
    log::debug!(
        "[cid:{}] add_quick_list_item called, quick_list_id={}, {}",
        cid, quick_list_id, redact::redact_string(Some(&input.name), "name")
    );

    let result = quick_lists::add_quick_list_item(&quick_list_id, input)
        .await
        .map_err(|e| e.to_string());

    let elapsed = start.elapsed();
    match &result {
        Ok(item) => log::info!("[cid:{}] add_quick_list_item succeeded in {:?}, id={}", cid, elapsed, item.id),
        Err(e) => log::error!("[cid:{}] add_quick_list_item failed in {:?}: {}", cid, elapsed, e),
    }
    result
}

#[command]
pub async fn update_quick_list_item(
    id: String,
    input: QuickListItemInput,
    correlation_id: Option<String>,
) -> Result<QuickListItem, String> {
    let cid = ensure_correlation_id(correlation_id);
    let start = Instant::now();
    log::debug!(
        "[cid:{}] update_quick_list_item called, id={}, {}",
        cid, id, redact::redact_string(Some(&input.name), "name")
    );

    let result = quick_lists::update_quick_list_item(&id, input)
        .await
        .map_err(|e| e.to_string());

    let elapsed = start.elapsed();
    match &result {
        Ok(item) => log::info!("[cid:{}] update_quick_list_item succeeded in {:?}, id={}", cid, elapsed, item.id),
        Err(e) => log::error!("[cid:{}] update_quick_list_item failed in {:?}: {}", cid, elapsed, e),
    }
    result
}

#[command]
pub async fn remove_quick_list_item(id: String, correlation_id: Option<String>) -> Result<(), String> {
    let cid = ensure_correlation_id(correlation_id);
    let start = Instant::now();
    log::debug!("[cid:{}] remove_quick_list_item called, id={}", cid, id);

    let result = quick_lists::remove_quick_list_item(&id)
        .await
        .map_err(|e| e.to_string());

    let elapsed = start.elapsed();
    match &result {
        Ok(()) => log::info!("[cid:{}] remove_quick_list_item succeeded in {:?}, id={}", cid, elapsed, id),
        Err(e) => log::error!("[cid:{}] remove_quick_list_item failed in {:?}: {}", cid, elapsed, e),
    }
    result
}

#[command]
pub async fn add_quick_list_to_shopping(
    quick_list_id: String,
    shopping_list_id: String,
    correlation_id: Option<String>,
) -> Result<Vec<ShoppingListItem>, String> {
    let cid = ensure_correlation_id(correlation_id);
    let start = Instant::now();
    log::debug!(
        "[cid:{}] add_quick_list_to_shopping called, quick_list_id={}, shopping_list_id={}",
        cid, quick_list_id, shopping_list_id
    );

    let result = quick_lists::add_quick_list_to_shopping(&quick_list_id, &shopping_list_id)
        .await
        .map_err(|e| e.to_string());

    let elapsed = start.elapsed();
    match &result {
        Ok(items) => log::info!(
            "[cid:{}] add_quick_list_to_shopping succeeded in {:?}, {}",
            cid, elapsed, redact::format_count(items.len(), "item")
        ),
        Err(e) => log::error!("[cid:{}] add_quick_list_to_shopping failed in {:?}: {}", cid, elapsed, e),
    }
    result
}
