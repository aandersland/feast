//! Quick list database operations

use crate::db::pool::get_db_pool;
use crate::db::shopping_lists::{add_shopping_item, ShoppingItemInput, ShoppingListItem};
use crate::error::AppError;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct QuickList {
    pub id: String,
    pub name: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct QuickListItem {
    pub id: String,
    pub quick_list_id: String,
    pub name: String,
    pub quantity: f64,
    pub unit: String,
    pub category: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QuickListWithItems {
    #[serde(flatten)]
    pub list: QuickList,
    pub items: Vec<QuickListItem>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuickListItemInput {
    pub name: String,
    pub quantity: f64,
    pub unit: String,
    pub category: String,
}

/// Get all quick lists with items
pub async fn get_quick_lists() -> Result<Vec<QuickListWithItems>, AppError> {
    let pool = get_db_pool();

    let lists = sqlx::query_as::<_, QuickList>(
        "SELECT id, name, created_at, updated_at FROM quick_lists ORDER BY name",
    )
    .fetch_all(pool)
    .await
    .map_err(|e| AppError::Database(e.to_string()))?;

    let mut result = vec![];
    for list in lists {
        let items = sqlx::query_as::<_, QuickListItem>(
            "SELECT id, quick_list_id, name, quantity, unit, category
             FROM quick_list_items WHERE quick_list_id = ?
             ORDER BY category, name",
        )
        .bind(&list.id)
        .fetch_all(pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        result.push(QuickListWithItems { list, items });
    }

    Ok(result)
}

/// Create a quick list
pub async fn create_quick_list(name: &str) -> Result<QuickList, AppError> {
    let pool = get_db_pool();
    let id = Uuid::new_v4().to_string();

    sqlx::query("INSERT INTO quick_lists (id, name) VALUES (?, ?)")
        .bind(&id)
        .bind(name)
        .execute(pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

    sqlx::query_as::<_, QuickList>(
        "SELECT id, name, created_at, updated_at FROM quick_lists WHERE id = ?",
    )
    .bind(&id)
    .fetch_one(pool)
    .await
    .map_err(|e| AppError::Database(e.to_string()))
}

/// Update (rename) a quick list
pub async fn update_quick_list(id: &str, name: &str) -> Result<QuickList, AppError> {
    let pool = get_db_pool();

    let result =
        sqlx::query("UPDATE quick_lists SET name = ?, updated_at = datetime('now') WHERE id = ?")
            .bind(name)
            .bind(id)
            .execute(pool)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound(format!(
            "Quick list with id {id} not found"
        )));
    }

    sqlx::query_as::<_, QuickList>(
        "SELECT id, name, created_at, updated_at FROM quick_lists WHERE id = ?",
    )
    .bind(id)
    .fetch_one(pool)
    .await
    .map_err(|e| AppError::Database(e.to_string()))
}

/// Delete a quick list
pub async fn delete_quick_list(id: &str) -> Result<(), AppError> {
    let pool = get_db_pool();

    let result = sqlx::query("DELETE FROM quick_lists WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound(format!(
            "Quick list with id {id} not found"
        )));
    }

    Ok(())
}

/// Add item to quick list
pub async fn add_quick_list_item(
    quick_list_id: &str,
    input: QuickListItemInput,
) -> Result<QuickListItem, AppError> {
    let pool = get_db_pool();
    let id = Uuid::new_v4().to_string();

    sqlx::query(
        "INSERT INTO quick_list_items (id, quick_list_id, name, quantity, unit, category)
         VALUES (?, ?, ?, ?, ?, ?)",
    )
    .bind(&id)
    .bind(quick_list_id)
    .bind(&input.name)
    .bind(input.quantity)
    .bind(&input.unit)
    .bind(&input.category)
    .execute(pool)
    .await
    .map_err(|e| AppError::Database(e.to_string()))?;

    // Update parent timestamp
    sqlx::query("UPDATE quick_lists SET updated_at = datetime('now') WHERE id = ?")
        .bind(quick_list_id)
        .execute(pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

    sqlx::query_as::<_, QuickListItem>(
        "SELECT id, quick_list_id, name, quantity, unit, category
         FROM quick_list_items WHERE id = ?",
    )
    .bind(&id)
    .fetch_one(pool)
    .await
    .map_err(|e| AppError::Database(e.to_string()))
}

/// Update quick list item
pub async fn update_quick_list_item(
    id: &str,
    input: QuickListItemInput,
) -> Result<QuickListItem, AppError> {
    let pool = get_db_pool();

    let result = sqlx::query(
        "UPDATE quick_list_items SET name = ?, quantity = ?, unit = ?, category = ? WHERE id = ?",
    )
    .bind(&input.name)
    .bind(input.quantity)
    .bind(&input.unit)
    .bind(&input.category)
    .bind(id)
    .execute(pool)
    .await
    .map_err(|e| AppError::Database(e.to_string()))?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound(format!(
            "Quick list item with id {id} not found"
        )));
    }

    sqlx::query_as::<_, QuickListItem>(
        "SELECT id, quick_list_id, name, quantity, unit, category
         FROM quick_list_items WHERE id = ?",
    )
    .bind(id)
    .fetch_one(pool)
    .await
    .map_err(|e| AppError::Database(e.to_string()))
}

/// Remove item from quick list
pub async fn remove_quick_list_item(id: &str) -> Result<(), AppError> {
    let pool = get_db_pool();

    let result = sqlx::query("DELETE FROM quick_list_items WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound(format!(
            "Quick list item with id {id} not found"
        )));
    }

    Ok(())
}

/// Copy quick list items to a shopping list
pub async fn add_quick_list_to_shopping(
    quick_list_id: &str,
    shopping_list_id: &str,
) -> Result<Vec<ShoppingListItem>, AppError> {
    let pool = get_db_pool();

    let items = sqlx::query_as::<_, QuickListItem>(
        "SELECT id, quick_list_id, name, quantity, unit, category
         FROM quick_list_items WHERE quick_list_id = ?",
    )
    .bind(quick_list_id)
    .fetch_all(pool)
    .await
    .map_err(|e| AppError::Database(e.to_string()))?;

    let mut added = vec![];
    for item in items {
        let input = ShoppingItemInput {
            list_id: shopping_list_id.to_string(),
            name: item.name,
            quantity: item.quantity,
            unit: item.unit,
            category: item.category,
        };
        let shopping_item = add_shopping_item(input).await?;
        added.push(shopping_item);
    }

    Ok(added)
}
