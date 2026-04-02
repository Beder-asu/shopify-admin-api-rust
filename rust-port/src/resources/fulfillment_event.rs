//! Fulfillment Event resource

use crate::{base::FindAllResponse, client::Client, error::Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FulfillmentEvent {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fulfillment_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub happened_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub province: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zip: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address1: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latitude: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub longitude: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shop_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_delivery_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub admin_graphql_api_id: Option<String>,
}

#[derive(Debug, Deserialize)]
struct FulfillmentEventWrapper { fulfillment_event: FulfillmentEvent }
#[derive(Debug, Deserialize)]
struct FulfillmentEventsWrapper { fulfillment_events: Vec<FulfillmentEvent> }
#[derive(Debug, Serialize)]
struct FulfillmentEventRequest { event: FulfillmentEvent }

impl FulfillmentEvent {
    pub async fn find(client: &Client, order_id: i64, fulfillment_id: i64, id: i64) -> Result<Option<Self>> {
        let path = format!("orders/{}/fulfillments/{}/events/{}.json", order_id, fulfillment_id, id);
        let response = client.get::<FulfillmentEventWrapper>(&path).await?;
        Ok(Some(response.data.fulfillment_event))
    }

    pub async fn all(client: &Client, order_id: i64, fulfillment_id: i64) -> Result<FindAllResponse<Self>> {
        let path = format!("orders/{}/fulfillments/{}/events.json", order_id, fulfillment_id);
        let response = client.get::<FulfillmentEventsWrapper>(&path).await?;
        Ok(FindAllResponse {
            data: response.data.fulfillment_events,
            next_page: response.page_info.as_ref().and_then(|p| p.next.clone()),
            previous_page: response.page_info.as_ref().and_then(|p| p.previous.clone()),
        })
    }

    pub async fn create(client: &Client, order_id: i64, fulfillment_id: i64, event: &Self) -> Result<Self> {
        let path = format!("orders/{}/fulfillments/{}/events.json", order_id, fulfillment_id);
        let request = FulfillmentEventRequest { event: event.clone() };
        let response = client.post::<FulfillmentEventWrapper, _>(&path, &request).await?;
        Ok(response.data.fulfillment_event)
    }

    pub async fn delete(client: &Client, order_id: i64, fulfillment_id: i64, id: i64) -> Result<()> {
        let path = format!("orders/{}/fulfillments/{}/events/{}.json", order_id, fulfillment_id, id);
        client.delete(&path).await
    }
}
