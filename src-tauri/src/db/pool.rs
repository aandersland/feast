//! Database connection pool management

use crate::error::AppError;
use sqlx::sqlite::{SqliteConnectOptions, SqlitePool, SqlitePoolOptions};
use std::path::Path;
use std::sync::OnceLock;

/// Global database pool
static DB_POOL: OnceLock<SqlitePool> = OnceLock::new();

/// Initialize the database connection pool
pub async fn init_db(app_data_dir: &Path) -> Result<(), AppError> {
    std::fs::create_dir_all(app_data_dir)?;

    let db_path = app_data_dir.join("feast.db");

    let options = SqliteConnectOptions::new()
        .filename(&db_path)
        .create_if_missing(true);

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(options)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

    run_migrations(&pool).await?;

    DB_POOL
        .set(pool)
        .map_err(|_| AppError::Database("Database already initialized".to_string()))?;

    log::info!("Database initialized at {:?}", db_path);
    Ok(())
}

/// Run database migrations
async fn run_migrations(pool: &SqlitePool) -> Result<(), AppError> {
    sqlx::migrate!("./migrations")
        .run(pool)
        .await
        .map_err(|e| AppError::Database(format!("Migration failed: {e}")))?;

    log::info!("Database migrations completed");
    Ok(())
}

/// Get the database pool
pub fn get_db_pool() -> &'static SqlitePool {
    DB_POOL
        .get()
        .expect("Database not initialized. Call init_db first.")
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::sqlite::SqlitePoolOptions;

    #[tokio::test]
    async fn test_migrations_run_successfully() {
        let pool = SqlitePoolOptions::new()
            .max_connections(1)
            .connect("sqlite::memory:")
            .await
            .expect("Failed to create in-memory database");

        let result = run_migrations(&pool).await;
        assert!(result.is_ok(), "Migrations should run successfully");
    }

    #[tokio::test]
    async fn test_migrations_are_idempotent() {
        let pool = SqlitePoolOptions::new()
            .max_connections(1)
            .connect("sqlite::memory:")
            .await
            .expect("Failed to create in-memory database");

        // Run migrations twice - should not fail
        run_migrations(&pool).await.expect("First migration run failed");
        let result = run_migrations(&pool).await;
        assert!(result.is_ok(), "Running migrations twice should succeed");
    }

    #[tokio::test]
    async fn test_items_table_created() {
        let pool = SqlitePoolOptions::new()
            .max_connections(1)
            .connect("sqlite::memory:")
            .await
            .expect("Failed to create in-memory database");

        run_migrations(&pool).await.expect("Migrations failed");

        // Verify items table exists by querying it
        let result = sqlx::query("SELECT COUNT(*) as count FROM items")
            .fetch_one(&pool)
            .await;

        assert!(result.is_ok(), "Items table should exist after migrations");
    }
}
