//! Item database operations

use crate::error::AppError;
use serde::Serialize;
use sqlx::FromRow;
use std::time::Instant;

use super::pool::get_db_pool;

#[derive(Debug, Serialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    pub id: i64,
    pub name: String,
    pub created_at: String,
}

/// Get all items
pub async fn get_all_items() -> Result<Vec<Item>, AppError> {
    let pool = get_db_pool();
    let start = Instant::now();

    let result = sqlx::query_as::<_, Item>(
        "SELECT id, name, created_at FROM items ORDER BY created_at DESC",
    )
    .fetch_all(pool)
    .await
    .map_err(|e| AppError::Database(e.to_string()));

    let elapsed = start.elapsed();
    match &result {
        Ok(rows) => log::debug!("db::get_all_items completed in {:?}, {} rows", elapsed, rows.len()),
        Err(e) => log::debug!("db::get_all_items failed in {:?}: {}", elapsed, e),
    }
    result
}

/// Create a new item
pub async fn create_new_item(name: &str) -> Result<Item, AppError> {
    let pool = get_db_pool();
    let start = Instant::now();

    let result = sqlx::query("INSERT INTO items (name) VALUES (?)")
        .bind(name)
        .execute(pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

    let id = result.last_insert_rowid();

    let item = sqlx::query_as::<_, Item>("SELECT id, name, created_at FROM items WHERE id = ?")
        .bind(id)
        .fetch_one(pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

    let elapsed = start.elapsed();
    log::debug!("db::create_new_item completed in {:?}, 1 row", elapsed);
    Ok(item)
}

/// Delete an item by ID
pub async fn delete_item_by_id(id: i64) -> Result<(), AppError> {
    let pool = get_db_pool();
    let start = Instant::now();

    let result = sqlx::query("DELETE FROM items WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

    let elapsed = start.elapsed();
    if result.rows_affected() == 0 {
        log::debug!("db::delete_item_by_id failed in {:?}: item not found", elapsed);
        return Err(AppError::NotFound(format!("Item with id {id} not found")));
    }

    log::debug!("db::delete_item_by_id completed in {:?}, deleted", elapsed);
    Ok(())
}
