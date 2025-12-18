//! Ingredient command handlers

use crate::correlation::ensure_correlation_id;
use crate::db::ingredients::{self, Ingredient};
use crate::logging::redact;
use std::time::Instant;
use tauri::command;

/// Get all ingredients
#[command]
pub async fn get_ingredients(correlation_id: Option<String>) -> Result<Vec<Ingredient>, String> {
    let cid = ensure_correlation_id(correlation_id);
    let start = Instant::now();
    log::debug!("[cid:{}] get_ingredients called", cid);

    let result = ingredients::get_all_ingredients()
        .await
        .map_err(|e| e.into());

    let elapsed = start.elapsed();
    match &result {
        Ok(ingredients) => log::info!(
            "[cid:{}] get_ingredients succeeded in {:?}, {}",
            cid, elapsed, redact::format_count(ingredients.len(), "ingredient")
        ),
        Err(e) => log::error!("[cid:{}] get_ingredients failed in {:?}: {}", cid, elapsed, e),
    }
    result
}

/// Create a new ingredient
#[command]
pub async fn create_ingredient(
    name: String,
    category: String,
    default_unit: Option<String>,
    correlation_id: Option<String>,
) -> Result<Ingredient, String> {
    let cid = ensure_correlation_id(correlation_id);
    let start = Instant::now();
    log::debug!(
        "[cid:{}] create_ingredient called, {}, category={}",
        cid,
        redact::redact_string(Some(&name), "name"),
        category
    );

    let result = ingredients::create_ingredient(&name, &category, default_unit.as_deref())
        .await
        .map_err(|e| e.into());

    let elapsed = start.elapsed();
    match &result {
        Ok(ingredient) => log::info!("[cid:{}] create_ingredient succeeded in {:?}, id={}", cid, elapsed, ingredient.id),
        Err(e) => log::error!("[cid:{}] create_ingredient failed in {:?}: {}", cid, elapsed, e),
    }
    result
}

/// Get or create an ingredient
#[command]
pub async fn get_or_create_ingredient(
    name: String,
    category: String,
    default_unit: Option<String>,
    correlation_id: Option<String>,
) -> Result<Ingredient, String> {
    let cid = ensure_correlation_id(correlation_id);
    let start = Instant::now();
    log::debug!(
        "[cid:{}] get_or_create_ingredient called, {}, category={}",
        cid,
        redact::redact_string(Some(&name), "name"),
        category
    );

    let result = ingredients::get_or_create_ingredient(&name, &category, default_unit.as_deref())
        .await
        .map_err(|e| e.into());

    let elapsed = start.elapsed();
    match &result {
        Ok(ingredient) => log::info!("[cid:{}] get_or_create_ingredient succeeded in {:?}, id={}", cid, elapsed, ingredient.id),
        Err(e) => log::error!("[cid:{}] get_or_create_ingredient failed in {:?}: {}", cid, elapsed, e),
    }
    result
}
