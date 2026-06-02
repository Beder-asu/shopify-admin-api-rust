//! Error types for the Shopify GraphQL API client

use thiserror::Error;

/// Result type alias for Shopify GraphQL API operations
pub type Result<T> = std::result::Result<T, ShopifyError>;

/// Source location of a GraphQL error (line and column in the query).
#[derive(Debug, Clone, serde::Deserialize)]
pub struct ErrorLocation {
    /// Line number in the GraphQL query document.
    pub line: u32,
    /// Column number in the GraphQL query document.
    pub column: u32,
}

/// A single error entry in the GraphQL response `errors` array.
///
/// Matches the standard GraphQL error format including optional diagnostic fields.
#[derive(Debug, Clone, serde::Deserialize)]
pub struct GraphQLErrorDetail {
    /// Human-readable error message.
    pub message: String,
    /// Source location(s) in the query that triggered this error.
    #[serde(default)]
    pub locations: Option<Vec<ErrorLocation>>,
    /// The response field path where the error occurred.
    #[serde(default)]
    pub path: Option<Vec<serde_json::Value>>,
    /// Optional extension data (error codes, cost info, etc.).
    #[serde(default)]
    pub extensions: Option<serde_json::Value>,
}

/// Errors that can occur when interacting with the Shopify GraphQL API
#[derive(Error, Debug)]
pub enum ShopifyError {
    /// HTTP request failed
    #[error("HTTP request failed: {0}")]
    Request(#[from] reqwest::Error),

    /// JSON serialization/deserialization failed
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    /// GraphQL payload execution errors (may accompany partial data)
    #[error("GraphQL error(s): {}", .0.iter().map(|e| e.message.as_str()).collect::<Vec<_>>().join("; "))]
    GraphQLError(Vec<GraphQLErrorDetail>),

    /// Response contained no data and no errors (unexpected empty envelope)
    #[error("GraphQL response contained no data")]
    NoData,

    /// General validation error
    #[error("Validation error: {0}")]
    ValidationError(String),

    /// URL parse failure
    #[error("URL parsing error: {0}")]
    UrlParse(#[from] url::ParseError),

    /// Rate limit exceeded after all retries
    #[error("Rate limited. Retry after {retry_after} seconds")]
    RateLimited { retry_after: u64 },

    /// HTTP-level API error
    #[error("API HTTP error ({status}): {message}")]
    ApiError { status: u16, message: String },
}

impl ShopifyError {
    /// Create an error from an HTTP response status and body
    pub fn from_response(status: u16, body: &str) -> Self {
        ShopifyError::ApiError {
            status,
            message: body.to_string(),
        }
    }
}
