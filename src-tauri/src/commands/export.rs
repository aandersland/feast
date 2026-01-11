//! Data export command for migration to web version

use crate::correlation::ensure_correlation_id;
use crate::db::{
    manual_items, meal_plans, quick_lists, recipes, shopping_lists,
};
use crate::error::AppError;
use serde::Serialize;
use std::time::Instant;
use tauri::command;

/// Complete data export for migration
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FeastExport {
    pub version: String,
    pub exported_at: String,
    pub recipes: Vec<recipes::Recipe>,
    pub meal_plans: Vec<meal_plans::MealPlan>,
    pub shopping_lists: Vec<shopping_lists::ShoppingListWithItems>,
    pub quick_lists: Vec<quick_lists::QuickListWithItems>,
    pub manual_items: Vec<manual_items::ManualItem>,
}

/// Export all data for migration to the web version
/// Returns JSON that can be saved to a file
#[command]
pub async fn export_all_data(correlation_id: Option<String>) -> Result<FeastExport, String> {
    let cid = ensure_correlation_id(correlation_id);
    let start = Instant::now();
    log::info!("[cid:{}] export_all_data called", cid);

    let result = export_data_internal().await;

    let elapsed = start.elapsed();
    match &result {
        Ok(export) => {
            log::info!(
                "[cid:{}] export_all_data succeeded in {:?}, {} recipes, {} meal plans, {} shopping lists, {} quick lists, {} manual items",
                cid, elapsed,
                export.recipes.len(),
                export.meal_plans.len(),
                export.shopping_lists.len(),
                export.quick_lists.len(),
                export.manual_items.len()
            );
        }
        Err(e) => {
            log::error!("[cid:{}] export_all_data failed in {:?}: {}", cid, elapsed, e);
        }
    }

    result.map_err(|e| e.to_string())
}

async fn export_data_internal() -> Result<FeastExport, AppError> {
    // Get all recipes with full details
    let recipe_rows = recipes::get_all_recipes().await?;
    let mut all_recipes = Vec::with_capacity(recipe_rows.len());
    for row in recipe_rows {
        let recipe = recipes::get_recipe_by_id(&row.id).await?;
        all_recipes.push(recipe);
    }

    // Get all meal plans (use a wide date range to get everything)
    let all_meal_plans = meal_plans::get_meal_plans("2000-01-01", "2100-12-31").await?;

    // Get all shopping lists (we need to query for all weeks)
    // First get unique week_starts from the database
    let all_shopping_lists = get_all_shopping_lists().await?;

    // Get all quick lists
    let all_quick_lists = quick_lists::get_quick_lists().await?;

    // Get all manual items (same approach - get all weeks)
    let all_manual_items = get_all_manual_items().await?;

    let now = chrono::Utc::now().to_rfc3339();

    Ok(FeastExport {
        version: "1.0".to_string(),
        exported_at: now,
        recipes: all_recipes,
        meal_plans: all_meal_plans,
        shopping_lists: all_shopping_lists,
        quick_lists: all_quick_lists,
        manual_items: all_manual_items,
    })
}

/// Get all shopping lists from all weeks
async fn get_all_shopping_lists() -> Result<Vec<shopping_lists::ShoppingListWithItems>, AppError> {
    use crate::db::pool::get_db_pool;

    let pool = get_db_pool();

    // Get all unique week_starts
    let week_starts: Vec<(String,)> = sqlx::query_as(
        "SELECT DISTINCT week_start FROM shopping_lists ORDER BY week_start",
    )
    .fetch_all(pool)
    .await
    .map_err(|e| AppError::Database(e.to_string()))?;

    let mut all_lists = Vec::new();
    for (week_start,) in week_starts {
        let lists = shopping_lists::get_shopping_lists(&week_start).await?;
        all_lists.extend(lists);
    }

    Ok(all_lists)
}

/// Get all manual items from all weeks
async fn get_all_manual_items() -> Result<Vec<manual_items::ManualItem>, AppError> {
    use crate::db::pool::get_db_pool;

    let pool = get_db_pool();

    // Get all unique week_starts
    let week_starts: Vec<(String,)> = sqlx::query_as(
        "SELECT DISTINCT week_start FROM manual_shopping_items ORDER BY week_start",
    )
    .fetch_all(pool)
    .await
    .map_err(|e| AppError::Database(e.to_string()))?;

    let mut all_items = Vec::new();
    for (week_start,) in week_starts {
        let items = manual_items::get_manual_items(&week_start).await?;
        all_items.extend(items);
    }

    Ok(all_items)
}
