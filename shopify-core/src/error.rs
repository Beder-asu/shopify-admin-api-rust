//! Error types for the Shopify API (REST and GraphQL)

use thiserror::Error;

/// Result type alias for Shopify API operations
/// 
/// RUST TIP: `Result<T, E>` is Rust's way of handling errors instead of exceptions (`try/catch`).
/// It is an enum that is either `Ok(T)` (success) or `Err(E)` (failure).
/// Here we create an alias `Result<T>` that hardcodes `ShopifyError` as the error type `E`.
pub type Result<T> = std::result::Result<T, ShopifyError>;

/// Source location of a GraphQL error (line and column in the query).
#[derive(Debug, Clone, serde::Deserialize)]
pub struct ErrorLocation {
    pub line: u32,
    pub column: u32,
}

/// A single error entry in the GraphQL response `errors` array.
#[derive(Debug, Clone, serde::Deserialize)]
pub struct GraphQLErrorDetail {
    pub message: String,
    #[serde(default)]
    pub locations: Option<Vec<ErrorLocation>>,
    #[serde(default)]
    pub path: Option<Vec<serde_json::Value>>,
    #[serde(default)]
    pub extensions: Option<serde_json::Value>,
}

/// Errors that can occur when interacting with the Shopify API.
/// 
/// RUST TIP: `enum` in Rust is much more powerful than in TypeScript. An enum variant can 
/// hold data (like `Unauthorized(String)` or `RateLimited { retry_after: u64 }`).
/// The `#[derive(Error)]` macro comes from the `thiserror` crate and automatically 
/// implements the standard `std::error::Error` trait for this enum.
#[derive(Error, Debug)]
pub enum ShopifyError {
    // ── Network / HTTP ─────────────────────────────────────────────────────────

    #[error("HTTP request failed: {0}")]
    Request(#[from] reqwest::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("URL parse error: {0}")]
    UrlParse(#[from] url::ParseError),

    // ── GraphQL Specific ───────────────────────────────────────────────────────

    #[error("GraphQL error(s): {}", .0.iter().map(|e| e.message.as_str()).collect::<Vec<_>>().join("; "))]
    GraphQLError(Vec<GraphQLErrorDetail>),

    #[error("GraphQL response contained no data")]
    NoData,

    // ── HTTP Status Errors ──────────────────────────────────────────────────────

    #[error("Authentication failed: {0}")]
    Unauthorized(String),

    #[error("Permission denied: {0}")]
    Forbidden(String),

    #[error("Resource not found: {resource} {id}")]
    NotFound { resource: String, id: String },

    #[error("Validation error: {0}")]
    ValidationError(String),

    #[error("Rate limit exceeded. Retry after {retry_after} seconds")]
    RateLimited { retry_after: u64 },

    #[error("Server error: {0}")]
    ServerError(String),

    #[error("API HTTP error ({status}): {message}")]
    ApiError { status: u16, message: String },

    // ── Auth / OAuth ────────────────────────────────────────────────────────────

    #[error("Invalid OAuth callback: {0}")]
    InvalidOAuth(String),

    #[error("Private app error: {0}")]
    PrivateAppError(String),

    #[error("OAuth cookie not found for shop: {0}")]
    CookieNotFound(String),

    // ── Webhook ─────────────────────────────────────────────────────────────────

    #[error("Webhook HMAC validation failed")]
    InvalidWebhookHmac,

    // ── Session ─────────────────────────────────────────────────────────────────

    #[error("Invalid session: {0}")]
    InvalidSession(String),

    // ── Configuration ────────────────────────────────────────────────────────────

    #[error("Invalid configuration — missing fields: {0}")]
    InvalidConfig(String),
}

impl ShopifyError {
    /// Map an HTTP response status code and body to the appropriate error variant.
    pub fn from_response(status: u16, body: &str) -> Self {
        match status {
            401 => ShopifyError::Unauthorized(body.to_string()),
            403 => ShopifyError::Forbidden(body.to_string()),
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
