//! HTTP client module for fetching web content.
//!
//! Provides a simple, safe interface for fetching HTML content from URLs.

use std::time::Duration;

// Constants
/// Request timeout in seconds
pub const REQUEST_TIMEOUT: Duration = Duration::from_secs(30);

/// Maximum number of redirects to follow
pub const MAX_REDIRECTS: usize = 5;

/// Maximum response size in bytes (10 MB)
pub const MAX_RESPONSE_SIZE: u64 = 10 * 1024 * 1024;

/// User agent string for requests
pub const USER_AGENT_VALUE: &str = "feast";

/// Errors that can occur during HTTP operations.
#[derive(Debug, thiserror::Error)]
pub enum FetchError {
    /// The provided URL could not be parsed.
    #[error("Invalid URL: {0}")]
    InvalidUrl(String),

    /// The URL scheme is not HTTP or HTTPS.
    #[error("Invalid URL scheme: only HTTP and HTTPS are supported")]
    InvalidUrlScheme,

    /// Failed to connect to the server.
    #[error("Connection failed: {0}")]
    ConnectionFailed(String),

    /// The request timed out.
    #[error("Request timed out after {0} seconds")]
    Timeout(u64),

    /// Too many redirects were followed.
    #[error("Too many redirects (max {0})")]
    TooManyRedirects(usize),

    /// The server returned an error status code.
    #[error("HTTP error {status}: {message}")]
    HttpError { status: u16, message: String },

    /// The response content type is not HTML.
    #[error("Invalid content type: expected HTML, got {0}")]
    InvalidContentType(String),

    /// The response body exceeds the maximum size.
    #[error("Response too large: exceeds {0} bytes")]
    ResponseTooLarge(u64),

    /// Failed to read the response body.
    #[error("Failed to read response: {0}")]
    ReadError(String),
}

/// Validates that a URL is well-formed and uses HTTP or HTTPS scheme.
pub fn validate_url(url: &str) -> Result<reqwest::Url, FetchError> {
    let parsed = reqwest::Url::parse(url).map_err(|e| FetchError::InvalidUrl(e.to_string()))?;

    match parsed.scheme() {
        "http" | "https" => Ok(parsed),
        _ => Err(FetchError::InvalidUrlScheme),
    }
}

/// Checks if a content type header indicates HTML content.
pub fn is_html_content_type(content_type: &str) -> bool {
    let lower = content_type.to_lowercase();
    lower.contains("text/html") || lower.contains("application/xhtml+xml")
}

/// Maps a reqwest error to a FetchError.
fn map_reqwest_error(err: reqwest::Error) -> FetchError {
    if err.is_timeout() {
        FetchError::Timeout(REQUEST_TIMEOUT.as_secs())
    } else if err.is_redirect() {
        FetchError::TooManyRedirects(MAX_REDIRECTS)
    } else {
        FetchError::ConnectionFailed(err.to_string())
    }
}

/// Returns a user-friendly message for an HTTP status code.
fn status_to_message(status: reqwest::StatusCode) -> String {
    match status.as_u16() {
        400 => "Bad request".to_string(),
        401 => "Authentication required".to_string(),
        403 => "Access forbidden".to_string(),
        404 => "Page not found".to_string(),
        429 => "Too many requests - please try again later".to_string(),
        500 => "Server error".to_string(),
        502 => "Bad gateway".to_string(),
        503 => "Service unavailable".to_string(),
        504 => "Gateway timeout".to_string(),
        _ => status
            .canonical_reason()
            .unwrap_or("Unknown error")
            .to_string(),
    }
}

