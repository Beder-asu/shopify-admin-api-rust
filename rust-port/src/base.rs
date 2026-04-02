//! Base traits and types for REST resources

use crate::{client::Client, error::Result};
use async_trait::async_trait;
use serde::{de::DeserializeOwned, Serialize};

/// Common query parameters for listing resources
#[derive(Debug, Clone, Default, Serialize)]
pub struct ListParams {
    /// Maximum number of results to return
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    /// Restrict results to after the specified ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub since_id: Option<i64>,
    /// Show only certain fields
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<String>,
    /// Filter by created_at minimum
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at_min: Option<String>,
    /// Filter by created_at maximum
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at_max: Option<String>,
    /// Filter by updated_at minimum
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at_min: Option<String>,
    /// Filter by updated_at maximum
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at_max: Option<String>,
}

/// Common count parameters
#[derive(Debug, Clone, Default, Serialize)]
pub struct CountParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at_min: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at_max: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at_min: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at_max: Option<String>,
}

/// Response containing a count
#[derive(Debug, Clone, Serialize, serde::Deserialize)]
pub struct CountResponse {
    pub count: i64,
}

/// Paginated response containing multiple resources
#[derive(Debug, Clone)]
pub struct FindAllResponse<T> {
    /// The list of resources
    pub data: Vec<T>,
    /// URL for the next page of results
    pub next_page: Option<String>,
    /// URL for the previous page of results
    pub previous_page: Option<String>,
}

/// Trait for resources that can be found by ID
#[async_trait]
pub trait Findable: Sized + DeserializeOwned {
    /// The resource name in the API response (e.g., "customer", "order")
    const RESOURCE_NAME: &'static str;
    /// The plural resource name (e.g., "customers", "orders")
    const RESOURCE_NAME_PLURAL: &'static str;

    /// Find a single resource by ID
    async fn find(client: &Client, id: i64) -> Result<Option<Self>>;
    
    /// Find a single resource by ID with specific fields
    async fn find_with_fields(client: &Client, id: i64, fields: &str) -> Result<Option<Self>>;
}

/// Trait for resources that can be listed
#[async_trait]
pub trait Listable: Sized + DeserializeOwned {
    /// Parameters type for listing
    type ListParams: Serialize + Send + Sync;

    /// List all resources with given parameters
    async fn all(client: &Client, params: Self::ListParams) -> Result<FindAllResponse<Self>>;
}

/// Trait for resources that can be counted
#[async_trait]
pub trait Countable {
    /// Parameters type for counting
    type CountParams: Serialize + Send + Sync;

    /// Count resources matching the given parameters
    async fn count(client: &Client, params: Self::CountParams) -> Result<i64>;
}

/// Trait for resources that can be created
#[async_trait]
pub trait Creatable: Sized + Serialize + DeserializeOwned + Send + Sync {
    /// Create a new resource
    async fn create(client: &Client, resource: &Self) -> Result<Self>;
}

/// Trait for resources that can be updated
#[async_trait]
pub trait Updatable: Sized + Serialize + DeserializeOwned + Send + Sync {
    /// Get the resource ID
    fn id(&self) -> Option<i64>;
    
    /// Update an existing resource
    async fn update(client: &Client, resource: &Self) -> Result<Self>;
}

/// Trait for resources that can be deleted
#[async_trait]
pub trait Deletable {
    /// Delete a resource by ID
    async fn delete(client: &Client, id: i64) -> Result<()>;
}

/// Trait for resources that can be saved (created or updated)
#[async_trait]
pub trait Saveable: Sized + Serialize + DeserializeOwned + Send + Sync {
    /// Get the resource ID (None if not yet created)
    fn id(&self) -> Option<i64>;

    /// Save the resource (create if new, update if existing)
    async fn save(client: &Client, resource: &Self) -> Result<Self>;
}
