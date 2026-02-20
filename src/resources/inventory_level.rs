//! Inventory Level resource

use crate::{base::FindAllResponse, client::Client, error::Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct InventoryLevel {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inventory_item_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub available: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub admin_graphql_api_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct InventoryLevelListParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inventory_item_ids: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_ids: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at_min: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct InventoryAdjustParams {
    pub inventory_item_id: i64,
    pub location_id: i64,
    pub available_adjustment: i32,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct InventorySetParams {
    pub inventory_item_id: i64,
    pub location_id: i64,
    pub available: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disconnect_if_necessary: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct InventoryConnectParams {
    pub inventory_item_id: i64,
    pub location_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relocate_if_necessary: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct InventoryDeleteParams {
    pub inventory_item_id: i64,
    pub location_id: i64,
}

#[derive(Debug, Deserialize)]
struct InventoryLevelWrapper { inventory_level: InventoryLevel }
#[derive(Debug, Deserialize)]
struct InventoryLevelsWrapper { inventory_levels: Vec<InventoryLevel> }

impl InventoryLevel {
    pub async fn all(client: &Client, params: InventoryLevelListParams) -> Result<FindAllResponse<Self>> {
        let response = client.get_with_params::<InventoryLevelsWrapper, _>("inventory_levels.json", &params).await?;
        Ok(FindAllResponse {
            data: response.data.inventory_levels,
            next_page: response.page_info.as_ref().and_then(|p| p.next.clone()),
            previous_page: response.page_info.as_ref().and_then(|p| p.previous.clone()),
        })
    }

    pub async fn adjust(client: &Client, params: InventoryAdjustParams) -> Result<Self> {
        let response = client.post::<InventoryLevelWrapper, _>("inventory_levels/adjust.json", &params).await?;
        Ok(response.data.inventory_level)
    }

    pub async fn set(client: &Client, params: InventorySetParams) -> Result<Self> {
        let response = client.post::<InventoryLevelWrapper, _>("inventory_levels/set.json", &params).await?;
        Ok(response.data.inventory_level)
    }

    pub async fn connect(client: &Client, params: InventoryConnectParams) -> Result<Self> {
        let response = client.post::<InventoryLevelWrapper, _>("inventory_levels/connect.json", &params).await?;
        Ok(response.data.inventory_level)
    }

    pub async fn delete(client: &Client, params: InventoryDeleteParams) -> Result<()> {
        client.delete_with_params("inventory_levels.json", &params).await
    }
}
