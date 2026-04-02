//! Error types for the Shopify GraphQL API client
use thiserror::Error;

/// Result type alias for Shopify API operations
pub type Result<T> = std::result::Result<T, ShopifyError>;

/// Represents an error returned inside a GraphQL JSON payload under the "errors" array
#[derive(Debug, Clone, serde::Deserialize)]
pub struct GraphQLErrorDetail {
    pub message: String,
    // Add path, locations, extensions etc. as needed
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

    /// Graphql Payload execution failure
    #[error("GraphQL error(s) occurred")]
    GraphQLError(Vec<GraphQLErrorDetail>),

    /// General validation
    #[error("Validation error: {0}")]
    ValidationError(String),

    /// URL parse failure
    #[error("URL parsing error: {0}")]
    UrlParse(#[from] url::ParseError),

    /// API Error based on HTTP response statuses
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
