//! Manual shopping item database operations

use crate::db::pool::get_db_pool;
use crate::error::AppError;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use std::time::Instant;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct ManualItem {
    pub id: String,
    pub week_start: String,
    pub name: String,
    pub quantity: f64,
    pub unit: String,
    pub category: String,
    pub is_checked: bool,
    pub created_at: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ManualItemInput {
    pub week_start: String,
    pub name: String,
    pub quantity: f64,
    pub unit: String,
    pub category: String,
}

/// Get manual items for a week
pub async fn get_manual_items(week_start: &str) -> Result<Vec<ManualItem>, AppError> {
    let pool = get_db_pool();
    let start = Instant::now();

    let result = sqlx::query_as::<_, ManualItem>(
        "SELECT id, week_start, name, quantity, unit, category, is_checked, created_at
         FROM manual_shopping_items WHERE week_start = ?
         ORDER BY category, name",
    )
    .bind(week_start)
    .fetch_all(pool)
    .await
    .map_err(|e| AppError::Database(e.to_string()));

    let elapsed = start.elapsed();
    match &result {
        Ok(rows) => log::debug!("db::get_manual_items completed in {:?}, {} rows", elapsed, rows.len()),
        Err(e) => log::debug!("db::get_manual_items failed in {:?}: {}", elapsed, e),
    }
    result
}

/// Create a manual item
pub async fn create_manual_item(input: ManualItemInput) -> Result<ManualItem, AppError> {
    let pool = get_db_pool();
    let start = Instant::now();
    let id = Uuid::new_v4().to_string();

    sqlx::query(
        "INSERT INTO manual_shopping_items (id, week_start, name, quantity, unit, category)
         VALUES (?, ?, ?, ?, ?, ?)",
    )
    .bind(&id)
    .bind(&input.week_start)
    .bind(&input.name)
    .bind(input.quantity)
    .bind(&input.unit)
    .bind(&input.category)
    .execute(pool)
    .await
    .map_err(|e| AppError::Database(e.to_string()))?;

    let result = sqlx::query_as::<_, ManualItem>(
        "SELECT id, week_start, name, quantity, unit, category, is_checked, created_at
         FROM manual_shopping_items WHERE id = ?",
    )
    .bind(&id)
    .fetch_one(pool)
    .await
    .map_err(|e| AppError::Database(e.to_string()));

    let elapsed = start.elapsed();
    match &result {
        Ok(_) => log::debug!("db::create_manual_item completed in {:?}, 1 row", elapsed),
        Err(e) => log::debug!("db::create_manual_item failed in {:?}: {}", elapsed, e),
    }
    result
}

/// Update a manual item
pub async fn update_manual_item(
    id: &str,
    quantity: Option<f64>,
    is_checked: Option<bool>,
) -> Result<ManualItem, AppError> {
    let pool = get_db_pool();
    let start = Instant::now();

    if let Some(qty) = quantity {
        sqlx::query("UPDATE manual_shopping_items SET quantity = ? WHERE id = ?")
            .bind(qty)
            .bind(id)
            .execute(pool)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;
    }

    if let Some(checked) = is_checked {
        sqlx::query("UPDATE manual_shopping_items SET is_checked = ? WHERE id = ?")
            .bind(checked)
            .bind(id)
            .execute(pool)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;
    }

    let result = sqlx::query_as::<_, ManualItem>(
        "SELECT id, week_start, name, quantity, unit, category, is_checked, created_at
         FROM manual_shopping_items WHERE id = ?",
    )
    .bind(id)
    .fetch_one(pool)
    .await
    .map_err(|e| AppError::Database(e.to_string()));

    let elapsed = start.elapsed();
    match &result {
        Ok(_) => log::debug!("db::update_manual_item completed in {:?}, 1 row", elapsed),
        Err(e) => log::debug!("db::update_manual_item failed in {:?}: {}", elapsed, e),
    }
    result
}

/// Delete a manual item
pub async fn delete_manual_item(id: &str) -> Result<(), AppError> {
    let pool = get_db_pool();
    let start = Instant::now();

    let result = sqlx::query("DELETE FROM manual_shopping_items WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

    let elapsed = start.elapsed();
    if result.rows_affected() == 0 {
        log::debug!("db::delete_manual_item failed in {:?}: manual item not found", elapsed);
        return Err(AppError::NotFound(format!(
            "Manual item with id {id} not found"
        )));
    }

    log::debug!("db::delete_manual_item completed in {:?}, deleted", elapsed);
    Ok(())
}
