//! Correlation ID utilities for request tracing
//!
//! Generates short, unique IDs for tracing requests through the stack.

/// Length of generated correlation IDs (8 hex chars = 32 bits)
const CORRELATION_ID_LENGTH: usize = 8;

/// Generate a new correlation ID
///
/// Returns a short, URL-safe string suitable for log filtering.
/// Format: 8 hex characters from UUID v4 (e.g., "a1b2c3d4")
pub fn generate_correlation_id() -> String {
    uuid::Uuid::new_v4().to_string().replace("-", "")[..CORRELATION_ID_LENGTH].to_string()
}

/// Ensure a correlation ID exists, generating one if needed
///
/// Use this at command entry points to guarantee all operations
/// have a correlation ID for tracing.
pub fn ensure_correlation_id(id: Option<String>) -> String {
    match id {
        Some(id) if !id.is_empty() => id,
        _ => generate_correlation_id(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_correlation_id_length() {
        let id = generate_correlation_id();
        assert_eq!(id.len(), CORRELATION_ID_LENGTH);
    }

    #[test]
    fn test_generate_correlation_id_uniqueness() {
        let id1 = generate_correlation_id();
        let id2 = generate_correlation_id();
        assert_ne!(id1, id2);
    }

    #[test]
    fn test_ensure_correlation_id_with_existing() {
        let existing = "abc12345".to_string();
        let result = ensure_correlation_id(Some(existing.clone()));
        assert_eq!(result, existing);
    }

    #[test]
    fn test_ensure_correlation_id_with_none() {
        let result = ensure_correlation_id(None);
        assert_eq!(result.len(), CORRELATION_ID_LENGTH);
    }

    #[test]
    fn test_ensure_correlation_id_with_empty() {
        let result = ensure_correlation_id(Some("".to_string()));
        assert_eq!(result.len(), CORRELATION_ID_LENGTH);
    }
}
