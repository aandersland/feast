//! Logging configuration and JSON formatter
//!
//! This module provides:
//! - `LogConfig` for runtime-configurable logging settings
//! - JSON formatter for structured log output
//! - Default configuration with sensible values

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// Maximum log file size in bytes (10 MB)
pub const MAX_LOG_FILE_SIZE: u64 = 10 * 1024 * 1024;

/// Number of rotated log files to keep
pub const ROTATION_FILE_COUNT: usize = 5;

/// Default log file name
pub const LOG_FILE_NAME: &str = "feast";

/// Logging configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogConfig {
    /// Global default log level
    #[serde(default = "default_level")]
    pub default_level: String,

    /// Per-module log level overrides
    /// Key: module path (e.g., "feast_lib::db")
    /// Value: log level (trace, debug, info, warn, error)
    #[serde(default)]
    pub module_levels: HashMap<String, String>,

    /// Whether to output to console (stdout)
    #[serde(default = "default_console_enabled")]
    pub console_enabled: bool,

    /// Whether to output to log file
    #[serde(default = "default_file_enabled")]
    pub file_enabled: bool,

    /// Whether this is a development build (more verbose console)
    #[serde(default)]
    pub dev_mode: bool,
}

fn default_level() -> String {
    "info".to_string()
}

fn default_console_enabled() -> bool {
    true
}

fn default_file_enabled() -> bool {
    true
}

impl Default for LogConfig {
    fn default() -> Self {
        Self {
            default_level: default_level(),
            module_levels: HashMap::new(),
            console_enabled: true,
            file_enabled: true,
            dev_mode: cfg!(debug_assertions),
        }
    }
}

impl LogConfig {
    /// Load config from file, falling back to defaults
    pub fn load(config_dir: &Path) -> Self {
        let config_path = config_dir.join("logging.json");

        if config_path.exists() {
            match fs::read_to_string(&config_path) {
                Ok(contents) => match serde_json::from_str(&contents) {
                    Ok(config) => return config,
                    Err(e) => {
                        eprintln!("Failed to parse logging config: {e}");
                    }
                },
                Err(e) => {
                    eprintln!("Failed to read logging config: {e}");
                }
            }
        }

        Self::default()
    }

    /// Parse log level string to LevelFilter
    pub fn parse_level(level: &str) -> log::LevelFilter {
        match level.to_lowercase().as_str() {
            "trace" => log::LevelFilter::Trace,
            "debug" => log::LevelFilter::Debug,
            "info" => log::LevelFilter::Info,
            "warn" | "warning" => log::LevelFilter::Warn,
            "error" => log::LevelFilter::Error,
            "off" => log::LevelFilter::Off,
            _ => log::LevelFilter::Info,
        }
    }
}

/// JSON log record structure
#[derive(Serialize)]
struct JsonLogRecord<'a> {
    timestamp: String,
    level: &'a str,
    target: &'a str,
    message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    file: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    line: Option<u32>,
}

