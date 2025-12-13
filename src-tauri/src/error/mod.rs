//! Application error types

use serde::Serialize;
use thiserror::Error;

/// Application error type
#[derive(Debug, Error)]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

/// Serializable error for frontend
#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub code: String,
    pub message: String,
}

impl From<&AppError> for ErrorResponse {
    fn from(err: &AppError) -> Self {
        match err {
            AppError::Database(_) => ErrorResponse {
                code: "DATABASE_ERROR".to_string(),
                message: "A database error occurred.".to_string(),
            },
            AppError::NotFound(msg) => ErrorResponse {
                code: "NOT_FOUND".to_string(),
                message: msg.clone(),
            },
            AppError::Validation(msg) => ErrorResponse {
                code: "VALIDATION_ERROR".to_string(),
                message: msg.clone(),
            },
            AppError::Io(_) => ErrorResponse {
                code: "IO_ERROR".to_string(),
                message: "An IO error occurred.".to_string(),
            },
        }
    }
}

impl From<AppError> for String {
    fn from(err: AppError) -> Self {
        let response = ErrorResponse::from(&err);
        serde_json::to_string(&response).unwrap_or_else(|_| err.to_string())
    }
}
