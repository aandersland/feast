//! Item command handlers

use crate::correlation::ensure_correlation_id;
use crate::db::items::{self, Item};
use crate::logging::redact;
use std::time::Instant;
use tauri::command;

/// Get all items
#[command]
pub async fn get_items(correlation_id: Option<String>) -> Result<Vec<Item>, String> {
    let cid = ensure_correlation_id(correlation_id);
    let start = Instant::now();
    log::debug!("[cid:{}] get_items called", cid);

    let result = items::get_all_items().await.map_err(|e| e.into());

    let elapsed = start.elapsed();
    match &result {
        Ok(items) => log::info!(
            "[cid:{}] get_items succeeded in {:?}, {}",
            cid, elapsed, redact::format_count(items.len(), "item")
        ),
        Err(e) => log::error!("[cid:{}] get_items failed in {:?}: {}", cid, elapsed, e),
    }
    result
}

/// Create a new item
#[command]
pub async fn create_item(name: String, correlation_id: Option<String>) -> Result<Item, String> {
    let cid = ensure_correlation_id(correlation_id);
    let start = Instant::now();
    log::debug!("[cid:{}] create_item called, {}", cid, redact::redact_string(Some(&name), "name"));

    let result = items::create_new_item(&name).await.map_err(|e| e.into());

    let elapsed = start.elapsed();
    match &result {
        Ok(item) => log::info!("[cid:{}] create_item succeeded in {:?}, id={}", cid, elapsed, item.id),
        Err(e) => log::error!("[cid:{}] create_item failed in {:?}: {}", cid, elapsed, e),
    }
    result
}

/// Delete an item
#[command]
pub async fn delete_item(id: i64, correlation_id: Option<String>) -> Result<(), String> {
    let cid = ensure_correlation_id(correlation_id);
    let start = Instant::now();
    log::debug!("[cid:{}] delete_item called, id={}", cid, id);

    let result = items::delete_item_by_id(id).await.map_err(|e| e.into());

    let elapsed = start.elapsed();
    match &result {
        Ok(()) => log::info!("[cid:{}] delete_item succeeded in {:?}, id={}", cid, elapsed, id),
        Err(e) => log::error!("[cid:{}] delete_item failed in {:?}: {}", cid, elapsed, e),
    }
    result
}
