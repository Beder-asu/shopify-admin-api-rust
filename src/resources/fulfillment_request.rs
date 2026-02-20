//! Fulfillment Request resource

use crate::{client::Client, error::Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FulfillmentRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fulfillment_order_line_items: Option<Vec<FulfillmentRequestLineItem>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FulfillmentRequestLineItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<i32>,
}

#[derive(Debug, Deserialize)]
struct OriginalFulfillmentOrderWrapper { original_fulfillment_order: serde_json::Value }
#[derive(Debug, Deserialize)]
struct FulfillmentOrderWrapper { fulfillment_order: serde_json::Value }

impl FulfillmentRequest {
    pub async fn send(client: &Client, fulfillment_order_id: i64, request: &Self) -> Result<serde_json::Value> {
        let path = format!("fulfillment_orders/{}/fulfillment_request.json", fulfillment_order_id);
        let body = serde_json::json!({"fulfillment_request": request});
        let response = client.post::<serde_json::Value, _>(&path, &body).await?;
        Ok(response.data)
    }

    pub async fn accept(client: &Client, fulfillment_order_id: i64, message: Option<&str>) -> Result<serde_json::Value> {
        let path = format!("fulfillment_orders/{}/fulfillment_request/accept.json", fulfillment_order_id);
        let body = message.map(|m| serde_json::json!({"fulfillment_request": {"message": m}})).unwrap_or(serde_json::json!({}));
        let response = client.post::<serde_json::Value, _>(&path, &body).await?;
        Ok(response.data)
    }

    pub async fn reject(client: &Client, fulfillment_order_id: i64, message: Option<&str>) -> Result<serde_json::Value> {
        let path = format!("fulfillment_orders/{}/fulfillment_request/reject.json", fulfillment_order_id);
        let body = message.map(|m| serde_json::json!({"fulfillment_request": {"message": m}})).unwrap_or(serde_json::json!({}));
        let response = client.post::<serde_json::Value, _>(&path, &body).await?;
        Ok(response.data)
    }
}