/// Fetches HTML content from a URL.
///
/// # Arguments
/// * `url` - The URL to fetch
///
/// # Returns
/// The HTML content as a string, or a `FetchError` if the request fails.
///
/// # Errors
/// - `InvalidUrl` if the URL cannot be parsed
/// - `InvalidUrlScheme` if the URL is not HTTP or HTTPS
/// - `ConnectionFailed` if the connection cannot be established
/// - `Timeout` if the request takes longer than 30 seconds
/// - `TooManyRedirects` if more than 5 redirects are followed
/// - `HttpError` if the server returns a non-2xx status code
/// - `InvalidContentType` if the response is not HTML
/// - `ResponseTooLarge` if the response exceeds 10 MB
/// - `ReadError` if the response body cannot be read
pub async fn fetch_url(url: &str) -> Result<String, FetchError> {
    // Validate URL
    let parsed_url = validate_url(url)?;

    // Build client with settings
    let client = reqwest::Client::builder()
        .timeout(REQUEST_TIMEOUT)
        .redirect(reqwest::redirect::Policy::limited(MAX_REDIRECTS))
        .user_agent(USER_AGENT_VALUE)
        .build()
        .map_err(|e| FetchError::ConnectionFailed(e.to_string()))?;

    // Send request
    let response = client
        .get(parsed_url)
        .send()
        .await
        .map_err(map_reqwest_error)?;

    // Check status
    let status = response.status();
    if !status.is_success() {
        return Err(FetchError::HttpError {
            status: status.as_u16(),
            message: status_to_message(status),
        });
    }

    // Check content type
    if let Some(content_type) = response.headers().get(reqwest::header::CONTENT_TYPE) {
        let content_type_str = content_type.to_str().unwrap_or("");
        if !is_html_content_type(content_type_str) {
            return Err(FetchError::InvalidContentType(content_type_str.to_string()));
        }
    }

    // Check content length if available
    if let Some(content_length) = response.content_length() {
        if content_length > MAX_RESPONSE_SIZE {
            return Err(FetchError::ResponseTooLarge(MAX_RESPONSE_SIZE));
        }
    }

    // Read response body
    let body = response
        .text()
        .await
        .map_err(|e| FetchError::ReadError(e.to_string()))?;

    // Check actual size
    if body.len() as u64 > MAX_RESPONSE_SIZE {
        return Err(FetchError::ResponseTooLarge(MAX_RESPONSE_SIZE));
    }

    Ok(body)
}

#[cfg(test)]
mod tests {
    use super::*;

    mod validate_url_tests {
        use super::*;

        #[test]
        fn accepts_https_url() {
            let result = validate_url("https://example.com/recipe");
            assert!(result.is_ok());
        }

        #[test]
        fn accepts_http_url() {
            let result = validate_url("http://example.com/recipe");
            assert!(result.is_ok());
        }

        #[test]
        fn rejects_ftp_url() {
            let result = validate_url("ftp://example.com/file");
            assert!(matches!(result, Err(FetchError::InvalidUrlScheme)));
        }

        #[test]
        fn rejects_file_url() {
            let result = validate_url("file:///etc/passwd");
            assert!(matches!(result, Err(FetchError::InvalidUrlScheme)));
        }

        #[test]
        fn rejects_javascript_url() {
            let result = validate_url("javascript:alert(1)");
            assert!(matches!(result, Err(FetchError::InvalidUrlScheme)));
        }

        #[test]
        fn rejects_invalid_url() {
            let result = validate_url("not a url");
            assert!(matches!(result, Err(FetchError::InvalidUrl(_))));
        }

        #[test]
        fn rejects_empty_url() {
            let result = validate_url("");
            assert!(matches!(result, Err(FetchError::InvalidUrl(_))));
        }
    }

    mod is_html_content_type_tests {
        use super::*;

        #[test]
        fn accepts_text_html() {
            assert!(is_html_content_type("text/html"));
        }

        #[test]
        fn accepts_text_html_with_charset() {
            assert!(is_html_content_type("text/html; charset=utf-8"));
        }

        #[test]
        fn accepts_xhtml() {
            assert!(is_html_content_type("application/xhtml+xml"));
        }

        #[test]
        fn accepts_case_insensitive() {
            assert!(is_html_content_type("TEXT/HTML"));
            assert!(is_html_content_type("Text/Html"));
        }

        #[test]
        fn rejects_json() {
            assert!(!is_html_content_type("application/json"));
        }

        #[test]
        fn rejects_plain_text() {
            assert!(!is_html_content_type("text/plain"));
        }

        #[test]
        fn rejects_image() {
            assert!(!is_html_content_type("image/png"));
        }
    }

    mod integration_tests {
        use super::*;
        use wiremock::matchers::{header, method, path};
        use wiremock::{Mock, MockServer, ResponseTemplate};

        #[tokio::test]
        async fn test_fetch_success() {
            let mock_server = MockServer::start().await;
            let html_content = "<html><body>Test</body></html>";

            Mock::given(method("GET"))
                .and(path("/recipe"))
                .respond_with(ResponseTemplate::new(200).set_body_raw(html_content, "text/html"))
                .mount(&mock_server)
                .await;

            let url = format!("{}/recipe", mock_server.uri());
            let result = fetch_url(&url).await;
            assert!(result.is_ok(), "Expected Ok, got: {:?}", result);
            assert_eq!(result.unwrap(), html_content);
        }

