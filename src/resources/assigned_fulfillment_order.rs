//! Assigned Fulfillment Order resource

use crate::{base::FindAllResponse, client::Client, error::Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AssignedFulfillmentOrder {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shop_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assigned_location_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_actions: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_items: Option<Vec<serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fulfill_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assigned_location: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct AssignedFulfillmentOrderListParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assignment_status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_ids: Option<String>,
}

#[derive(Debug, Deserialize)]
struct AssignedFulfillmentOrdersWrapper { fulfillment_orders: Vec<AssignedFulfillmentOrder> }

impl AssignedFulfillmentOrder {
    pub async fn all(client: &Client, params: AssignedFulfillmentOrderListParams) -> Result<FindAllResponse<Self>> {
        let response = client.get_with_params::<AssignedFulfillmentOrdersWrapper, _>("assigned_fulfillment_orders.json", &params).await?;
        Ok(FindAllResponse {
            data: response.data.fulfillment_orders,
            next_page: response.page_info.as_ref().and_then(|p| p.next.clone()),
            previous_page: response.page_info.as_ref().and_then(|p| p.previous.clone()),
        })
    }
}
