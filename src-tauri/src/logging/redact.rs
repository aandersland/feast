//! Sensitive data redaction for logging
//!
//! Provides utilities to redact user data, file paths, and other
//! sensitive information before logging.

/// Maximum length for logged string values before truncation
const MAX_LOG_STRING_LENGTH: usize = 100;

/// Redact a potentially sensitive string value
///
/// - Truncates long strings (e.g., recipe instructions)
/// - Returns placeholder for None values
pub fn redact_string(value: Option<&str>, field_name: &str) -> String {
    match value {
        None => format!("{}=None", field_name),
        Some("") => format!("{}=\"\"", field_name),
        Some(s) if s.len() > MAX_LOG_STRING_LENGTH => {
            format!("{}=\"{}...\" ({} chars)", field_name, &s[..50], s.len())
        }
        Some(s) => format!("{}=\"{}\"", field_name, s),
    }
}

/// Redact a file path (may contain username/directory structure)
///
/// Shows only the filename, not full path
pub fn redact_path(path: Option<&str>) -> String {
    match path {
        None => "path=None".to_string(),
        Some(p) => {
            let filename = std::path::Path::new(p)
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("<invalid>");
            format!("path=\"...{}\"", filename)
        }
    }
}

/// Redact user-entered content (notes, descriptions)
///
/// Shows length only, not content (may contain personal info)
pub fn redact_user_content(value: Option<&str>, field_name: &str) -> String {
    match value {
        None => format!("{}=None", field_name),
        Some(s) => format!("{}=<{} chars>", field_name, s.len()),
    }
}

/// Format a count result for logging
pub fn format_count(count: usize, entity: &str) -> String {
    format!("{} {}(s)", count, entity)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_redact_string_none() {
        assert_eq!(redact_string(None, "name"), "name=None");
    }

    #[test]
    fn test_redact_string_empty() {
        assert_eq!(redact_string(Some(""), "name"), "name=\"\"");
    }

    #[test]
    fn test_redact_string_short() {
        assert_eq!(redact_string(Some("hello"), "name"), "name=\"hello\"");
    }

    #[test]
    fn test_redact_string_long() {
        let long = "a".repeat(150);
        let result = redact_string(Some(&long), "desc");
        assert!(result.contains("..."));
        assert!(result.contains("150 chars"));
    }

    #[test]
    fn test_redact_path() {
        assert_eq!(redact_path(Some("/home/user/photos/recipe.jpg")), "path=\"...recipe.jpg\"");
        assert_eq!(redact_path(None), "path=None");
    }

    #[test]
    fn test_redact_user_content() {
        assert_eq!(redact_user_content(Some("my secret notes"), "notes"), "notes=<15 chars>");
        assert_eq!(redact_user_content(None, "notes"), "notes=None");
    }
}
