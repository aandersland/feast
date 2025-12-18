//! Shopping list database operations

use crate::db::pool::get_db_pool;
use crate::error::AppError;
use crate::utils::units::aggregate_quantities;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use std::time::Instant;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct ShoppingList {
    pub id: String,
    pub week_start: String,
    pub name: String,
    pub list_type: String,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct ShoppingListItem {
    pub id: String,
    pub list_id: String,
    pub ingredient_id: Option<String>,
    pub name: String,
    pub quantity: f64,
    pub unit: String,
    pub category: String,
    pub is_checked: bool,
    pub is_deleted: bool,
    pub deleted_at: Option<String>,
    pub moved_to_list_id: Option<String>,
    pub source_recipe_ids: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ShoppingListWithItems {
    #[serde(flatten)]
    pub list: ShoppingList,
    pub items: Vec<ShoppingListItem>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShoppingListInput {
    pub week_start: String,
    pub name: String,
    pub list_type: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShoppingItemInput {
    pub list_id: String,
    pub name: String,
    pub quantity: f64,
    pub unit: String,
    pub category: String,
}

/// Aggregated shopping item (from meal plans)
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AggregatedShoppingItem {
    pub name: String,
    pub quantity: f64,
    pub unit: String,
    pub category: String,
    pub source_recipe_ids: Vec<String>,
    pub is_converted: bool,
}

/// Get shopping lists for a week
pub async fn get_shopping_lists(week_start: &str) -> Result<Vec<ShoppingListWithItems>, AppError> {
    let pool = get_db_pool();
    let start = Instant::now();

    let lists = sqlx::query_as::<_, ShoppingList>(
        "SELECT id, week_start, name, list_type, created_at
         FROM shopping_lists WHERE week_start = ?
         ORDER BY created_at",
    )
    .bind(week_start)
    .fetch_all(pool)
    .await
    .map_err(|e| AppError::Database(e.to_string()))?;

    let mut result = vec![];
    for list in lists {
        let items = get_list_items(&list.id).await?;
        result.push(ShoppingListWithItems { list, items });
    }

    let elapsed = start.elapsed();
    log::debug!("db::get_shopping_lists completed in {:?}, {} lists", elapsed, result.len());

    Ok(result)
}

async fn get_list_items(list_id: &str) -> Result<Vec<ShoppingListItem>, AppError> {
    let pool = get_db_pool();
    let start = Instant::now();

    let result = sqlx::query_as::<_, ShoppingListItem>(
        "SELECT id, list_id, ingredient_id, name, quantity, unit, category,
                is_checked, is_deleted, deleted_at, moved_to_list_id,
                source_recipe_ids, created_at
         FROM shopping_list_items WHERE list_id = ?
         ORDER BY category, name",
    )
    .bind(list_id)
    .fetch_all(pool)
    .await
    .map_err(|e| AppError::Database(e.to_string()));

    let elapsed = start.elapsed();
    match &result {
        Ok(rows) => log::debug!("db::get_list_items completed in {:?}, {} rows", elapsed, rows.len()),
        Err(e) => log::debug!("db::get_list_items failed in {:?}: {}", elapsed, e),
    }
    result
}

/// Create a shopping list
pub async fn create_shopping_list(input: ShoppingListInput) -> Result<ShoppingList, AppError> {
    let pool = get_db_pool();
    let start = Instant::now();
    let id = Uuid::new_v4().to_string();
    let list_type = input.list_type.unwrap_or_else(|| "custom".to_string());

    sqlx::query("INSERT INTO shopping_lists (id, week_start, name, list_type) VALUES (?, ?, ?, ?)")
        .bind(&id)
        .bind(&input.week_start)
        .bind(&input.name)
        .bind(&list_type)
        .execute(pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

    let result = sqlx::query_as::<_, ShoppingList>(
        "SELECT id, week_start, name, list_type, created_at FROM shopping_lists WHERE id = ?",
    )
    .bind(&id)
    .fetch_one(pool)
    .await
    .map_err(|e| AppError::Database(e.to_string()));

    let elapsed = start.elapsed();
    match &result {
        Ok(_) => log::debug!("db::create_shopping_list completed in {:?}, 1 row", elapsed),
        Err(e) => log::debug!("db::create_shopping_list failed in {:?}: {}", elapsed, e),
    }
    result
}

/// Delete a shopping list
pub async fn delete_shopping_list(id: &str) -> Result<(), AppError> {
    let pool = get_db_pool();
    let start = Instant::now();

    let result = sqlx::query("DELETE FROM shopping_lists WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

    let elapsed = start.elapsed();
    if result.rows_affected() == 0 {
        log::debug!("db::delete_shopping_list failed in {:?}: shopping list not found", elapsed);
        return Err(AppError::NotFound(format!(
            "Shopping list with id {id} not found"
        )));
    }

    log::debug!("db::delete_shopping_list completed in {:?}, deleted", elapsed);
    Ok(())
}

/// Add an item to a shopping list
pub async fn add_shopping_item(input: ShoppingItemInput) -> Result<ShoppingListItem, AppError> {
    let pool = get_db_pool();
    let start = Instant::now();
    let id = Uuid::new_v4().to_string();

    sqlx::query(
        "INSERT INTO shopping_list_items (id, list_id, name, quantity, unit, category)
         VALUES (?, ?, ?, ?, ?, ?)",
    )
    .bind(&id)
    .bind(&input.list_id)
    .bind(&input.name)
    .bind(input.quantity)
    .bind(&input.unit)
    .bind(&input.category)
    .execute(pool)
    .await
    .map_err(|e| AppError::Database(e.to_string()))?;

    let result = sqlx::query_as::<_, ShoppingListItem>(
        "SELECT id, list_id, ingredient_id, name, quantity, unit, category,
                is_checked, is_deleted, deleted_at, moved_to_list_id,
                source_recipe_ids, created_at
         FROM shopping_list_items WHERE id = ?",
    )
    .bind(&id)
    .fetch_one(pool)
    .await
    .map_err(|e| AppError::Database(e.to_string()));

    let elapsed = start.elapsed();
    match &result {
        Ok(_) => log::debug!("db::add_shopping_item completed in {:?}, 1 row", elapsed),
        Err(e) => log::debug!("db::add_shopping_item failed in {:?}: {}", elapsed, e),
    }
    result
}

/// Update a shopping item
pub async fn update_shopping_item(
    id: &str,
    quantity: Option<f64>,
    is_checked: Option<bool>,
) -> Result<ShoppingListItem, AppError> {
    let pool = get_db_pool();
    let start = Instant::now();

    if let Some(qty) = quantity {
        sqlx::query("UPDATE shopping_list_items SET quantity = ? WHERE id = ?")
            .bind(qty)
            .bind(id)
            .execute(pool)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;
    }

    if let Some(checked) = is_checked {
        sqlx::query("UPDATE shopping_list_items SET is_checked = ? WHERE id = ?")
            .bind(checked)
            .bind(id)
            .execute(pool)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;
    }

    let result = sqlx::query_as::<_, ShoppingListItem>(
        "SELECT id, list_id, ingredient_id, name, quantity, unit, category,
                is_checked, is_deleted, deleted_at, moved_to_list_id,
                source_recipe_ids, created_at
         FROM shopping_list_items WHERE id = ?",
    )
    .bind(id)
    .fetch_one(pool)
    .await
    .map_err(|e| AppError::Database(e.to_string()));

    let elapsed = start.elapsed();
    match &result {
        Ok(_) => log::debug!("db::update_shopping_item completed in {:?}, 1 row", elapsed),
        Err(e) => log::debug!("db::update_shopping_item failed in {:?}: {}", elapsed, e),
    }
    result
}

/// Soft delete an item
pub async fn soft_delete_shopping_item(id: &str) -> Result<(), AppError> {
    let pool = get_db_pool();
    let start = Instant::now();

    let result = sqlx::query(
        "UPDATE shopping_list_items SET is_deleted = 1, deleted_at = datetime('now') WHERE id = ?",
    )
    .bind(id)
    .execute(pool)
    .await
    .map_err(|e| AppError::Database(e.to_string()))?;

    let elapsed = start.elapsed();
    if result.rows_affected() == 0 {
        log::debug!("db::soft_delete_shopping_item failed in {:?}: item not found", elapsed);
        return Err(AppError::NotFound(format!(
            "Shopping item with id {id} not found"
        )));
    }

    log::debug!("db::soft_delete_shopping_item completed in {:?}, 1 row", elapsed);
    Ok(())
}

/// Restore a soft-deleted item
pub async fn restore_shopping_item(id: &str) -> Result<ShoppingListItem, AppError> {
    let pool = get_db_pool();
    let start = Instant::now();

    let result = sqlx::query(
        "UPDATE shopping_list_items SET is_deleted = 0, deleted_at = NULL WHERE id = ?",
    )
    .bind(id)
    .execute(pool)
    .await
    .map_err(|e| AppError::Database(e.to_string()))?;

    let elapsed = start.elapsed();
    if result.rows_affected() == 0 {
        log::debug!("db::restore_shopping_item failed in {:?}: item not found", elapsed);
        return Err(AppError::NotFound(format!(
            "Shopping item with id {id} not found"
        )));
    }

    let fetch_result = sqlx::query_as::<_, ShoppingListItem>(
        "SELECT id, list_id, ingredient_id, name, quantity, unit, category,
                is_checked, is_deleted, deleted_at, moved_to_list_id,
                source_recipe_ids, created_at
         FROM shopping_list_items WHERE id = ?",
    )
    .bind(id)
    .fetch_one(pool)
    .await
    .map_err(|e| AppError::Database(e.to_string()));

    let total_elapsed = start.elapsed();
    match &fetch_result {
        Ok(_) => log::debug!("db::restore_shopping_item completed in {:?}, 1 row", total_elapsed),
        Err(e) => log::debug!("db::restore_shopping_item failed in {:?}: {}", total_elapsed, e),
    }
    fetch_result
}

/// Move an item to another list
pub async fn move_shopping_item(id: &str, to_list_id: &str) -> Result<ShoppingListItem, AppError> {
    let pool = get_db_pool();
    let start = Instant::now();

    sqlx::query("UPDATE shopping_list_items SET list_id = ?, moved_to_list_id = ? WHERE id = ?")
        .bind(to_list_id)
        .bind(to_list_id)
        .bind(id)
        .execute(pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

    let result = sqlx::query_as::<_, ShoppingListItem>(
        "SELECT id, list_id, ingredient_id, name, quantity, unit, category,
                is_checked, is_deleted, deleted_at, moved_to_list_id,
                source_recipe_ids, created_at
         FROM shopping_list_items WHERE id = ?",
    )
    .bind(id)
    .fetch_one(pool)
    .await
    .map_err(|e| AppError::Database(e.to_string()));

    let elapsed = start.elapsed();
    match &result {
        Ok(_) => log::debug!("db::move_shopping_item completed in {:?}, 1 row", elapsed),
        Err(e) => log::debug!("db::move_shopping_item failed in {:?}: {}", elapsed, e),
    }
    result
}

/// Get aggregated shopping list from meal plans for a date range
pub async fn get_aggregated_shopping_list(
    start_date: &str,
    end_date: &str,
) -> Result<Vec<AggregatedShoppingItem>, AppError> {
    let pool = get_db_pool();
    let start = Instant::now();

    // Get all ingredients from meal plans in date range
    #[derive(FromRow)]
    struct RawItem {
        name: String,
        quantity: f64,
        unit: String,
        category: String,
        recipe_id: String,
        servings_multiplier: f64,
    }

    let items = sqlx::query_as::<_, RawItem>(
        "SELECT
            i.name,
            ri.quantity,
            ri.unit,
            i.category,
            mp.recipe_id,
            CAST(mp.servings AS REAL) / CAST(r.servings AS REAL) as servings_multiplier
         FROM meal_plans mp
         JOIN recipes r ON mp.recipe_id = r.id
         JOIN recipe_ingredients ri ON r.id = ri.recipe_id
         JOIN ingredients i ON ri.ingredient_id = i.id
         WHERE mp.date >= ? AND mp.date <= ?",
    )
    .bind(start_date)
    .bind(end_date)
    .fetch_all(pool)
    .await
    .map_err(|e| AppError::Database(e.to_string()))?;

    let query_elapsed = start.elapsed();
    log::debug!("db::get_aggregated_shopping_list query completed in {:?}, {} raw items", query_elapsed, items.len());

    // Group by ingredient name (lowercase) and aggregate
    use std::collections::HashMap;

    struct GroupedItem {
        category: String,
        quantities: Vec<(f64, String)>,
        recipe_ids: Vec<String>,
    }

    let mut grouped: HashMap<String, GroupedItem> = HashMap::new();

    for item in items {
        let key = item.name.to_lowercase();
        let adjusted_qty = item.quantity * item.servings_multiplier;

        grouped
            .entry(key.clone())
            .or_insert_with(|| GroupedItem {
                category: item.category.clone(),
                quantities: vec![],
                recipe_ids: vec![],
            })
            .quantities
            .push((adjusted_qty, item.unit));

        let entry = grouped.get_mut(&key).unwrap();
        if !entry.recipe_ids.contains(&item.recipe_id) {
            entry.recipe_ids.push(item.recipe_id.clone());
        }
    }

    // Aggregate quantities using unit conversion
    let mut result = vec![];
    for (name, group) in grouped {
        let aggregated = aggregate_quantities(&group.quantities);

        for agg in aggregated {
            result.push(AggregatedShoppingItem {
                name: name.clone(),
                quantity: agg.quantity,
                unit: agg.unit,
                category: group.category.clone(),
                source_recipe_ids: group.recipe_ids.clone(),
                is_converted: agg.is_converted,
            });
        }
    }

    // Sort by category then name
    result.sort_by(|a, b| {
        a.category
            .cmp(&b.category)
            .then_with(|| a.name.cmp(&b.name))
    });

    let total_elapsed = start.elapsed();
    log::debug!("db::get_aggregated_shopping_list completed in {:?}, {} aggregated items", total_elapsed, result.len());

    Ok(result)
}
