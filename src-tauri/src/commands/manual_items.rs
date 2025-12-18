//! Manual shopping item command handlers

use crate::correlation::ensure_correlation_id;
use crate::db::manual_items::{self, ManualItem, ManualItemInput};
use crate::logging::redact;
use std::time::Instant;
use tauri::command;

/// Get manual items for a week
#[command]
pub async fn get_manual_items(week_start: String, correlation_id: Option<String>) -> Result<Vec<ManualItem>, String> {
    let cid = ensure_correlation_id(correlation_id);
    let start = Instant::now();
    log::debug!("[cid:{}] get_manual_items called, week_start={}", cid, week_start);

    let result = manual_items::get_manual_items(&week_start)
        .await
        .map_err(|e| e.to_string());

    let elapsed = start.elapsed();
    match &result {
        Ok(items) => log::info!(
            "[cid:{}] get_manual_items succeeded in {:?}, {}",
            cid, elapsed, redact::format_count(items.len(), "item")
        ),
        Err(e) => log::error!("[cid:{}] get_manual_items failed in {:?}: {}", cid, elapsed, e),
    }
    result
}

/// Create a manual item
#[command]
pub async fn create_manual_item(input: ManualItemInput, correlation_id: Option<String>) -> Result<ManualItem, String> {
    let cid = ensure_correlation_id(correlation_id);
    let start = Instant::now();
    log::debug!(
        "[cid:{}] create_manual_item called, {}, week_start={}",
        cid,
        redact::redact_string(Some(&input.name), "name"),
        input.week_start
    );

    let result = manual_items::create_manual_item(input)
        .await
        .map_err(|e| e.to_string());

    let elapsed = start.elapsed();
    match &result {
        Ok(item) => log::info!("[cid:{}] create_manual_item succeeded in {:?}, id={}", cid, elapsed, item.id),
        Err(e) => log::error!("[cid:{}] create_manual_item failed in {:?}: {}", cid, elapsed, e),
    }
    result
}

/// Update a manual item
#[command]
pub async fn update_manual_item(
    id: String,
    quantity: Option<f64>,
    is_checked: Option<bool>,
    correlation_id: Option<String>,
) -> Result<ManualItem, String> {
    let cid = ensure_correlation_id(correlation_id);
    let start = Instant::now();
    log::debug!(
        "[cid:{}] update_manual_item called, id={}, quantity={:?}, is_checked={:?}",
        cid, id, quantity, is_checked
    );

    let result = manual_items::update_manual_item(&id, quantity, is_checked)
        .await
        .map_err(|e| e.to_string());

    let elapsed = start.elapsed();
    match &result {
        Ok(item) => log::info!("[cid:{}] update_manual_item succeeded in {:?}, id={}", cid, elapsed, item.id),
        Err(e) => log::error!("[cid:{}] update_manual_item failed in {:?}: {}", cid, elapsed, e),
    }
    result
}

/// Delete a manual item
#[command]
pub async fn delete_manual_item(id: String, correlation_id: Option<String>) -> Result<(), String> {
    let cid = ensure_correlation_id(correlation_id);
    let start = Instant::now();
    log::debug!("[cid:{}] delete_manual_item called, id={}", cid, id);

    let result = manual_items::delete_manual_item(&id)
        .await
        .map_err(|e| e.to_string());

    let elapsed = start.elapsed();
    match &result {
        Ok(()) => log::info!("[cid:{}] delete_manual_item succeeded in {:?}, id={}", cid, elapsed, id),
        Err(e) => log::error!("[cid:{}] delete_manual_item failed in {:?}: {}", cid, elapsed, e),
    }
    result
}
