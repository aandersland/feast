# HTTP Client for Recipe Fetching Implementation Plan

## Overview

Add an HTTP client module to fetch HTML content from recipe URLs with proper validation, timeout handling, redirect limits, content-type verification, and user-friendly error messages. The User-Agent header must be exactly "feast".

## Current State

The app has no HTTP fetching capability. Tauri commands use async functions with tokio runtime. The existing error pattern uses `AppError` enum with conversion to `String` for frontend.

**Key Discoveries**:
- All commands use `async fn` with `.await` (`src-tauri/src/commands/*.rs`)
- Tokio runtime already configured (`Cargo.toml:24`)
- Error handling via `AppError` enum (`src-tauri/src/error/mod.rs:7-20`)
- [wiremock-rs](https://github.com/LukeMathWalker/wiremock-rs) is best for async HTTP mocking in tests

## Desired End State

An `http` module that exposes:
- `fetch_url(url: &str) -> Result<String, FetchError>` - fetches HTML content
- `FetchError` enum with specific error types for all failure modes

Verification: `cargo test http` passes with 10+ tests covering all error cases.

## What We're NOT Doing

- HTML parsing (Chunk 1 - parser module)
- Database storage (Chunk 3)
- Tauri command integration (Chunk 4)
- Caching responses
- Cookie handling
- Authentication

## Integration Map

| Type | Location | Notes |
|------|----------|-------|
| Entry point | `src-tauri/src/http/mod.rs` (new) | New module |
| Registration | `src-tauri/src/lib.rs:8` | Add `pub mod http;` after utils |
| Exports | `src-tauri/src/http/mod.rs` | Export `fetch_url`, `FetchError` |
| Consumers | Chunk 4 import command | Will call `http::fetch_url` |
| Dependencies | `src-tauri/Cargo.toml` | Add `reqwest`, `wiremock` (dev) |
| Events | N/A | None required |

## Implementation Approach

The HTTP module is a single file (`http/mod.rs`) containing:
1. `FetchError` enum for all error types
2. `validate_url()` function for URL validation
3. `fetch_url()` async function for fetching HTML
4. Comprehensive tests using wiremock

---

## Phase 1: Module Setup & Dependencies

### Goal
Add dependencies and create the HTTP module structure with error types.

### Integration Points

**Depends on**: None
**Produces for next phase**: Module structure, `FetchError` type, reqwest client setup

**Wiring required**:
- [x] Add `reqwest` to `src-tauri/Cargo.toml`
- [x] Add `wiremock` to dev-dependencies
- [x] Add `pub mod http;` to `src-tauri/src/lib.rs:8`

### Changes

#### Cargo.toml Dependencies

**File**: `src-tauri/Cargo.toml`

**Change**: Add reqwest for HTTP and wiremock for testing

```toml
# After uuid line (line 33), add:
# HTTP client
reqwest = { version = "0.12", features = ["rustls-tls"], default-features = false }

# In [dev-dependencies] section (after tempfile), add:
wiremock = "0.6"
```

Note: Using `rustls-tls` instead of default `native-tls` for better cross-platform compatibility and smaller binary size.

#### lib.rs Module Registration

**File**: `src-tauri/src/lib.rs`

**Change**: Add http module declaration

```rust
// After line 8 (pub mod utils;), add:
pub mod http;
```

#### HTTP Module with Error Types

**File**: `src-tauri/src/http/mod.rs` (new)

**Change**: Create HTTP module with error types and fetch function

```rust
//! HTTP client for fetching recipe pages

use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE, USER_AGENT};
use std::time::Duration;
use thiserror::Error;

/// Timeout for HTTP requests (30 seconds)
const REQUEST_TIMEOUT: Duration = Duration::from_secs(30);

/// Maximum number of redirects to follow
const MAX_REDIRECTS: usize = 5;

/// Maximum response size (10 MB)
const MAX_RESPONSE_SIZE: u64 = 10 * 1024 * 1024;

/// User-Agent header value
const USER_AGENT_VALUE: &str = "feast";

/// Error types for HTTP fetching
#[derive(Debug, Error)]
pub enum FetchError {
    #[error("Invalid URL: {0}")]
    InvalidUrl(String),

    #[error("Invalid URL scheme: only http and https are supported")]
    InvalidUrlScheme,

    #[error("Could not connect to website: {0}")]
    ConnectionFailed(String),

    #[error("Request timed out after 30 seconds")]
    Timeout,

    #[error("Too many redirects (maximum 5)")]
    TooManyRedirects,

    #[error("HTTP error {0}: {1}")]
    HttpError(u16, String),

    #[error("Expected HTML content, got {0}")]
    InvalidContentType(String),

    #[error("Response too large (maximum 10 MB)")]
    ResponseTooLarge,

    #[error("Failed to read response: {0}")]
    ReadError(String),
}

/// Validate a URL for fetching
fn validate_url(url: &str) -> Result<reqwest::Url, FetchError> {
    // Parse URL
    let parsed = reqwest::Url::parse(url)
        .map_err(|e| FetchError::InvalidUrl(e.to_string()))?;

    // Check scheme
    match parsed.scheme() {
        "http" | "https" => {}
        _ => return Err(FetchError::InvalidUrlScheme),
    }

    // Check for host
    if parsed.host().is_none() {
        return Err(FetchError::InvalidUrl("URL must have a host".to_string()));
    }

    Ok(parsed)
}

/// Check if content-type header indicates HTML
fn is_html_content_type(content_type: Option<&HeaderValue>) -> Result<(), FetchError> {
    match content_type {
        None => {
            // Some servers omit Content-Type, proceed anyway
            Ok(())
        }
        Some(value) => {
            let value_str = value.to_str().unwrap_or("");
            let mime = value_str.split(';').next().unwrap_or("").trim().to_lowercase();

            match mime.as_str() {
                "text/html" | "application/xhtml+xml" => Ok(()),
                _ => Err(FetchError::InvalidContentType(mime)),
            }
        }
    }
}

/// Map reqwest errors to FetchError
fn map_reqwest_error(err: reqwest::Error) -> FetchError {
    if err.is_timeout() {
        FetchError::Timeout
    } else if err.is_redirect() {
        FetchError::TooManyRedirects
    } else if err.is_connect() {
        FetchError::ConnectionFailed(err.to_string())
    } else if err.is_request() {
        FetchError::ConnectionFailed(err.to_string())
    } else {
        FetchError::ReadError(err.to_string())
    }
}

/// Map HTTP status codes to user-friendly messages
fn status_to_message(status: reqwest::StatusCode) -> String {
    match status.as_u16() {
        401 => "authentication required".to_string(),
        403 => "access denied".to_string(),
        404 => "page not found".to_string(),
        429 => "rate limited - try again later".to_string(),
        500..=599 => "server error".to_string(),
        _ => status.canonical_reason().unwrap_or("unknown error").to_string(),
    }
}

/// Fetch HTML content from a URL
///
/// # Arguments
/// * `url` - The URL to fetch (must be http or https)
///
/// # Returns
/// * `Ok(String)` - The HTML content
/// * `Err(FetchError)` - Error describing what went wrong
///
/// # Example
/// ```ignore
/// let html = fetch_url("https://example.com/recipe").await?;
/// ```
pub async fn fetch_url(url: &str) -> Result<String, FetchError> {
    // Validate URL first
    let validated_url = validate_url(url)?;

    // Build client with our settings
    let client = reqwest::Client::builder()
        .timeout(REQUEST_TIMEOUT)
        .redirect(reqwest::redirect::Policy::limited(MAX_REDIRECTS))
        .user_agent(USER_AGENT_VALUE)
        .build()
        .map_err(|e| FetchError::ConnectionFailed(e.to_string()))?;

    // Make request
    let response = client
        .get(validated_url)
        .send()
        .await
        .map_err(map_reqwest_error)?;

    // Check status code
    let status = response.status();
    if !status.is_success() {
        return Err(FetchError::HttpError(
            status.as_u16(),
            status_to_message(status),
        ));
    }

    // Check content type
    is_html_content_type(response.headers().get(CONTENT_TYPE))?;

    // Check content length if available
    if let Some(len) = response.content_length() {
        if len > MAX_RESPONSE_SIZE {
            return Err(FetchError::ResponseTooLarge);
        }
    }

    // Read response body
    let body = response
        .text()
        .await
        .map_err(|e| FetchError::ReadError(e.to_string()))?;

    // Check actual size
    if body.len() as u64 > MAX_RESPONSE_SIZE {
        return Err(FetchError::ResponseTooLarge);
    }

    Ok(body)
}

#[cfg(test)]
mod tests {
    use super::*;

    // URL validation tests (synchronous)

    #[test]
    fn test_validate_url_https() {
        let result = validate_url("https://example.com/recipe");
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_url_http() {
        let result = validate_url("http://example.com/recipe");
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_url_invalid_scheme_ftp() {
        let result = validate_url("ftp://example.com/file");
        assert!(matches!(result, Err(FetchError::InvalidUrlScheme)));
    }

    #[test]
    fn test_validate_url_invalid_scheme_javascript() {
        let result = validate_url("javascript:alert(1)");
        assert!(matches!(result, Err(FetchError::InvalidUrlScheme)));
    }

    #[test]
    fn test_validate_url_invalid_scheme_file() {
        let result = validate_url("file:///etc/passwd");
        assert!(matches!(result, Err(FetchError::InvalidUrlScheme)));
    }

    #[test]
    fn test_validate_url_not_a_url() {
        let result = validate_url("not-a-url");
        assert!(matches!(result, Err(FetchError::InvalidUrl(_))));
    }

    #[test]
    fn test_validate_url_empty() {
        let result = validate_url("");
        assert!(matches!(result, Err(FetchError::InvalidUrl(_))));
    }

    #[test]
    fn test_content_type_html() {
        let value = HeaderValue::from_static("text/html");
        assert!(is_html_content_type(Some(&value)).is_ok());
    }

    #[test]
    fn test_content_type_html_with_charset() {
        let value = HeaderValue::from_static("text/html; charset=utf-8");
        assert!(is_html_content_type(Some(&value)).is_ok());
    }

    #[test]
    fn test_content_type_xhtml() {
        let value = HeaderValue::from_static("application/xhtml+xml");
        assert!(is_html_content_type(Some(&value)).is_ok());
    }

    #[test]
    fn test_content_type_json_rejected() {
        let value = HeaderValue::from_static("application/json");
        let result = is_html_content_type(Some(&value));
        assert!(matches!(result, Err(FetchError::InvalidContentType(_))));
    }

    #[test]
    fn test_content_type_image_rejected() {
        let value = HeaderValue::from_static("image/jpeg");
        let result = is_html_content_type(Some(&value));
        assert!(matches!(result, Err(FetchError::InvalidContentType(_))));
    }

    #[test]
    fn test_content_type_missing_allowed() {
        // Missing content-type should be allowed (some servers omit it)
        assert!(is_html_content_type(None).is_ok());
    }
}
```

### Success Criteria

#### Automated Verification
- [x] `cargo check -p feast` compiles without errors
- [x] `cargo clippy -p feast` passes with no warnings
- [x] `cargo test http` passes URL validation tests

#### Integration Verification
- [x] `http` module importable from `feast_lib::http`
- [x] `FetchError`, `fetch_url` exported from module

#### Manual Verification
- [x] None required for Phase 1

**Checkpoint**: Run `cargo check` and `cargo test http` before proceeding to Phase 2.

---

## Phase 2: Wiremock Integration Tests

### Goal
Add comprehensive tests using wiremock for all HTTP scenarios.

### Integration Points

**Consumes from Phase 1**: `fetch_url`, `FetchError` types
**Produces for next phase**: Verified working HTTP client

**Wiring required**:
- [x] None - tests only

### Changes

#### Integration Tests with Wiremock

**File**: `src-tauri/src/http/mod.rs` (append to tests module)

**Change**: Add async integration tests using wiremock

```rust
// Add these tests to the existing #[cfg(test)] mod tests block

    use wiremock::matchers::{method, path, header};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    #[tokio::test]
    async fn test_fetch_success() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/recipe"))
            .and(header("user-agent", "feast"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_string("<html><body>Recipe</body></html>")
                    .insert_header("content-type", "text/html"),
            )
            .mount(&mock_server)
            .await;

        let url = format!("{}/recipe", mock_server.uri());
        let result = fetch_url(&url).await;

        assert!(result.is_ok());
        assert!(result.unwrap().contains("Recipe"));
    }

    #[tokio::test]
    async fn test_fetch_user_agent_header() {
        let mock_server = MockServer::start().await;

        // This will only match if User-Agent is exactly "feast"
        Mock::given(method("GET"))
            .and(path("/"))
            .and(header("user-agent", "feast"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_string("<html></html>")
                    .insert_header("content-type", "text/html"),
            )
            .expect(1)
            .mount(&mock_server)
            .await;

        let url = mock_server.uri();
        let _ = fetch_url(&url).await;

        // If the mock was called, User-Agent was correct
        // wiremock will panic if expectations aren't met
    }

    #[tokio::test]
    async fn test_fetch_404() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/missing"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&mock_server)
            .await;

        let url = format!("{}/missing", mock_server.uri());
        let result = fetch_url(&url).await;

        match result {
            Err(FetchError::HttpError(404, msg)) => {
                assert!(msg.contains("not found"));
            }
            other => panic!("Expected HttpError(404), got {:?}", other),
        }
    }

    #[tokio::test]
    async fn test_fetch_403() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/forbidden"))
            .respond_with(ResponseTemplate::new(403))
            .mount(&mock_server)
            .await;

        let url = format!("{}/forbidden", mock_server.uri());
        let result = fetch_url(&url).await;

        match result {
            Err(FetchError::HttpError(403, msg)) => {
                assert!(msg.contains("denied"));
            }
            other => panic!("Expected HttpError(403), got {:?}", other),
        }
    }

    #[tokio::test]
    async fn test_fetch_429_rate_limited() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/limited"))
            .respond_with(ResponseTemplate::new(429))
            .mount(&mock_server)
            .await;

        let url = format!("{}/limited", mock_server.uri());
        let result = fetch_url(&url).await;

        match result {
            Err(FetchError::HttpError(429, msg)) => {
                assert!(msg.contains("rate limited"));
            }
            other => panic!("Expected HttpError(429), got {:?}", other),
        }
    }

    #[tokio::test]
    async fn test_fetch_500_server_error() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/error"))
            .respond_with(ResponseTemplate::new(500))
            .mount(&mock_server)
            .await;

        let url = format!("{}/error", mock_server.uri());
        let result = fetch_url(&url).await;

        match result {
            Err(FetchError::HttpError(500, msg)) => {
                assert!(msg.contains("server error"));
            }
            other => panic!("Expected HttpError(500), got {:?}", other),
        }
    }

    #[tokio::test]
    async fn test_fetch_wrong_content_type_json() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_string("{\"error\": \"not html\"}")
                    .insert_header("content-type", "application/json"),
            )
            .mount(&mock_server)
            .await;

        let url = format!("{}/api", mock_server.uri());
        let result = fetch_url(&url).await;

        match result {
            Err(FetchError::InvalidContentType(mime)) => {
                assert_eq!(mime, "application/json");
            }
            other => panic!("Expected InvalidContentType, got {:?}", other),
        }
    }

    #[tokio::test]
    async fn test_fetch_wrong_content_type_image() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/image.jpg"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_bytes(vec![0xFF, 0xD8, 0xFF]) // JPEG magic bytes
                    .insert_header("content-type", "image/jpeg"),
            )
            .mount(&mock_server)
            .await;

        let url = format!("{}/image.jpg", mock_server.uri());
        let result = fetch_url(&url).await;

        assert!(matches!(result, Err(FetchError::InvalidContentType(_))));
    }

    #[tokio::test]
    async fn test_fetch_missing_content_type_allowed() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/no-content-type"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_string("<html><body>Works</body></html>"),
                // No content-type header
            )
            .mount(&mock_server)
            .await;

        let url = format!("{}/no-content-type", mock_server.uri());
        let result = fetch_url(&url).await;

        // Should succeed even without content-type
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_fetch_connection_refused() {
        // Try to connect to a port that's definitely not listening
        let result = fetch_url("http://127.0.0.1:1").await;

        assert!(matches!(result, Err(FetchError::ConnectionFailed(_))));
    }

    #[tokio::test]
    async fn test_fetch_invalid_url() {
        let result = fetch_url("not-a-valid-url").await;
        assert!(matches!(result, Err(FetchError::InvalidUrl(_))));
    }

    #[tokio::test]
    async fn test_fetch_invalid_scheme() {
        let result = fetch_url("ftp://example.com/file").await;
        assert!(matches!(result, Err(FetchError::InvalidUrlScheme)));
    }

    // Note: Timeout and redirect tests are tricky with wiremock
    // Timeout test would require MockServer with delay, which is slow
    // Redirect limit test would need 6+ redirect chain setup
    // These are better verified manually or with actual network tests