        #[tokio::test]
        async fn test_fetch_user_agent_header() {
            let mock_server = MockServer::start().await;

            Mock::given(method("GET"))
                .and(path("/check-agent"))
                .and(header("user-agent", "feast"))
                .respond_with(ResponseTemplate::new(200).set_body_raw("<html></html>", "text/html"))
                .mount(&mock_server)
                .await;

            let url = format!("{}/check-agent", mock_server.uri());
            let result = fetch_url(&url).await;

            assert!(result.is_ok(), "Expected Ok, got: {:?}", result);
        }

        #[tokio::test]
        async fn test_fetch_404() {
            let mock_server = MockServer::start().await;

            Mock::given(method("GET"))
                .and(path("/not-found"))
                .respond_with(ResponseTemplate::new(404))
                .mount(&mock_server)
                .await;

            let url = format!("{}/not-found", mock_server.uri());
            let result = fetch_url(&url).await;

            match result {
                Err(FetchError::HttpError { status, message }) => {
                    assert_eq!(status, 404);
                    assert!(message.to_lowercase().contains("not found"));
                }
                _ => panic!("Expected HttpError with 404"),
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
                Err(FetchError::HttpError { status, message }) => {
                    assert_eq!(status, 403);
                    assert!(message.to_lowercase().contains("forbidden"));
                }
                _ => panic!("Expected HttpError with 403"),
            }
        }

        #[tokio::test]
        async fn test_fetch_429_rate_limited() {
            let mock_server = MockServer::start().await;

            Mock::given(method("GET"))
                .and(path("/rate-limited"))
                .respond_with(ResponseTemplate::new(429))
                .mount(&mock_server)
                .await;

            let url = format!("{}/rate-limited", mock_server.uri());
            let result = fetch_url(&url).await;

            match result {
                Err(FetchError::HttpError { status, message }) => {
                    assert_eq!(status, 429);
                    assert!(message.to_lowercase().contains("request"));
                }
                _ => panic!("Expected HttpError with 429"),
            }
        }

        #[tokio::test]
        async fn test_fetch_500_server_error() {
            let mock_server = MockServer::start().await;

            Mock::given(method("GET"))
                .and(path("/server-error"))
                .respond_with(ResponseTemplate::new(500))
                .mount(&mock_server)
                .await;

            let url = format!("{}/server-error", mock_server.uri());
            let result = fetch_url(&url).await;

            match result {
                Err(FetchError::HttpError { status, message }) => {
                    assert_eq!(status, 500);
                    assert!(message.to_lowercase().contains("server error"));
                }
                _ => panic!("Expected HttpError with 500"),
            }
        }

        #[tokio::test]
        async fn test_fetch_wrong_content_type_json() {
            let mock_server = MockServer::start().await;

            Mock::given(method("GET"))
                .and(path("/json"))
                .respond_with(ResponseTemplate::new(200).set_body_raw("{}", "application/json"))
                .mount(&mock_server)
                .await;

            let url = format!("{}/json", mock_server.uri());
            let result = fetch_url(&url).await;

            match result {
                Err(FetchError::InvalidContentType(ct)) => {
                    assert!(ct.contains("application/json"));
                }
                _ => panic!("Expected InvalidContentType error, got: {:?}", result),
            }
        }

        #[tokio::test]
        async fn test_fetch_wrong_content_type_image() {
            let mock_server = MockServer::start().await;

            Mock::given(method("GET"))
                .and(path("/image"))
                .respond_with(ResponseTemplate::new(200).set_body_raw(vec![0u8; 10], "image/png"))
                .mount(&mock_server)
                .await;

            let url = format!("{}/image", mock_server.uri());
            let result = fetch_url(&url).await;

            match result {
                Err(FetchError::InvalidContentType(ct)) => {
                    assert!(ct.contains("image/png"));
                }
                _ => panic!("Expected InvalidContentType error"),
            }
        }

