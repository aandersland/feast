//! Ingredient database operations

use crate::db::pool::get_db_pool;
use crate::error::AppError;
use serde::Serialize;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct Ingredient {
    pub id: String,
    pub name: String,
    pub category: String,
    pub default_unit: Option<String>,
}

/// Get all ingredients
pub async fn get_all_ingredients() -> Result<Vec<Ingredient>, AppError> {
    let pool = get_db_pool();

    sqlx::query_as::<_, Ingredient>(
        "SELECT id, name, category, default_unit FROM ingredients ORDER BY name",
    )
    .fetch_all(pool)
    .await
    .map_err(|e| AppError::Database(e.to_string()))
}

/// Create a new ingredient
pub async fn create_ingredient(
    name: &str,
    category: &str,
    default_unit: Option<&str>,
) -> Result<Ingredient, AppError> {
    let pool = get_db_pool();
    let id = Uuid::new_v4().to_string();
    let normalized_name = name.trim().to_lowercase();

    sqlx::query("INSERT INTO ingredients (id, name, category, default_unit) VALUES (?, ?, ?, ?)")
        .bind(&id)
        .bind(&normalized_name)
        .bind(category)
        .bind(default_unit)
        .execute(pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

    Ok(Ingredient {
        id,
        name: normalized_name,
        category: category.to_string(),
        default_unit: default_unit.map(String::from),
    })
}

/// Get or create an ingredient by name (for recipe creation)
pub async fn get_or_create_ingredient(
    name: &str,
    category: &str,
    default_unit: Option<&str>,
) -> Result<Ingredient, AppError> {
    let pool = get_db_pool();
    let normalized_name = name.trim().to_lowercase();

    // Try to find existing
    let existing = sqlx::query_as::<_, Ingredient>(
        "SELECT id, name, category, default_unit FROM ingredients WHERE name = ?",
    )
    .bind(&normalized_name)
    .fetch_optional(pool)
    .await
    .map_err(|e| AppError::Database(e.to_string()))?;

    if let Some(ingredient) = existing {
        return Ok(ingredient);
    }

    // Create new
    create_ingredient(&normalized_name, category, default_unit).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::pool::init_db_for_test;

    #[tokio::test]
    async fn test_create_ingredient() {
        init_db_for_test().await;

        let ingredient = create_ingredient("Chicken Breast", "Meat & Seafood", Some("lb"))
            .await
            .unwrap();

        assert_eq!(ingredient.name, "chicken breast"); // normalized
        assert_eq!(ingredient.category, "Meat & Seafood");
        assert_eq!(ingredient.default_unit, Some("lb".to_string()));
    }

    #[tokio::test]
    async fn test_get_or_create_ingredient() {
        init_db_for_test().await;

        let ing1 = get_or_create_ingredient("Tomato", "Produce", None)
            .await
            .unwrap();
        let ing2 = get_or_create_ingredient("TOMATO", "Produce", None)
            .await
            .unwrap();

        assert_eq!(ing1.id, ing2.id); // Same ingredient returned
    }
}
