//! Meal plan command handlers

use crate::correlation::ensure_correlation_id;
use crate::db::meal_plans::{self, MealPlan, MealPlanInput};
use crate::logging::redact;
use std::time::Instant;
use tauri::command;

/// Get meal plans for a date range
#[command]
pub async fn get_meal_plans(start_date: String, end_date: String, correlation_id: Option<String>) -> Result<Vec<MealPlan>, String> {
    let cid = ensure_correlation_id(correlation_id);
    let start = Instant::now();
    log::debug!("[cid:{}] get_meal_plans called, start_date={}, end_date={}", cid, start_date, end_date);

    let result = meal_plans::get_meal_plans(&start_date, &end_date)
        .await
        .map_err(|e| e.into());

    let elapsed = start.elapsed();
    match &result {
        Ok(plans) => log::info!(
            "[cid:{}] get_meal_plans succeeded in {:?}, {}",
            cid, elapsed, redact::format_count(plans.len(), "plan")
        ),
        Err(e) => log::error!("[cid:{}] get_meal_plans failed in {:?}: {}", cid, elapsed, e),
    }
    result
}

/// Create a meal plan entry
#[command]
pub async fn create_meal_plan(input: MealPlanInput, correlation_id: Option<String>) -> Result<MealPlan, String> {
    let cid = ensure_correlation_id(correlation_id);
    let start = Instant::now();
    log::debug!(
        "[cid:{}] create_meal_plan called, recipe_id={}, date={}, meal_type={}",
        cid, input.recipe_id, input.date, input.meal_type
    );

    let result = meal_plans::create_meal_plan(input)
        .await
        .map_err(|e| e.into());

    let elapsed = start.elapsed();
    match &result {
        Ok(plan) => log::info!("[cid:{}] create_meal_plan succeeded in {:?}, id={}", cid, elapsed, plan.id),
        Err(e) => log::error!("[cid:{}] create_meal_plan failed in {:?}: {}", cid, elapsed, e),
    }
    result
}

/// Update meal plan servings
#[command]
pub async fn update_meal_plan(id: String, servings: i64, correlation_id: Option<String>) -> Result<MealPlan, String> {
    let cid = ensure_correlation_id(correlation_id);
    let start = Instant::now();
    log::debug!("[cid:{}] update_meal_plan called, id={}, servings={}", cid, id, servings);

    let result = meal_plans::update_meal_plan(&id, servings)
        .await
        .map_err(|e| e.into());

    let elapsed = start.elapsed();
    match &result {
        Ok(plan) => log::info!("[cid:{}] update_meal_plan succeeded in {:?}, id={}", cid, elapsed, plan.id),
        Err(e) => log::error!("[cid:{}] update_meal_plan failed in {:?}: {}", cid, elapsed, e),
    }
    result
}

/// Delete a meal plan entry
#[command]
pub async fn delete_meal_plan(id: String, correlation_id: Option<String>) -> Result<(), String> {
    let cid = ensure_correlation_id(correlation_id);
    let start = Instant::now();
    log::debug!("[cid:{}] delete_meal_plan called, id={}", cid, id);

    let result = meal_plans::delete_meal_plan(&id)
        .await
        .map_err(|e| e.into());

    let elapsed = start.elapsed();
    match &result {
        Ok(()) => log::info!("[cid:{}] delete_meal_plan succeeded in {:?}, id={}", cid, elapsed, id),
        Err(e) => log::error!("[cid:{}] delete_meal_plan failed in {:?}: {}", cid, elapsed, e),
    }
    result
}
