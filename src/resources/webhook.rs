//! Webhook resource
//! 
//! Represents a webhook subscription in the Shopify store.

use crate::{
    base::{CountResponse, FindAllResponse},
    client::Client,
    error::Result,
};
use serde::{Deserialize, Serialize};

/// Webhook resource
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Webhook {
    /// Unique identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// The destination URI to which the webhook will send POST requests
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// The event topic that triggers the webhook
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic: Option<String>,
    /// The API version used to serialize the webhook payload
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_version: Option<String>,
    /// When the webhook was created
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// Specific fields to include in the webhook payload
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<Vec<String>>,
    /// The format of the webhook payload (json or xml)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    /// Metafield namespaces to include in the webhook payload
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metafield_namespaces: Option<Vec<String>>,
    /// Private metafield namespaces to include in the webhook payload
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_metafield_namespaces: Option<Vec<String>>,
    /// When the webhook was last updated
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

/// Parameters for listing webhooks
#[derive(Debug, Clone, Default, Serialize)]
pub struct WebhookListParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at_min: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at_max: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at_min: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at_max: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub since_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<String>,
}

/// Parameters for counting webhooks
#[derive(Debug, Clone, Default, Serialize)]
pub struct WebhookCountParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic: Option<String>,
}

/// Common webhook topics
pub mod topics {
    pub const ORDERS_CREATE: &str = "orders/create";
    pub const ORDERS_UPDATED: &str = "orders/updated";
    pub const ORDERS_PAID: &str = "orders/paid";
    pub const ORDERS_CANCELLED: &str = "orders/cancelled";
    pub const ORDERS_FULFILLED: &str = "orders/fulfilled";
    pub const ORDERS_PARTIALLY_FULFILLED: &str = "orders/partially_fulfilled";
    pub const ORDERS_DELETE: &str = "orders/delete";
    pub const ORDER_TRANSACTIONS_CREATE: &str = "order_transactions/create";
    pub const PRODUCTS_CREATE: &str = "products/create";
    pub const PRODUCTS_UPDATE: &str = "products/update";
    pub const PRODUCTS_DELETE: &str = "products/delete";
    pub const CUSTOMERS_CREATE: &str = "customers/create";
    pub const CUSTOMERS_UPDATE: &str = "customers/update";
    pub const CUSTOMERS_DELETE: &str = "customers/delete";
    pub const CUSTOMERS_ENABLE: &str = "customers/enable";
    pub const CUSTOMERS_DISABLE: &str = "customers/disable";
    pub const CARTS_CREATE: &str = "carts/create";
    pub const CARTS_UPDATE: &str = "carts/update";
    pub const CHECKOUTS_CREATE: &str = "checkouts/create";
    pub const CHECKOUTS_UPDATE: &str = "checkouts/update";
    pub const CHECKOUTS_DELETE: &str = "checkouts/delete";
    pub const COLLECTIONS_CREATE: &str = "collections/create";
    pub const COLLECTIONS_UPDATE: &str = "collections/update";
    pub const COLLECTIONS_DELETE: &str = "collections/delete";
    pub const FULFILLMENTS_CREATE: &str = "fulfillments/create";
    pub const FULFILLMENTS_UPDATE: &str = "fulfillments/update";
    pub const INVENTORY_ITEMS_CREATE: &str = "inventory_items/create";
    pub const INVENTORY_ITEMS_UPDATE: &str = "inventory_items/update";
    pub const INVENTORY_ITEMS_DELETE: &str = "inventory_items/delete";
    pub const INVENTORY_LEVELS_CONNECT: &str = "inventory_levels/connect";
    pub const INVENTORY_LEVELS_UPDATE: &str = "inventory_levels/update";
    pub const INVENTORY_LEVELS_DISCONNECT: &str = "inventory_levels/disconnect";
    pub const REFUNDS_CREATE: &str = "refunds/create";
    pub const APP_UNINSTALLED: &str = "app/uninstalled";
    pub const SHOP_UPDATE: &str = "shop/update";
    pub const THEMES_CREATE: &str = "themes/create";
    pub const THEMES_PUBLISH: &str = "themes/publish";
    pub const THEMES_UPDATE: &str = "themes/update";
    pub const THEMES_DELETE: &str = "themes/delete";
}

