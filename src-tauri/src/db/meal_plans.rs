//! Meal plan database operations

use crate::db::pool::get_db_pool;
use crate::error::AppError;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use std::time::Instant;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct MealPlan {
    pub id: String,
    pub date: String,
    pub meal_type: String,
    pub recipe_id: String,
    pub servings: i64,
    pub created_at: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MealPlanInput {
    pub date: String,
    pub meal_type: String,
    pub recipe_id: String,
    pub servings: i64,
}

/// Get meal plans for a date range
pub async fn get_meal_plans(start_date: &str, end_date: &str) -> Result<Vec<MealPlan>, AppError> {
    let pool = get_db_pool();
    let start = Instant::now();

    let result = sqlx::query_as::<_, MealPlan>(
        "SELECT id, date, meal_type, recipe_id, servings, created_at
         FROM meal_plans
         WHERE date >= ? AND date <= ?
         ORDER BY date,
           CASE meal_type
             WHEN 'breakfast' THEN 1
             WHEN 'lunch' THEN 2
             WHEN 'dinner' THEN 3
             WHEN 'snack' THEN 4
           END",
    )
    .bind(start_date)
    .bind(end_date)
    .fetch_all(pool)
    .await
    .map_err(|e| AppError::Database(e.to_string()));

    let elapsed = start.elapsed();
    match &result {
        Ok(rows) => log::debug!("db::get_meal_plans completed in {:?}, {} rows", elapsed, rows.len()),
        Err(e) => log::debug!("db::get_meal_plans failed in {:?}: {}", elapsed, e),
    }
    result
}

/// Create a meal plan entry
pub async fn create_meal_plan(input: MealPlanInput) -> Result<MealPlan, AppError> {
    let pool = get_db_pool();
    let start = Instant::now();
    let id = Uuid::new_v4().to_string();

    // Validate meal_type
    if !["breakfast", "lunch", "dinner", "snack"].contains(&input.meal_type.as_str()) {
        return Err(AppError::Validation(format!(
            "Invalid meal type: {}. Must be breakfast, lunch, dinner, or snack",
            input.meal_type
        )));
    }

    sqlx::query(
        "INSERT INTO meal_plans (id, date, meal_type, recipe_id, servings)
         VALUES (?, ?, ?, ?, ?)",
    )
    .bind(&id)
    .bind(&input.date)
    .bind(&input.meal_type)
    .bind(&input.recipe_id)
    .bind(input.servings)
    .execute(pool)
    .await
    .map_err(|e| AppError::Database(e.to_string()))?;

    let result = sqlx::query_as::<_, MealPlan>(
        "SELECT id, date, meal_type, recipe_id, servings, created_at
         FROM meal_plans WHERE id = ?",
    )
    .bind(&id)
    .fetch_one(pool)
    .await
    .map_err(|e| AppError::Database(e.to_string()));

    let elapsed = start.elapsed();
    match &result {
        Ok(_) => log::debug!("db::create_meal_plan completed in {:?}, 1 row", elapsed),
        Err(e) => log::debug!("db::create_meal_plan failed in {:?}: {}", elapsed, e),
    }
    result
}

/// Update meal plan servings
pub async fn update_meal_plan(id: &str, servings: i64) -> Result<MealPlan, AppError> {
    let pool = get_db_pool();
    let start = Instant::now();

    let result = sqlx::query("UPDATE meal_plans SET servings = ? WHERE id = ?")
        .bind(servings)
        .bind(id)
        .execute(pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

    let elapsed = start.elapsed();
    if result.rows_affected() == 0 {
        log::debug!("db::update_meal_plan failed in {:?}: meal plan not found", elapsed);
        return Err(AppError::NotFound(format!(
            "Meal plan with id {id} not found"
        )));
    }

    let fetch_result = sqlx::query_as::<_, MealPlan>(
        "SELECT id, date, meal_type, recipe_id, servings, created_at
         FROM meal_plans WHERE id = ?",
    )
    .bind(id)
    .fetch_one(pool)
    .await
    .map_err(|e| AppError::Database(e.to_string()));

    let total_elapsed = start.elapsed();
    match &fetch_result {
        Ok(_) => log::debug!("db::update_meal_plan completed in {:?}, 1 row", total_elapsed),
        Err(e) => log::debug!("db::update_meal_plan failed in {:?}: {}", total_elapsed, e),
    }
    fetch_result
}

/// Delete a meal plan entry
pub async fn delete_meal_plan(id: &str) -> Result<(), AppError> {
    let pool = get_db_pool();
    let start = Instant::now();

    let result = sqlx::query("DELETE FROM meal_plans WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

    let elapsed = start.elapsed();
    if result.rows_affected() == 0 {
        log::debug!("db::delete_meal_plan failed in {:?}: meal plan not found", elapsed);
        return Err(AppError::NotFound(format!(
            "Meal plan with id {id} not found"
        )));
    }

    log::debug!("db::delete_meal_plan completed in {:?}, deleted", elapsed);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::pool::init_db_for_test;
    use crate::db::recipes::{create_recipe, RecipeInput};

    async fn create_test_recipe() -> String {
        let input = RecipeInput {
            name: "Test Recipe".to_string(),
            description: "".to_string(),
            prep_time: 0,
            cook_time: 0,
            servings: 4,
            image_path: None,
            source_url: None,
            notes: None,
            tags: vec![],
            ingredients: vec![],
            instructions: vec![],
        };
        create_recipe(input).await.unwrap().id
    }

    #[tokio::test]
    async fn test_create_meal_plan() {
        init_db_for_test().await;
        let recipe_id = create_test_recipe().await;

        let input = MealPlanInput {
            date: "2025-01-15".to_string(),
            meal_type: "dinner".to_string(),
            recipe_id,
            servings: 2,
        };

        let plan = create_meal_plan(input).await.unwrap();

        assert_eq!(plan.date, "2025-01-15");
        assert_eq!(plan.meal_type, "dinner");
        assert_eq!(plan.servings, 2);
    }

    #[tokio::test]
    async fn test_get_meal_plans_by_date_range() {
        init_db_for_test().await;
        let recipe_id = create_test_recipe().await;

        // Create plans for multiple days (use unique dates to avoid conflict with other tests)
        for date in ["2025-02-13", "2025-02-14", "2025-02-15", "2025-02-20"] {
            let input = MealPlanInput {
                date: date.to_string(),
                meal_type: "dinner".to_string(),
                recipe_id: recipe_id.clone(),
                servings: 2,
            };
            create_meal_plan(input).await.unwrap();
        }

        let plans = get_meal_plans("2025-02-13", "2025-02-15").await.unwrap();
        assert_eq!(plans.len(), 3); // Only dates in range
    }
}
