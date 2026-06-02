//! Core data structures for GraphQL interactions

use serde::{Deserialize, Serialize};
use crate::error::GraphQLErrorDetail;

/// The request body structure for Shopify GraphQL calls
#[derive(Debug, Serialize)]
pub struct GraphQLRequest<'a, V> {
    pub query: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variables: Option<&'a V>,
}

/// Throttle Status providing rate limiting feedback
#[derive(Debug, Deserialize, Clone)]
pub struct ThrottleStatus {
    #[serde(rename = "maximumAvailable")]
    pub maximum_available: f64,
    #[serde(rename = "currentlyAvailable")]
    pub currently_available: f64,
    #[serde(rename = "restoreRate")]
    pub restore_rate: f64,
}

/// Query Cost provided in the response extensions
#[derive(Debug, Deserialize, Clone)]
pub struct Cost {
    #[serde(rename = "requestedQueryCost")]
    pub requested_query_cost: f64,
    #[serde(rename = "actualQueryCost")]
    pub actual_query_cost: Option<f64>,
    #[serde(rename = "throttleStatus")]
    pub throttle_status: ThrottleStatus,
}

/// Shopify Extensions field
#[derive(Debug, Deserialize, Clone)]
pub struct Extensions {
    pub cost: Option<Cost>,
}

/// The response envelope for all GraphQL queries.
///
/// `errors` and `extensions` use `#[serde(default)]` — both are `Option<_>`,
/// so `Default = None` works without any bound on `T`.
///
/// `data` is also `Option<T>`. We **do not** add `#[serde(default)]` here
/// because `Option<T>` already deserializes a missing key as `None` without
/// requiring `T: Default`.
#[derive(Debug, Deserialize)]
pub struct GraphQLResponse<T> {
    /// The actual data payload mapped to the caller's type.
    pub data: Option<T>,

    /// Field-level errors from the GraphQL execution engine.
    /// `None` when the response is fully successful.
    #[serde(default)]
    pub errors: Option<Vec<GraphQLErrorDetail>>,

    /// API cost and throttle metadata.
    #[serde(default)]
    pub extensions: Option<Extensions>,
}

/// Common connection fields (Relay-style pagination)
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PageInfo {
    #[serde(rename = "hasNextPage")]
    pub has_next_page: bool,
    #[serde(rename = "hasPreviousPage")]
    pub has_previous_page: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

/// A single edge in a Relay-style Connection
#[derive(Debug, Clone, Deserialize)]
pub struct Edge<T> {
    pub cursor: String,
    pub node: T,
}

/// A Relay-style Connection wrapping a list of typed nodes with pagination
#[derive(Debug, Clone, Deserialize)]
pub struct Connection<T> {
    pub edges: Vec<Edge<T>>,
    #[serde(rename = "pageInfo")]
    pub page_info: PageInfo,
}
