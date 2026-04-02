//! Error types for the Shopify API client

use thiserror::Error;

/// Result type alias for Shopify API operations
pub type Result<T> = std::result::Result<T, ShopifyError>;

/// Errors that can occur when interacting with the Shopify API
#[derive(Error, Debug)]
pub enum ShopifyError {
    /// HTTP request failed
    #[error("HTTP request failed: {0}")]
    Request(#[from] reqwest::Error),

    /// JSON serialization/deserialization failed
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    /// Resource not found (404)
    #[error("Resource not found: {resource} with id {id}")]
    NotFound { resource: String, id: String },

    /// Authentication failed (401)
    #[error("Authentication failed: {0}")]
    Unauthorized(String),

    /// Permission denied (403)
    #[error("Permission denied: {0}")]
    Forbidden(String),

    /// Rate limit exceeded (429)
    #[error("Rate limit exceeded. Retry after {retry_after} seconds")]
    RateLimited { retry_after: u64 },

    /// Validation error (422)
    #[error("Validation error: {0}")]
    ValidationError(String),

    /// Server error (5xx)
    #[error("Server error: {0}")]
    ServerError(String),

    /// URL parsing error
    #[error("URL parse error: {0}")]
    UrlParse(#[from] url::ParseError),

    /// Generic API error
    #[error("API error ({status}): {message}")]
    ApiError { status: u16, message: String },
}

impl ShopifyError {
    /// Create an error from an HTTP response status and body
    pub fn from_response(status: u16, body: &str) -> Self {
        match status {
            401 => ShopifyError::Unauthorized(body.to_string()),
            403 => ShopifyError::Forbidden(body.to_string()),
            404 => ShopifyError::ApiError {
                status,
                message: body.to_string(),
            },
            422 => ShopifyError::ValidationError(body.to_string()),
            429 => ShopifyError::RateLimited { retry_after: 2 },
            500..=599 => ShopifyError::ServerError(body.to_string()),
            _ => ShopifyError::ApiError {
                status,
                message: body.to_string(),
            },
        }
    }
}
