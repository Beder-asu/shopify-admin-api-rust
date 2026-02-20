//! Inventory Item resource

use crate::{base::FindAllResponse, client::Client, error::Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct InventoryItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sku: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requires_shipping: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cost: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_code_of_origin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub province_code_of_origin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub harmonized_system_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracked: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub admin_graphql_api_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct InventoryItemListParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ids: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
}

#[derive(Debug, Deserialize)]
struct InventoryItemWrapper { inventory_item: InventoryItem }
#[derive(Debug, Deserialize)]
struct InventoryItemsWrapper { inventory_items: Vec<InventoryItem> }
#[derive(Debug, Serialize)]
struct InventoryItemRequest { inventory_item: InventoryItem }

impl InventoryItem {
    pub async fn find(client: &Client, id: i64) -> Result<Option<Self>> {
        let path = format!("inventory_items/{}.json", id);
        let response = client.get::<InventoryItemWrapper>(&path).await?;
        Ok(Some(response.data.inventory_item))
    }

    pub async fn all(client: &Client, params: InventoryItemListParams) -> Result<FindAllResponse<Self>> {
        let response = client.get_with_params::<InventoryItemsWrapper, _>("inventory_items.json", &params).await?;
        Ok(FindAllResponse {
            data: response.data.inventory_items,
            next_page: response.page_info.as_ref().and_then(|p| p.next.clone()),
            previous_page: response.page_info.as_ref().and_then(|p| p.previous.clone()),
        })
    }

    pub async fn update(client: &Client, item: &Self) -> Result<Self> {
        let id = item.id.ok_or_else(|| crate::ShopifyError::ValidationError("Inventory Item ID required".to_string()))?;
        let path = format!("inventory_items/{}.json", id);
        let request = InventoryItemRequest { inventory_item: item.clone() };
        let response = client.put::<InventoryItemWrapper, _>(&path, &request).await?;
        Ok(response.data.inventory_item)
    }
}
