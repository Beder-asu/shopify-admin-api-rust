//! Cancellation Request resource

use crate::client::Client;
use crate::error::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CancellationRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Debug, Deserialize)]
struct FulfillmentOrderWrapper { fulfillment_order: serde_json::Value }

impl CancellationRequest {
    pub async fn send(client: &Client, fulfillment_order_id: i64, message: Option<&str>) -> Result<serde_json::Value> {
        let path = format!("fulfillment_orders/{}/cancellation_request.json", fulfillment_order_id);
        let body = message.map(|m| serde_json::json!({"cancellation_request": {"message": m}})).unwrap_or(serde_json::json!({}));
        let response = client.post::<FulfillmentOrderWrapper, _>(&path, &body).await?;
        Ok(response.data.fulfillment_order)
    }

    pub async fn accept(client: &Client, fulfillment_order_id: i64, message: Option<&str>) -> Result<serde_json::Value> {
        let path = format!("fulfillment_orders/{}/cancellation_request/accept.json", fulfillment_order_id);
        let body = message.map(|m| serde_json::json!({"cancellation_request": {"message": m}})).unwrap_or(serde_json::json!({}));
        let response = client.post::<FulfillmentOrderWrapper, _>(&path, &body).await?;
        Ok(response.data.fulfillment_order)
    }

    pub async fn reject(client: &Client, fulfillment_order_id: i64, message: Option<&str>) -> Result<serde_json::Value> {
        let path = format!("fulfillment_orders/{}/cancellation_request/reject.json", fulfillment_order_id);
        let body = message.map(|m| serde_json::json!({"cancellation_request": {"message": m}})).unwrap_or(serde_json::json!({}));
        let response = client.post::<FulfillmentOrderWrapper, _>(&path, &body).await?;
        Ok(response.data.fulfillment_order)
    }
}
