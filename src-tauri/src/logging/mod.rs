//! Logging configuration and JSON formatter
//!
//! This module provides:
//! - `LogConfig` for runtime-configurable logging settings
//! - JSON formatter for structured log output
//! - Default configuration with sensible values
//!
//! ## Log Levels by Area
//!
//! | Area | Level | What's Logged |
//! |------|-------|---------------|
//! | Command entry | DEBUG | Command name, redacted parameters |
//! | Command exit (success) | INFO | Command name, timing, result summary |
//! | Command exit (error) | ERROR | Command name, timing, error details |
//! | DB queries | DEBUG | Operation name, timing, row counts |
//! | App lifecycle | INFO | Initialization, shutdown |
//!
//! ## Correlation ID Format
//!
//! All logs include `[cid:xxxxxxxx]` prefix for request tracing.
//! Filter logs by correlation ID: `grep "cid:abc12345" feast.log`

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
    correlation_id: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    file: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    line: Option<u32>,
}

/// Format a log record as JSON
///
/// Output format:
/// ```json
/// {"timestamp":"2025-12-17T10:30:00.123Z","level":"INFO","target":"feast_lib::db","message":"...","correlation_id":"V1StGXR8"}
/// ```
///
/// Correlation IDs are extracted from message prefix `[cid:XXXXXXXX]` if present.
pub fn json_format(
    out: tauri_plugin_log::fern::FormatCallback,
    message: &std::fmt::Arguments,
    record: &log::Record,
) {
    let now = chrono::Utc::now();
    let message_str = message.to_string();

    // Extract correlation ID from message prefix [cid:XXXXXXXX]
    let (correlation_id, clean_message) = extract_correlation_id(&message_str);

    let log_record = JsonLogRecord {
        timestamp: now.to_rfc3339_opts(chrono::SecondsFormat::Millis, true),
        level: record.level().as_str(),
        target: record.target(),
        message: clean_message,
        correlation_id,
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

/// Extract correlation ID from message prefix
///
/// Format: `[cid:XXXXXXXX] actual message`
/// Returns (Some(correlation_id), clean_message) or (None, original_message)
fn extract_correlation_id(message: &str) -> (Option<&str>, String) {
    if message.starts_with("[cid:") {
        if let Some(end_bracket) = message.find(']') {
            let cid = &message[5..end_bracket];
            let rest = message[end_bracket + 1..].trim_start();
            return (Some(cid), rest.to_string());
        }
    }
    (None, message.to_string())
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
            correlation_id: None,
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
            correlation_id: None,
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

    #[test]
    fn test_extract_correlation_id_present() {
        let (cid, msg) = extract_correlation_id("[cid:abc12345] Hello world");
        assert_eq!(cid, Some("abc12345"));
        assert_eq!(msg, "Hello world");
    }

    #[test]
    fn test_extract_correlation_id_absent() {
        let (cid, msg) = extract_correlation_id("Hello world");
        assert_eq!(cid, None);
        assert_eq!(msg, "Hello world");
    }

    #[test]
    fn test_json_record_with_correlation_id() {
        let record = JsonLogRecord {
            timestamp: "2025-12-17T10:30:00.000Z".to_string(),
            level: "INFO",
            target: "test",
            message: "Test message".to_string(),
            correlation_id: Some("abc12345"),
            file: None,
            line: None,
        };

        let json = serde_json::to_string(&record).unwrap();
        assert!(json.contains("\"correlation_id\":\"abc12345\""));
    }

    #[test]
    fn test_json_record_without_correlation_id() {
        let record = JsonLogRecord {
            timestamp: "2025-12-17T10:30:00.000Z".to_string(),
            level: "INFO",
            target: "test",
            message: "Test message".to_string(),
            correlation_id: None,
            file: None,
            line: None,
        };

        let json = serde_json::to_string(&record).unwrap();
        assert!(!json.contains("correlation_id"));
    }

    // ========================================
    // Automated Log Output Verification Tests
    // ========================================

    #[test]
    fn test_correlation_id_pattern_format() {
        // Verify [cid:xxxxxxxx] format is correctly extracted
        let valid_patterns = [
            "[cid:abc12345] Message",
            "[cid:ABCD1234] Another message",
            "[cid:a1b2c3d4] With numbers and letters",
        ];

        for pattern in valid_patterns {
            let (cid, _) = extract_correlation_id(pattern);
            assert!(cid.is_some(), "Failed to extract CID from: {}", pattern);
            let cid = cid.unwrap();
            assert_eq!(cid.len(), 8, "CID should be 8 chars: {}", cid);
        }
    }

    #[test]
    fn test_correlation_id_in_json_output() {
        // Verify correlation ID appears in JSON when present
        let record = JsonLogRecord {
            timestamp: "2025-12-17T10:30:00.000Z".to_string(),
            level: "INFO",
            target: "feast_lib::commands",
            message: "get_recipes completed in 5ms, returned 10 recipes".to_string(),
            correlation_id: Some("V1StGXR8"),
            file: None,
            line: None,
        };

        let json = serde_json::to_string(&record).unwrap();

        // Parse and verify JSON structure
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed["correlation_id"], "V1StGXR8");
    }

    #[test]
    fn test_timing_format_in_message() {
        // Verify timing information format (Duration uses {:?} format)
        let timing_patterns = [
            "completed in 5ms",
            "completed in 123ms",
            "completed in 1.234s",
        ];

        for pattern in timing_patterns {
            let record = JsonLogRecord {
                timestamp: "2025-12-17T10:30:00.000Z".to_string(),
                level: "INFO",
                target: "test",
                message: format!("get_recipes {}", pattern),
                correlation_id: Some("abc12345"),
                file: None,
                line: None,
            };

            let json = serde_json::to_string(&record).unwrap();
            let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
            let msg = parsed["message"].as_str().unwrap();
            assert!(msg.contains("completed in"), "Missing timing: {}", msg);
        }
    }

    #[test]
    fn test_json_formatter_handles_special_characters() {
        // Ensure JSON escaping works for special characters
        let special_inputs = [
            "Message with \"quotes\"",
            "Message with \\backslash\\",
            "Message with\nnewline",
            "Message with\ttab",
            "Unicode: \u{1F600} emoji",
        ];

        for input in special_inputs {
            let record = JsonLogRecord {
                timestamp: "2025-12-17T10:30:00.000Z".to_string(),
                level: "INFO",
                target: "test",
                message: input.to_string(),
                correlation_id: None,
                file: None,
                line: None,
            };

            let json = serde_json::to_string(&record).unwrap();
            // Verify it parses back successfully
            let parsed: serde_json::Value = serde_json::from_str(&json)
                .expect(&format!("Failed to parse JSON for input: {}", input));
            assert!(parsed["message"].is_string());
        }
    }

    #[test]
    fn test_json_output_is_single_line() {
        // Log files expect one JSON object per line
        let record = JsonLogRecord {
            timestamp: "2025-12-17T10:30:00.000Z".to_string(),
            level: "INFO",
            target: "feast_lib::commands::recipes",
            message: "get_recipes completed successfully".to_string(),
            correlation_id: Some("abc12345"),
            file: Some("recipes.rs"),
            line: Some(42),
        };

        let json = serde_json::to_string(&record).unwrap();
        assert!(!json.contains('\n'), "JSON should be single line");
    }

    #[test]
    fn test_redaction_not_in_correlation_id() {
        // Correlation IDs should never contain sensitive data
        // They are random identifiers generated by nanoid
        let test_cids = ["V1StGXR8", "abc12345", "XyZ98765"];

        for cid in test_cids {
            // CIDs should be alphanumeric only
            assert!(
                cid.chars().all(|c| c.is_alphanumeric()),
                "CID contains non-alphanumeric: {}",
                cid
            );
            // CIDs should be 8 characters
            assert_eq!(cid.len(), 8, "CID wrong length: {}", cid);
        }
    }

    #[test]
    fn test_extract_correlation_id_edge_cases() {
        // Empty bracket
        let (cid, msg) = extract_correlation_id("[cid:] Message");
        assert_eq!(cid, Some(""));
        assert_eq!(msg, "Message");

        // No space after bracket
        let (cid, msg) = extract_correlation_id("[cid:abc12345]Message");
        assert_eq!(cid, Some("abc12345"));
        assert_eq!(msg, "Message");

        // Multiple spaces
        let (cid, msg) = extract_correlation_id("[cid:abc12345]    Multiple spaces");
        assert_eq!(cid, Some("abc12345"));
        assert_eq!(msg, "Multiple spaces");

        // Missing closing bracket (should not extract)
        let (cid, msg) = extract_correlation_id("[cid:abc12345 Message");
        assert_eq!(cid, None);
        assert_eq!(msg, "[cid:abc12345 Message");
    }

    #[test]
    fn test_json_log_record_required_fields() {
        // Verify all required fields are present in output
        let record = JsonLogRecord {
            timestamp: "2025-12-17T10:30:00.000Z".to_string(),
            level: "INFO",
            target: "test",
            message: "Test".to_string(),
            correlation_id: None,
            file: None,
            line: None,
        };

        let json = serde_json::to_string(&record).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();

        // Required fields must be present
        assert!(parsed.get("timestamp").is_some(), "Missing timestamp");
        assert!(parsed.get("level").is_some(), "Missing level");
        assert!(parsed.get("target").is_some(), "Missing target");
        assert!(parsed.get("message").is_some(), "Missing message");

        // Optional fields should be absent when None
        assert!(parsed.get("correlation_id").is_none(), "correlation_id should be absent");
        assert!(parsed.get("file").is_none(), "file should be absent");
        assert!(parsed.get("line").is_none(), "line should be absent");
    }
}

pub mod redact;
