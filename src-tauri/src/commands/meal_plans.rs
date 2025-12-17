//! Meal plan command handlers

use crate::db::meal_plans::{self, MealPlan, MealPlanInput};
use tauri::command;

/// Get meal plans for a date range
#[command]
pub async fn get_meal_plans(start_date: String, end_date: String) -> Result<Vec<MealPlan>, String> {
    meal_plans::get_meal_plans(&start_date, &end_date)
        .await
        .map_err(|e| e.into())
}

/// Create a meal plan entry
#[command]
pub async fn create_meal_plan(input: MealPlanInput) -> Result<MealPlan, String> {
    meal_plans::create_meal_plan(input)
        .await
        .map_err(|e| e.into())
}

/// Update meal plan servings
#[command]
pub async fn update_meal_plan(id: String, servings: i64) -> Result<MealPlan, String> {
    meal_plans::update_meal_plan(&id, servings)
        .await
        .map_err(|e| e.into())
}

/// Delete a meal plan entry
#[command]
pub async fn delete_meal_plan(id: String) -> Result<(), String> {
    meal_plans::delete_meal_plan(&id)
        .await
        .map_err(|e| e.into())
}
