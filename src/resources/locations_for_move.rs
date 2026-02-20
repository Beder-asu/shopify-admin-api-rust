//! Locations for Move resource

use crate::{base::FindAllResponse, client::Client, error::Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LocationsForMove {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub movable: Option<bool>,
}

#[derive(Debug, Deserialize)]
struct LocationsForMoveWrapper { locations_for_move: Vec<LocationsForMove> }

impl LocationsForMove {
    pub async fn all(client: &Client, fulfillment_order_id: i64) -> Result<FindAllResponse<Self>> {
        let path = format!("fulfillment_orders/{}/locations_for_move.json", fulfillment_order_id);
        let response = client.get::<LocationsForMoveWrapper>(&path).await?;
        Ok(FindAllResponse {
            data: response.data.locations_for_move,
            next_page: response.page_info.as_ref().and_then(|p| p.next.clone()),
            previous_page: response.page_info.as_ref().and_then(|p| p.previous.clone()),
        })
    }
}