        #[tokio::test]
        async fn test_fetch_missing_content_type_allowed() {
            let mock_server = MockServer::start().await;
            let html_content = "<html><body>No content type</body></html>";

            // Use set_body_bytes to avoid setting content-type
            Mock::given(method("GET"))
                .and(path("/no-content-type"))
                .respond_with(
                    ResponseTemplate::new(200).set_body_bytes(html_content.as_bytes().to_vec()),
                )
                .mount(&mock_server)
                .await;

            let url = format!("{}/no-content-type", mock_server.uri());
            let result = fetch_url(&url).await;

            assert!(result.is_ok(), "Expected Ok, got: {:?}", result);
            assert_eq!(result.unwrap(), html_content);
        }

        #[tokio::test]
        async fn test_fetch_connection_refused() {
            // Port 1 should be unreachable
            let result = fetch_url("http://127.0.0.1:1/test").await;

            assert!(matches!(result, Err(FetchError::ConnectionFailed(_))));
        }

        #[tokio::test]
        async fn test_fetch_invalid_url() {
            let result = fetch_url("not a valid url").await;

            assert!(matches!(result, Err(FetchError::InvalidUrl(_))));
        }

        #[tokio::test]
        async fn test_fetch_invalid_scheme() {
            let result = fetch_url("ftp://example.com/file").await;

            assert!(matches!(result, Err(FetchError::InvalidUrlScheme)));
        }

        #[tokio::test]
        async fn test_fetch_html_with_charset() {
            let mock_server = MockServer::start().await;
            let html_content = "<html><body>UTF-8</body></html>";

            Mock::given(method("GET"))
                .and(path("/charset"))
                .respond_with(
                    ResponseTemplate::new(200)
                        .set_body_raw(html_content, "text/html; charset=utf-8"),
                )
                .mount(&mock_server)
                .await;

            let url = format!("{}/charset", mock_server.uri());
            let result = fetch_url(&url).await;

            assert!(result.is_ok(), "Expected Ok, got: {:?}", result);
            assert_eq!(result.unwrap(), html_content);
        }

        #[tokio::test]
        async fn test_fetch_xhtml_content_type() {
            let mock_server = MockServer::start().await;
            let xhtml_content = "<?xml version=\"1.0\"?><html></html>";

            Mock::given(method("GET"))
                .and(path("/xhtml"))
                .respond_with(
                    ResponseTemplate::new(200).set_body_raw(xhtml_content, "application/xhtml+xml"),
                )
                .mount(&mock_server)
                .await;

            let url = format!("{}/xhtml", mock_server.uri());
            let result = fetch_url(&url).await;

            assert!(result.is_ok(), "Expected Ok, got: {:?}", result);
            assert_eq!(result.unwrap(), xhtml_content);
        }

        #[tokio::test]
        async fn test_fetch_401_unauthorized() {
            let mock_server = MockServer::start().await;

            Mock::given(method("GET"))
                .and(path("/auth-required"))
                .respond_with(ResponseTemplate::new(401))
                .mount(&mock_server)
                .await;

            let url = format!("{}/auth-required", mock_server.uri());
            let result = fetch_url(&url).await;

            match result {
                Err(FetchError::HttpError { status, message }) => {
                    assert_eq!(status, 401);
                    assert!(message.to_lowercase().contains("authentication"));
                }
                _ => panic!("Expected HttpError with 401"),
            }
        }

        #[tokio::test]
        async fn test_fetch_502_bad_gateway() {
            let mock_server = MockServer::start().await;

            Mock::given(method("GET"))
                .and(path("/bad-gateway"))
                .respond_with(ResponseTemplate::new(502))
                .mount(&mock_server)
                .await;

            let url = format!("{}/bad-gateway", mock_server.uri());
            let result = fetch_url(&url).await;

            match result {
                Err(FetchError::HttpError { status, message }) => {
                    assert_eq!(status, 502);
                    assert!(message.to_lowercase().contains("gateway"));
                }
                _ => panic!("Expected HttpError with 502"),
            }
        }

        #[tokio::test]
        async fn test_fetch_503_service_unavailable() {
            let mock_server = MockServer::start().await;

            Mock::given(method("GET"))
                .and(path("/unavailable"))
                .respond_with(ResponseTemplate::new(503))
                .mount(&mock_server)
                .await;

            let url = format!("{}/unavailable", mock_server.uri());
            let result = fetch_url(&url).await;

            match result {
                Err(FetchError::HttpError { status, message }) => {
                    assert_eq!(status, 503);
                    assert!(message.to_lowercase().contains("unavailable"));
                }
                _ => panic!("Expected HttpError with 503"),
            }
        }
    }
}
