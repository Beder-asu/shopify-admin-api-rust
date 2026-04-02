//! Fulfillment Order resource

use crate::{base::FindAllResponse, client::Client, error::Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FulfillmentOrder {
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
    pub line_items: Option<Vec<FulfillmentOrderLineItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fulfill_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fulfill_by: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub international_duties: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fulfillment_holds: Option<Vec<serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_method: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assigned_location: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merchant_requests: Option<Vec<serde_json::Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FulfillmentOrderLineItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shop_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fulfillment_order_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_item_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inventory_item_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fulfillable_quantity: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variant_id: Option<i64>,
}

#[derive(Debug, Deserialize)]
struct FulfillmentOrderWrapper { fulfillment_order: FulfillmentOrder }
#[derive(Debug, Deserialize)]
struct FulfillmentOrdersWrapper { fulfillment_orders: Vec<FulfillmentOrder> }

impl FulfillmentOrder {
    pub async fn find(client: &Client, id: i64) -> Result<Option<Self>> {
        let path = format!("fulfillment_orders/{}.json", id);
        let response = client.get::<FulfillmentOrderWrapper>(&path).await?;
        Ok(Some(response.data.fulfillment_order))
    }

    pub async fn all_for_order(client: &Client, order_id: i64) -> Result<FindAllResponse<Self>> {
        let path = format!("orders/{}/fulfillment_orders.json", order_id);
        let response = client.get::<FulfillmentOrdersWrapper>(&path).await?;
        Ok(FindAllResponse {
            data: response.data.fulfillment_orders,
            next_page: response.page_info.as_ref().and_then(|p| p.next.clone()),
            previous_page: response.page_info.as_ref().and_then(|p| p.previous.clone()),
        })
    }

    pub async fn cancel(client: &Client, id: i64) -> Result<Self> {
        let path = format!("fulfillment_orders/{}/cancel.json", id);
        let response = client.post::<FulfillmentOrderWrapper, _>(&path, &serde_json::json!({})).await?;
        Ok(response.data.fulfillment_order)
    }

    pub async fn close(client: &Client, id: i64, message: Option<&str>) -> Result<Self> {
        let path = format!("fulfillment_orders/{}/close.json", id);
        let body = message.map(|m| serde_json::json!({"fulfillment_order": {"message": m}})).unwrap_or(serde_json::json!({}));
        let response = client.post::<FulfillmentOrderWrapper, _>(&path, &body).await?;
        Ok(response.data.fulfillment_order)
    }

    pub async fn hold(client: &Client, id: i64, reason: &str, reason_notes: Option<&str>) -> Result<Self> {
        let path = format!("fulfillment_orders/{}/hold.json", id);
        let mut body = serde_json::json!({"fulfillment_hold": {"reason": reason}});
        if let Some(notes) = reason_notes {
            body["fulfillment_hold"]["reason_notes"] = serde_json::json!(notes);
        }
        let response = client.post::<FulfillmentOrderWrapper, _>(&path, &body).await?;
        Ok(response.data.fulfillment_order)
    }

    pub async fn release_hold(client: &Client, id: i64) -> Result<Self> {
        let path = format!("fulfillment_orders/{}/release_hold.json", id);
        let response = client.post::<FulfillmentOrderWrapper, _>(&path, &serde_json::json!({})).await?;
        Ok(response.data.fulfillment_order)
    }

    pub async fn reschedule(client: &Client, id: i64, new_fulfill_at: &str) -> Result<Self> {
        let path = format!("fulfillment_orders/{}/reschedule.json", id);
        let body = serde_json::json!({"fulfillment_order": {"new_fulfill_at": new_fulfill_at}});
        let response = client.post::<FulfillmentOrderWrapper, _>(&path, &body).await?;
        Ok(response.data.fulfillment_order)
    }

    pub async fn r#move(client: &Client, id: i64, new_location_id: i64) -> Result<serde_json::Value> {
        let path = format!("fulfillment_orders/{}/move.json", id);
        let body = serde_json::json!({"fulfillment_order": {"new_location_id": new_location_id}});
        let response = client.post::<serde_json::Value, _>(&path, &body).await?;
        Ok(response.data)
    }
}
