//! Recipe command handlers

use crate::correlation::ensure_correlation_id;
use crate::db::pool::get_db_pool;
use crate::db::recipes::{self, IngredientInput, Recipe, RecipeInput, RecipeRow};
use crate::http::{self, FetchError};
use crate::logging::redact;
use crate::parser::{self, ParseError, ParsedRecipe};
use std::time::Instant;
use tauri::command;

/// Get all recipes (list view)
#[command]
pub async fn get_recipes(correlation_id: Option<String>) -> Result<Vec<RecipeRow>, String> {
    let cid = ensure_correlation_id(correlation_id);
    let start = Instant::now();
    log::debug!("[cid:{}] get_recipes called", cid);

    let result = recipes::get_all_recipes().await.map_err(|e| e.into());

    let elapsed = start.elapsed();
    match &result {
        Ok(recipes) => log::info!(
            "[cid:{}] get_recipes succeeded in {:?}, {}",
            cid, elapsed, redact::format_count(recipes.len(), "recipe")
        ),
        Err(e) => log::error!("[cid:{}] get_recipes failed in {:?}: {}", cid, elapsed, e),
    }
    result
}

/// Get a single recipe with full details
#[command]
pub async fn get_recipe(id: String, correlation_id: Option<String>) -> Result<Recipe, String> {
    let cid = ensure_correlation_id(correlation_id);
    let start = Instant::now();
    log::debug!("[cid:{}] get_recipe called, id={}", cid, id);

    let result = recipes::get_recipe_by_id(&id).await.map_err(|e| e.into());

    let elapsed = start.elapsed();
    match &result {
        Ok(recipe) => log::info!("[cid:{}] get_recipe succeeded in {:?}, id={}", cid, elapsed, recipe.id),
        Err(e) => log::error!("[cid:{}] get_recipe failed in {:?}: {}", cid, elapsed, e),
    }
    result
}

/// Create a new recipe
#[command]
pub async fn create_recipe(input: RecipeInput, correlation_id: Option<String>) -> Result<Recipe, String> {
    let cid = ensure_correlation_id(correlation_id);
    let start = Instant::now();
    log::debug!(
        "[cid:{}] create_recipe called, {}, {}",
        cid,
        redact::redact_string(Some(&input.name), "name"),
        redact::format_count(input.ingredients.len(), "ingredient")
    );

    let result = recipes::create_recipe(input).await.map_err(|e| e.into());

    let elapsed = start.elapsed();
    match &result {
        Ok(recipe) => log::info!("[cid:{}] create_recipe succeeded in {:?}, id={}", cid, elapsed, recipe.id),
        Err(e) => log::error!("[cid:{}] create_recipe failed in {:?}: {}", cid, elapsed, e),
    }
    result
}

/// Update an existing recipe
#[command]
pub async fn update_recipe(id: String, input: RecipeInput, correlation_id: Option<String>) -> Result<Recipe, String> {
    let cid = ensure_correlation_id(correlation_id);
    let start = Instant::now();
    log::debug!(
        "[cid:{}] update_recipe called, id={}, {}, {}",
        cid, id,
        redact::redact_string(Some(&input.name), "name"),
        redact::format_count(input.ingredients.len(), "ingredient")
    );

    let result = recipes::update_recipe(&id, input).await.map_err(|e| e.into());

    let elapsed = start.elapsed();
    match &result {
        Ok(recipe) => log::info!("[cid:{}] update_recipe succeeded in {:?}, id={}", cid, elapsed, recipe.id),
        Err(e) => log::error!("[cid:{}] update_recipe failed in {:?}: {}", cid, elapsed, e),
    }
    result
}

/// Delete a recipe
#[command]
pub async fn delete_recipe(id: String, correlation_id: Option<String>) -> Result<(), String> {
    let cid = ensure_correlation_id(correlation_id);
    let start = Instant::now();
    log::debug!("[cid:{}] delete_recipe called, id={}", cid, id);

    let result = recipes::delete_recipe(&id).await.map_err(|e| e.into());

    let elapsed = start.elapsed();
    match &result {
        Ok(()) => log::info!("[cid:{}] delete_recipe succeeded in {:?}, id={}", cid, elapsed, id),
        Err(e) => log::error!("[cid:{}] delete_recipe failed in {:?}: {}", cid, elapsed, e),
    }
    result
}