/// Wrapper for API responses
#[derive(Debug, Deserialize)]
struct WebhookWrapper {
    webhook: Webhook,
}

#[derive(Debug, Deserialize)]
struct WebhooksWrapper {
    webhooks: Vec<Webhook>,
}

#[derive(Debug, Serialize)]
struct WebhookRequest {
    webhook: Webhook,
}

impl Webhook {
    /// Find a webhook by ID
    pub async fn find(client: &Client, id: i64) -> Result<Option<Self>> {
        let path = format!("webhooks/{}.json", id);
        let response = client.get::<WebhookWrapper>(&path).await?;
        Ok(Some(response.data.webhook))
    }

    /// Find a webhook by ID with specific fields
    pub async fn find_with_fields(client: &Client, id: i64, fields: &str) -> Result<Option<Self>> {
        let path = format!("webhooks/{}.json?fields={}", id, fields);
        let response = client.get::<WebhookWrapper>(&path).await?;
        Ok(Some(response.data.webhook))
    }

    /// List all webhooks
    pub async fn all(client: &Client, params: WebhookListParams) -> Result<FindAllResponse<Self>> {
        let response = client
            .get_with_params::<WebhooksWrapper, _>("webhooks.json", &params)
            .await?;
        
        Ok(FindAllResponse {
            data: response.data.webhooks,
            next_page: response.page_info.as_ref().and_then(|p| p.next.clone()),
            previous_page: response.page_info.as_ref().and_then(|p| p.previous.clone()),
        })
    }

    /// Count webhooks
    pub async fn count(client: &Client, params: WebhookCountParams) -> Result<i64> {
        let response = client
            .get_with_params::<CountResponse, _>("webhooks/count.json", &params)
            .await?;
        Ok(response.data.count)
    }

    /// Create a new webhook
    pub async fn create(client: &Client, webhook: &Self) -> Result<Self> {
        let request = WebhookRequest {
            webhook: webhook.clone(),
        };
        let response = client
            .post::<WebhookWrapper, _>("webhooks.json", &request)
            .await?;
        Ok(response.data.webhook)
    }

    /// Update an existing webhook
    pub async fn update(client: &Client, webhook: &Self) -> Result<Self> {
        let id = webhook.id.ok_or_else(|| {
            crate::ShopifyError::ValidationError("Webhook ID is required for update".to_string())
        })?;
        let path = format!("webhooks/{}.json", id);
        let request = WebhookRequest {
            webhook: webhook.clone(),
        };
        let response = client.put::<WebhookWrapper, _>(&path, &request).await?;
        Ok(response.data.webhook)
    }

    /// Save the webhook (create or update)
    pub async fn save(client: &Client, webhook: &Self) -> Result<Self> {
        if webhook.id.is_some() {
            Self::update(client, webhook).await
        } else {
            Self::create(client, webhook).await
        }
    }

    /// Delete a webhook
    pub async fn delete(client: &Client, id: i64) -> Result<()> {
        let path = format!("webhooks/{}.json", id);
        client.delete(&path).await
    }

    /// Create a webhook with a builder pattern
    pub fn builder() -> WebhookBuilder {
        WebhookBuilder::new()
    }
}

/// Builder for creating webhooks
#[derive(Debug, Default)]
pub struct WebhookBuilder {
    webhook: Webhook,
}

impl WebhookBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn topic(mut self, topic: impl Into<String>) -> Self {
        self.webhook.topic = Some(topic.into());
        self
    }

    pub fn address(mut self, address: impl Into<String>) -> Self {
        self.webhook.address = Some(address.into());
        self
    }

    pub fn format(mut self, format: impl Into<String>) -> Self {
        self.webhook.format = Some(format.into());
        self
    }

    pub fn fields(mut self, fields: Vec<String>) -> Self {
        self.webhook.fields = Some(fields);
        self
    }

    pub fn metafield_namespaces(mut self, namespaces: Vec<String>) -> Self {
        self.webhook.metafield_namespaces = Some(namespaces);
        self
    }

    pub fn build(self) -> Webhook {
        self.webhook
    }
}