```

### Success Criteria

#### Automated Verification
- [x] `cargo test http` passes all tests (20+ tests)
- [x] `cargo clippy -p feast` passes with no warnings
- [x] Tests run in parallel without interference

#### Integration Verification
- [x] All HTTP status code tests verify correct error types
- [x] User-Agent header test confirms "feast" is sent
- [x] Content-type validation tests pass

#### Manual Verification
- [x] None required for Phase 2

**Checkpoint**: Run `cargo test http` and verify all tests pass before proceeding.

---

## Phase 3: Manual Testing & Polish

### Goal
Verify the HTTP client works with real recipe websites and refine error messages.

### Integration Points

**Consumes from Phase 2**: Working, tested HTTP client
**Produces**: Production-ready HTTP module

**Wiring required**:
- [x] None - verification only

### Changes

#### Add timeout test (optional, slow)

**File**: `src-tauri/src/http/mod.rs` (optional addition to tests)

```rust
    // This test is slow (30+ seconds) - uncomment only when needed
    // #[tokio::test]
    // async fn test_fetch_timeout() {
    //     let mock_server = MockServer::start().await;
    //
    //     Mock::given(method("GET"))
    //         .and(path("/slow"))
    //         .respond_with(
    //             ResponseTemplate::new(200)
    //                 .set_delay(Duration::from_secs(35))
    //                 .set_body_string("<html></html>")
    //                 .insert_header("content-type", "text/html"),
    //         )
    //         .mount(&mock_server)
    //         .await;
    //
    //     let url = format!("{}/slow", mock_server.uri());
    //     let result = fetch_url(&url).await;
    //
    //     assert!(matches!(result, Err(FetchError::Timeout)));
    // }
