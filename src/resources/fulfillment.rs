//! Fulfillment resource
//! 
//! Represents a fulfillment in the Shopify store.

use crate::{
    base::{CountResponse, FindAllResponse},
    client::Client,
    error::Result,
};
use serde::{Deserialize, Serialize};

/// Fulfillment resource
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Fulfillment {
    /// Unique identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// When fulfillment was created
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// Location ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_id: Option<i64>,
    /// Fulfillment name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Order ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<i64>,
    /// Origin address
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin_address: Option<Vec<serde_json::Value>>,
    /// Receipt
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receipt: Option<serde_json::Value>,
    /// Service name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service: Option<String>,
    /// Shipment status
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipment_status: Option<String>,
    /// Fulfillment status
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Tracking company
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracking_company: Option<String>,
    /// Single tracking number
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracking_number: Option<String>,
    /// Multiple tracking numbers
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracking_numbers: Option<Vec<String>>,
    /// Single tracking URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracking_url: Option<String>,
    /// Multiple tracking URLs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracking_urls: Option<Vec<String>>,
    /// When fulfillment was last updated
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    /// Variant inventory management
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variant_inventory_management: Option<String>,
    /// Line items in this fulfillment
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_items: Option<Vec<FulfillmentLineItem>>,
    /// Whether to notify customer
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notify_customer: Option<bool>,
    /// Tracking info for update operations
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracking_info: Option<TrackingInfo>,
    /// Line items by fulfillment order
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_items_by_fulfillment_order: Option<Vec<LineItemsByFulfillmentOrder>>,
}

/// Line item in a fulfillment
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FulfillmentLineItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variant_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sku: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variant_title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vendor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fulfillment_service: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requires_shipping: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub taxable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gift_card: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grams: Option<i64>,
}

/// Tracking information
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TrackingInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company: Option<String>,
}

/// Line items by fulfillment order
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LineItemsByFulfillmentOrder {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fulfillment_order_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fulfillment_order_line_items: Option<Vec<FulfillmentOrderLineItem>>,
}

/// Fulfillment order line item
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FulfillmentOrderLineItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<i32>,
}

/// Parameters for listing fulfillments
#[derive(Debug, Clone, Default, Serialize)]
pub struct FulfillmentListParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at_min: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at_max: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at_min: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at_max: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub since_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<String>,
}

/// Parameters for counting fulfillments
#[derive(Debug, Clone, Default, Serialize)]
pub struct FulfillmentCountParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at_min: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at_max: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at_min: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at_max: Option<String>,
}

/// Parameters for updating tracking
#[derive(Debug, Clone, Default, Serialize)]
pub struct UpdateTrackingParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notify_customer: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracking_info: Option<TrackingInfo>,
}

/// Wrapper for API responses
#[derive(Debug, Deserialize)]
struct FulfillmentWrapper {
    fulfillment: Fulfillment,
}

#[derive(Debug, Deserialize)]
struct FulfillmentsWrapper {
    fulfillments: Vec<Fulfillment>,
}

#[derive(Debug, Serialize)]
struct FulfillmentRequest {
    fulfillment: Fulfillment,
}

impl Fulfillment {
    /// Find a fulfillment by ID for a specific order
    pub async fn find(client: &Client, order_id: i64, id: i64) -> Result<Option<Self>> {
        let path = format!("orders/{}/fulfillments/{}.json", order_id, id);
        let response = client.get::<FulfillmentWrapper>(&path).await?;
        Ok(Some(response.data.fulfillment))
    }

    /// Find a fulfillment by ID with specific fields
    pub async fn find_with_fields(
        client: &Client,
        order_id: i64,
        id: i64,
        fields: &str,
    ) -> Result<Option<Self>> {
        let path = format!("orders/{}/fulfillments/{}.json?fields={}", order_id, id, fields);
        let response = client.get::<FulfillmentWrapper>(&path).await?;
        Ok(Some(response.data.fulfillment))
    }

    /// List all fulfillments for an order
    pub async fn all_for_order(
        client: &Client,
        order_id: i64,
        params: FulfillmentListParams,
    ) -> Result<FindAllResponse<Self>> {
        let path = format!("orders/{}/fulfillments.json", order_id);
        let response = client
            .get_with_params::<FulfillmentsWrapper, _>(&path, &params)
            .await?;
        
        Ok(FindAllResponse {
            data: response.data.fulfillments,
            next_page: response.page_info.as_ref().and_then(|p| p.next.clone()),
            previous_page: response.page_info.as_ref().and_then(|p| p.previous.clone()),
        })
    }

    /// List all fulfillments for a fulfillment order
    pub async fn all_for_fulfillment_order(
        client: &Client,
        fulfillment_order_id: i64,
        params: FulfillmentListParams,
    ) -> Result<FindAllResponse<Self>> {
        let path = format!("fulfillment_orders/{}/fulfillments.json", fulfillment_order_id);
        let response = client
            .get_with_params::<FulfillmentsWrapper, _>(&path, &params)
            .await?;
        
        Ok(FindAllResponse {
            data: response.data.fulfillments,
            next_page: response.page_info.as_ref().and_then(|p| p.next.clone()),
            previous_page: response.page_info.as_ref().and_then(|p| p.previous.clone()),
        })
    }

    /// Count fulfillments for an order
    pub async fn count(client: &Client, order_id: i64, params: FulfillmentCountParams) -> Result<i64> {
        let path = format!("orders/{}/fulfillments/count.json", order_id);
        let response = client
            .get_with_params::<CountResponse, _>(&path, &params)
            .await?;
        Ok(response.data.count)
    }

    /// Create a new fulfillment
    pub async fn create(client: &Client, fulfillment: &Self) -> Result<Self> {
        let request = FulfillmentRequest {
            fulfillment: fulfillment.clone(),
        };
        let response = client
            .post::<FulfillmentWrapper, _>("fulfillments.json", &request)
            .await?;
        Ok(response.data.fulfillment)
    }

    /// Cancel a fulfillment
    pub async fn cancel(client: &Client, id: i64) -> Result<Self> {
        let path = format!("fulfillments/{}/cancel.json", id);
        let response = client
            .post::<FulfillmentWrapper, _>(&path, &serde_json::json!({}))
            .await?;
        Ok(response.data.fulfillment)
    }

    /// Update tracking information
    pub async fn update_tracking(client: &Client, id: i64, params: UpdateTrackingParams) -> Result<Self> {
        let path = format!("fulfillments/{}/update_tracking.json", id);
        
        #[derive(Serialize)]
        struct UpdateTrackingRequest {
            fulfillment: UpdateTrackingParams,
        }
        
        let request = UpdateTrackingRequest { fulfillment: params };
        let response = client
            .post::<FulfillmentWrapper, _>(&path, &request)
            .await?;
        Ok(response.data.fulfillment)
    }
}
