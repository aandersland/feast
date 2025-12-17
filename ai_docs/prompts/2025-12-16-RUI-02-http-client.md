---
date: 2025-12-16
status: draft
parent_roadmap: ai_docs/roadmaps/2025-12-16-recipe-url-import.md
chunk: 2
chunk_name: HTTP Client for Recipe Fetching
target_command: create_plan
---

# HTTP Client for Recipe Fetching

## Inherited Context

> **Feature**: Recipe URL Import
> **Roadmap**: ai_docs/roadmaps/2025-12-16-recipe-url-import.md
> **Chunk**: 2 of 4
> **Depends on**: None
> **Produces**: HTTP fetching capability used by Chunk 4 (Import Command) to retrieve recipe pages

## Goal

Add HTTP client capability to fetch HTML content from recipe URLs with proper error handling, validation, and a minimal User-Agent header.

## Background

The recipe import feature needs to fetch HTML from user-provided URLs. The HTTP client must handle real-world conditions: redirects, timeouts, and various error states. The User-Agent must be exactly "feast" per user requirements.

**Constraint**: User-Agent header must be exactly "feast" — no version, no additional info.

## Requirements

### Must Have

- Add `reqwest` crate (or equivalent) with async support
- Validate URL format before fetching:
  - Must be http:// or https:// scheme
  - Must have valid host
  - Reject file://, ftp://, or other schemes
- Set User-Agent header to exactly "feast"
- Fixed 30-second timeout
- Follow up to 5 redirects maximum
- Verify response Content-Type is HTML (`text/html`) before returning
- Return clear error types for different failure modes
- Unit tests with mocked HTTP responses

### Out of Scope (handled by other chunks)

- HTML parsing (Chunk 1)
- Database storage (Chunk 3)
- Tauri command integration (Chunk 4)
- Caching responses
- Cookie handling
- Authentication

## Affected Areas

- **Systems/Components**:
  - New `src-tauri/src/http/` module (or `src-tauri/src/http_client.rs`)
  - `Cargo.toml` — add `reqwest` dependency
- **Data**: Returns HTML string on success, error type on failure

## Edge Cases

### URL Validation
- Valid: `https://www.example.com/recipe/123`
- Valid: `http://example.com/recipe` (will likely redirect to https)
- Invalid: `ftp://example.com/file` → `InvalidUrlScheme`
- Invalid: `not-a-url` → `InvalidUrl`
- Invalid: `javascript:alert(1)` → `InvalidUrlScheme`
- Invalid: Empty string → `InvalidUrl`

### Network Errors
- DNS resolution failure → `ConnectionFailed("could not resolve host")`
- Connection refused → `ConnectionFailed("connection refused")`
- Connection reset → `ConnectionFailed("connection reset")`
- SSL/TLS errors → `ConnectionFailed("SSL error: ...")`

### Timeout
- Request takes >30 seconds → `Timeout`

### Redirects
- 1-5 redirects → follow transparently
- 6+ redirects → `TooManyRedirects`
- Redirect to non-HTTP scheme → `InvalidUrlScheme`

### HTTP Status Codes
- 200 OK → success (if Content-Type is HTML)
- 301, 302, 307, 308 → follow redirect (counted toward limit)
- 403 Forbidden → `HttpError(403, "access denied")`
- 404 Not Found → `HttpError(404, "page not found")`
- 429 Too Many Requests → `HttpError(429, "rate limited")`
- 500-599 → `HttpError(code, "server error")`

### Content-Type Validation
- `text/html` → valid
- `text/html; charset=utf-8` → valid (ignore charset parameter)
- `application/xhtml+xml` → valid (XHTML)
- `application/json` → `InvalidContentType("expected HTML, got application/json")`
- `image/jpeg` → `InvalidContentType("expected HTML, got image/jpeg")`
- Missing Content-Type header → attempt to proceed (some servers omit it)

### Response Size
- Consider adding a reasonable max response size (e.g., 10MB) to prevent memory issues with malformed responses

## Success Criteria

### Automated Verification
- [ ] `cargo test` passes for HTTP client module
- [ ] `cargo clippy` passes with no warnings
- [ ] Tests cover: successful fetch, timeout, redirect limits, invalid URLs, HTTP errors, content-type validation
- [ ] Tests use mocked HTTP (no real network calls in tests)

### Manual Verification
- [ ] Fetches HTML from 3 different real recipe sites
- [ ] Correctly rejects non-HTML URLs (e.g., direct image links)
- [ ] Error messages are user-friendly

## Open Questions for Planning

- Should `reqwest` be added with blocking or async features? (Check Tauri async patterns in existing code)
- Best approach for mocking HTTP in tests — `mockito`, `wiremock`, or manual mocking?
- Should this module live alongside the parser or in a separate `http` module?

---

**To execute**: `/create_plan ai_docs/prompts/2025-12-16-02-http-client.md`