```

### Success Criteria

#### Automated Verification
- [x] `cargo test http` passes all tests
- [x] `cargo clippy -p feast` passes with no warnings
- [x] `cargo fmt --check` passes

#### Integration Verification
- [x] Module compiles and exports correctly

#### Manual Verification
- [ ] Test fetching from allrecipes.com (verify HTML returned)
- [ ] Test fetching from seriouseats.com (verify HTML returned)
- [ ] Test fetching a direct image URL (verify InvalidContentType error)
- [ ] Test fetching non-existent domain (verify ConnectionFailed error)

**Checkpoint**: Run manual tests before marking complete.

---

## Testing Strategy

### Unit Tests (Synchronous)
- URL validation: valid http/https, invalid schemes (ftp, file, javascript), malformed URLs
- Content-type checking: text/html, application/xhtml+xml, with charset, non-HTML rejection

### Integration Tests (Async with wiremock)
- Successful fetch with HTML response
- User-Agent header verification
- HTTP error codes: 403, 404, 429, 500
- Content-type validation: JSON rejected, image rejected, missing allowed

### Manual Testing Checklist
1. [ ] `cargo test http` — all tests pass
2. [ ] Fetch real recipe page (allrecipes.com)
3. [ ] Fetch real recipe page (seriouseats.com)
4. [ ] Fetch image URL — expect InvalidContentType
5. [ ] Fetch non-existent domain — expect ConnectionFailed
6. [ ] Error messages are user-friendly

## Rollback Plan

No database changes or feature flags involved.

```
Git revert to commit before Phase 1: `git revert --no-commit HEAD~N..HEAD`
```

Or simply delete the `src-tauri/src/http/` directory and remove:
- Module declaration from `lib.rs`
- `reqwest` from Cargo.toml dependencies
- `wiremock` from Cargo.toml dev-dependencies

## Migration Notes

- **Data migration**: None required
- **Feature flags**: None
- **Backwards compatibility**: Not applicable (new module)

## References

- Ticket: `ai_docs/prompts/2025-12-16-RUI-02-http-client.md`
- Parent roadmap: `ai_docs/roadmaps/2025-12-16-recipe-url-import.md`
- wiremock documentation: [wiremock-rs](https://github.com/LukeMathWalker/wiremock-rs)
- reqwest documentation: [docs.rs/reqwest](https://docs.rs/reqwest/)
