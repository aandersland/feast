//! Item database operations

use crate::error::AppError;
use serde::Serialize;
use sqlx::FromRow;

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

    let items = sqlx::query_as::<_, Item>("SELECT id, name, created_at FROM items ORDER BY created_at DESC")
        .fetch_all(pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

    Ok(items)
}

/// Create a new item
pub async fn create_new_item(name: &str) -> Result<Item, AppError> {
    let pool = get_db_pool();

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

    Ok(item)
}

/// Delete an item by ID
pub async fn delete_item_by_id(id: i64) -> Result<(), AppError> {
    let pool = get_db_pool();

    let result = sqlx::query("DELETE FROM items WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound(format!("Item with id {id} not found")));
    }

    Ok(())
}
