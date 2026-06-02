//! Error types for the Shopify Admin REST API client

use thiserror::Error;

/// Result type alias for Shopify API operations
pub type Result<T> = std::result::Result<T, ShopifyError>;

/// Errors that can occur when interacting with the Shopify Admin API.
#[derive(Error, Debug)]
pub enum ShopifyError {
    // ── Network / HTTP ─────────────────────────────────────────────────────────

    /// HTTP request failed at the transport level
    #[error("HTTP request failed: {0}")]
    Request(#[from] reqwest::Error),

    /// JSON serialization/deserialization failed
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    /// URL parse error
    #[error("URL parse error: {0}")]
    UrlParse(#[from] url::ParseError),

    // ── HTTP Status Errors ──────────────────────────────────────────────────────

    /// Authentication failed (401)
    #[error("Authentication failed: {0}")]
    Unauthorized(String),

    /// Permission denied (403)
    #[error("Permission denied: {0}")]
    Forbidden(String),

    /// Resource not found (404)
    #[error("Resource not found: {resource} {id}")]
    NotFound { resource: String, id: String },

    /// Validation error (422)
    #[error("Validation error: {0}")]
    ValidationError(String),

    /// Rate limit exceeded (429) — returned after all retries are exhausted
    #[error("Rate limit exceeded. Retry after {retry_after} seconds")]
    RateLimited { retry_after: u64 },

    /// Server error (5xx)
    #[error("Server error: {0}")]
    ServerError(String),

    /// Generic HTTP API error for other status codes
    #[error("API error ({status}): {message}")]
    ApiError { status: u16, message: String },

    // ── Auth / OAuth ────────────────────────────────────────────────────────────

    /// OAuth callback HMAC validation failed or state mismatch
    #[error("Invalid OAuth callback: {0}")]
    InvalidOAuth(String),

    /// OAuth not available for custom store apps
    #[error("Private app error: {0}")]
    PrivateAppError(String),

    /// OAuth state cookie was not found during callback
    #[error("OAuth cookie not found for shop: {0}")]
    CookieNotFound(String),

    // ── Webhook ─────────────────────────────────────────────────────────────────

    /// Incoming webhook HMAC validation failed
    #[error("Webhook HMAC validation failed")]
    InvalidWebhookHmac,

    // ── Session ─────────────────────────────────────────────────────────────────

    /// Session data was invalid or could not be reconstructed
    #[error("Invalid session: {0}")]
    InvalidSession(String),

    // ── Configuration ────────────────────────────────────────────────────────────

    /// SDK configuration is missing required fields
    #[error("Invalid configuration — missing fields: {0}")]
    InvalidConfig(String),
}

impl ShopifyError {
    /// Map an HTTP response status code and body to the appropriate error variant.
    ///
    /// Note: `RateLimited` is **not** returned here — the client's retry loop
    /// returns it directly after exhausting retries, so it can include the actual
    /// `Retry-After` header value.
    pub fn from_response(status: u16, body: &str) -> Self {
        match status {
            401 => ShopifyError::Unauthorized(body.to_string()),
            403 => ShopifyError::Forbidden(body.to_string()),
            // Problem 11 fix: 404 now maps to the dedicated NotFound variant
            404 => ShopifyError::NotFound {
                resource: String::new(),
                id: body.to_string(),
            },
            422 => ShopifyError::ValidationError(body.to_string()),
            500..=599 => ShopifyError::ServerError(body.to_string()),
            _ => ShopifyError::ApiError {
                status,
                message: body.to_string(),
            },
        }
    }
}
