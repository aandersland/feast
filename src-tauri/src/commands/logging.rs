//! Frontend logging command handlers

use serde::Deserialize;
use std::collections::HashMap;
use tauri::command;

/// Input for frontend log entries
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FrontendLogEntry {
    /// Log level: trace, debug, info, warn, error
    pub level: String,
    /// Log message
    pub message: String,
    /// Source target (e.g., "frontend::ShoppingList")
    pub target: String,
    /// Optional correlation ID for request tracing
    #[serde(default)]
    pub correlation_id: Option<String>,
    /// Optional structured data as key-value pairs
    #[serde(default)]
    pub data: HashMap<String, serde_json::Value>,
}

/// Receive log entries from the frontend
///
/// Logs are written through the backend logging system with the same
/// JSON structure, enabling unified filtering and analysis.
#[command]
pub fn log_from_frontend(entries: Vec<FrontendLogEntry>) -> Result<(), String> {
    for entry in entries {
        let base_message = if entry.data.is_empty() {
            entry.message
        } else {
            // Append structured data as JSON
            match serde_json::to_string(&entry.data) {
                Ok(data_json) => format!("{} | {}", entry.message, data_json),
                Err(_) => entry.message,
            }
        };

        // Prepend correlation ID if present
        let message = match &entry.correlation_id {
            Some(cid) if !cid.is_empty() => format!("[cid:{}] {}", cid, base_message),
            _ => base_message,
        };

        match entry.level.to_lowercase().as_str() {
            "trace" => log::trace!(target: &entry.target, "{}", message),
            "debug" => log::debug!(target: &entry.target, "{}", message),
            "info" => log::info!(target: &entry.target, "{}", message),
            "warn" => log::warn!(target: &entry.target, "{}", message),
            "error" => log::error!(target: &entry.target, "{}", message),
            _ => log::info!(target: &entry.target, "{}", message),
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_frontend_log_entry_deserialize() {
        let json = r#"{
            "level": "info",
            "message": "Test message",
            "target": "frontend::TestComponent"
        }"#;

        let entry: FrontendLogEntry = serde_json::from_str(json).unwrap();
        assert_eq!(entry.level, "info");
        assert_eq!(entry.message, "Test message");
        assert_eq!(entry.target, "frontend::TestComponent");
        assert!(entry.data.is_empty());
        assert!(entry.correlation_id.is_none());
    }

    #[test]
    fn test_frontend_log_entry_with_correlation_id() {
        let json = r#"{
            "level": "info",
            "message": "Test message",
            "target": "frontend::TestComponent",
            "correlationId": "abc12345"
        }"#;

        let entry: FrontendLogEntry = serde_json::from_str(json).unwrap();
        assert_eq!(entry.correlation_id, Some("abc12345".to_string()));
    }

    #[test]
    fn test_frontend_log_entry_with_data() {
        let json = r#"{
            "level": "debug",
            "message": "User action",
            "target": "frontend::UserStore",
            "data": {
                "userId": 123,
                "action": "login"
            }
        }"#;

        let entry: FrontendLogEntry = serde_json::from_str(json).unwrap();
        assert_eq!(entry.level, "debug");
        assert_eq!(entry.data.get("userId"), Some(&serde_json::json!(123)));
        assert_eq!(entry.data.get("action"), Some(&serde_json::json!("login")));
    }

    #[test]
    fn test_log_from_frontend_empty() {
        // Empty batch should succeed
        let result = log_from_frontend(vec![]);
        assert!(result.is_ok());
    }
}