/// Import a recipe from a URL
#[command]
pub async fn import_recipe_from_url(url: String, correlation_id: Option<String>) -> Result<Recipe, String> {
    let cid = ensure_correlation_id(correlation_id);
    let start = Instant::now();
    log::debug!("[cid:{}] import_recipe_from_url called, url_len={}", cid, url.len());

    // Validate URL format
    let url = url.trim();
    if url.is_empty() {
        log::warn!("[cid:{}] import_recipe_from_url failed: empty URL", cid);
        return Err("Please enter a valid website URL".to_string());
    }

    // Check for duplicate source_url
    if recipe_exists_by_source_url(url).await? {
        log::warn!("[cid:{}] import_recipe_from_url failed: duplicate URL", cid);
        return Err("A recipe from this URL has already been imported".to_string());
    }

    // Fetch HTML
    let html = http::fetch_url(url).await.map_err(|e| {
        let msg = match e {
            FetchError::InvalidUrl(_) => "Please enter a valid website URL".to_string(),
            FetchError::InvalidUrlScheme => "Please enter a valid website URL".to_string(),
            FetchError::ConnectionFailed(_) => "Could not connect to the website".to_string(),
            FetchError::Timeout(_) => "The website took too long to respond".to_string(),
            FetchError::TooManyRedirects(_) => "Could not connect to the website".to_string(),
            FetchError::HttpError { status, .. } => {
                format!("The website returned an error (HTTP {})", status)
            }
            FetchError::InvalidContentType(_) => {
                "This URL does not appear to be a recipe page".to_string()
            }
            FetchError::ResponseTooLarge(_) => "The page is too large to process".to_string(),
            FetchError::ReadError(_) => "Could not read the website response".to_string(),
        };
        log::error!("[cid:{}] import_recipe_from_url fetch failed: {}", cid, msg);
        msg
    })?;

    // Parse JSON-LD
    let parsed = parser::parse_recipe_from_html(&html).map_err(|e| {
        let msg = match e {
            ParseError::NoJsonLdFound => "Could not find recipe data on this page".to_string(),
            ParseError::NoRecipeFound => "Could not find recipe data on this page".to_string(),
            ParseError::MultipleRecipesFound => {
                "This page contains multiple recipes. Please try a more specific URL".to_string()
            }
            ParseError::MalformedRecipe(msg) => {
                format!("The recipe data on this page could not be read: {}", msg)
            }
        };
        log::error!("[cid:{}] import_recipe_from_url parse failed: {}", cid, msg);
        msg
    })?;

    // Convert ParsedRecipe to RecipeInput
    let input = parsed_to_input(parsed, url);

    // Create recipe
    let result = recipes::create_recipe(input).await.map_err(|e| e.into());

    let elapsed = start.elapsed();
    match &result {
        Ok(recipe) => log::info!("[cid:{}] import_recipe_from_url succeeded in {:?}, id={}", cid, elapsed, recipe.id),
        Err(e) => log::error!("[cid:{}] import_recipe_from_url failed in {:?}: {}", cid, elapsed, e),
    }
    result
}

/// Check if a recipe with this source_url already exists
async fn recipe_exists_by_source_url(url: &str) -> Result<bool, String> {
    let pool = get_db_pool();
    let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM recipes WHERE source_url = ?")
        .bind(url)
        .fetch_one(pool)
        .await
        .map_err(|e| format!("Database error: {}", e))?;

    Ok(count > 0)
}

/// Convert ParsedRecipe to RecipeInput
fn parsed_to_input(parsed: ParsedRecipe, source_url: &str) -> RecipeInput {
    RecipeInput {
        name: parsed.name,
        description: parsed.description,
        prep_time: parsed.prep_time,
        cook_time: parsed.cook_time,
        servings: parsed.servings,
        image_path: parsed.image_url,
        source_url: Some(source_url.to_string()),
        notes: None,
        tags: vec![],
        ingredients: parsed
            .ingredients
            .into_iter()
            .map(|i| IngredientInput {
                name: i.name,
                quantity: i.quantity,
                unit: i.unit,
                category: None,
                notes: None,
            })
            .collect(),
        instructions: parsed.instructions,
    }
}