/// Format a log record as JSON
///
/// Output format:
/// ```json
/// {"timestamp":"2025-12-17T10:30:00.123Z","level":"INFO","target":"feast_lib::db","message":"..."}
/// ```
pub fn json_format(
    out: tauri_plugin_log::fern::FormatCallback,
    message: &std::fmt::Arguments,
    record: &log::Record,
) {
    let now = chrono::Utc::now();
    let log_record = JsonLogRecord {
        timestamp: now.to_rfc3339_opts(chrono::SecondsFormat::Millis, true),
        level: record.level().as_str(),
        target: record.target(),
        message: message.to_string(),
        file: record.file(),
        line: record.line(),
    };

    let json = serde_json::to_string(&log_record).unwrap_or_else(|_| {
        format!(
            r#"{{"timestamp":"{}","level":"ERROR","target":"logging","message":"Failed to serialize log record"}}"#,
            now.to_rfc3339()
        )
    });

    out.finish(format_args!("{json}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = LogConfig::default();
        assert_eq!(config.default_level, "info");
        assert!(config.console_enabled);
        assert!(config.file_enabled);
        assert!(config.module_levels.is_empty());
    }

    #[test]
    fn test_parse_level() {
        assert_eq!(LogConfig::parse_level("trace"), log::LevelFilter::Trace);
        assert_eq!(LogConfig::parse_level("DEBUG"), log::LevelFilter::Debug);
        assert_eq!(LogConfig::parse_level("Info"), log::LevelFilter::Info);
        assert_eq!(LogConfig::parse_level("WARN"), log::LevelFilter::Warn);
        assert_eq!(LogConfig::parse_level("warning"), log::LevelFilter::Warn);
        assert_eq!(LogConfig::parse_level("error"), log::LevelFilter::Error);
        assert_eq!(LogConfig::parse_level("off"), log::LevelFilter::Off);
        assert_eq!(LogConfig::parse_level("invalid"), log::LevelFilter::Info);
    }

    #[test]
    fn test_config_deserialize() {
        let json = r#"{
            "default_level": "debug",
            "module_levels": {
                "feast_lib::db": "trace",
                "feast_lib::commands": "info"
            },
            "console_enabled": false,
            "file_enabled": true,
            "dev_mode": true
        }"#;

        let config: LogConfig = serde_json::from_str(json).unwrap();
        assert_eq!(config.default_level, "debug");
        assert_eq!(config.module_levels.get("feast_lib::db"), Some(&"trace".to_string()));
        assert!(!config.console_enabled);
        assert!(config.file_enabled);
        assert!(config.dev_mode);
    }

    #[test]
    fn test_config_deserialize_partial() {
        // Only override some fields, rest should be defaults
        let json = r#"{"default_level": "warn"}"#;

        let config: LogConfig = serde_json::from_str(json).unwrap();
        assert_eq!(config.default_level, "warn");
        assert!(config.console_enabled); // default
        assert!(config.file_enabled); // default
    }

    #[test]
    fn test_json_format_output() {
        // We can't easily create log::Record in tests
        // so we test the JsonLogRecord serialization directly
        let record = JsonLogRecord {
            timestamp: "2025-12-17T10:30:00.000Z".to_string(),
            level: "INFO",
            target: "test_module",
            message: "Test message".to_string(),
            file: Some("test.rs"),
            line: Some(42),
        };

        let json = serde_json::to_string(&record).unwrap();
        assert!(json.contains("\"level\":\"INFO\""));
        assert!(json.contains("\"target\":\"test_module\""));
        assert!(json.contains("\"message\":\"Test message\""));
        assert!(json.contains("\"file\":\"test.rs\""));
        assert!(json.contains("\"line\":42"));
    }

    #[test]
    fn test_load_missing_config() {
        let temp_dir = std::env::temp_dir().join("feast_test_logging_missing");
        let _ = std::fs::remove_dir_all(&temp_dir);
        std::fs::create_dir_all(&temp_dir).unwrap();

        let config = LogConfig::load(&temp_dir);
        // Should return defaults when file doesn't exist
        assert_eq!(config.default_level, "info");

        let _ = std::fs::remove_dir_all(&temp_dir);
    }

    #[test]
    fn test_load_valid_config() {
        let temp_dir = std::env::temp_dir().join("feast_test_logging_valid");
        let _ = std::fs::remove_dir_all(&temp_dir);
        std::fs::create_dir_all(&temp_dir).unwrap();

        let config_path = temp_dir.join("logging.json");
        fs::write(&config_path, r#"{"default_level": "debug"}"#).unwrap();

        let config = LogConfig::load(&temp_dir);
        assert_eq!(config.default_level, "debug");

        let _ = std::fs::remove_dir_all(&temp_dir);
    }

    #[test]
    fn test_load_invalid_config() {
        let temp_dir = std::env::temp_dir().join("feast_test_logging_invalid");
        let _ = std::fs::remove_dir_all(&temp_dir);
        std::fs::create_dir_all(&temp_dir).unwrap();

        let config_path = temp_dir.join("logging.json");
        fs::write(&config_path, "not valid json").unwrap();

        let config = LogConfig::load(&temp_dir);
        // Should return defaults when file is invalid
        assert_eq!(config.default_level, "info");

        let _ = std::fs::remove_dir_all(&temp_dir);
    }

    #[test]
    fn test_json_format_integration() {
        // Test that json_format produces valid JSON
        // We can't easily invoke json_format with a real log::Record
        // but we can test the JsonLogRecord serialization
        let record = JsonLogRecord {
            timestamp: chrono::Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Millis, true),
            level: "INFO",
            target: "feast_lib::test",
            message: "Test with special chars: \"quotes\" and \\backslash\\".to_string(),
            file: Some("test.rs"),
            line: Some(100),
        };

        let json = serde_json::to_string(&record).unwrap();

        // Verify it's valid JSON by parsing it back
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed["level"], "INFO");
        assert_eq!(parsed["target"], "feast_lib::test");
        assert!(parsed["message"].as_str().unwrap().contains("quotes"));
    }

    #[test]
    fn test_constants() {
        // Verify constants are set correctly
        assert_eq!(MAX_LOG_FILE_SIZE, 10 * 1024 * 1024); // 10 MB
        assert_eq!(ROTATION_FILE_COUNT, 5);
        assert_eq!(LOG_FILE_NAME, "feast");
    }
}
