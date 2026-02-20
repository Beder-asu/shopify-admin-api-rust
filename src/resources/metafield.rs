//! Metafield resource

use crate::{base::{CountResponse, FindAllResponse}, client::Client, error::Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Metafield {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_resource: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub admin_graphql_api_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct MetafieldListParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub since_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<String>,
}

#[derive(Debug, Deserialize)]
struct MetafieldWrapper { metafield: Metafield }
#[derive(Debug, Deserialize)]
struct MetafieldsWrapper { metafields: Vec<Metafield> }
#[derive(Debug, Serialize)]
struct MetafieldRequest { metafield: Metafield }

impl Metafield {
    pub async fn find(client: &Client, id: i64) -> Result<Option<Self>> {
        let path = format!("metafields/{}.json", id);
        let response = client.get::<MetafieldWrapper>(&path).await?;
        Ok(Some(response.data.metafield))
    }

    pub async fn all(client: &Client, params: MetafieldListParams) -> Result<FindAllResponse<Self>> {
        let response = client.get_with_params::<MetafieldsWrapper, _>("metafields.json", &params).await?;
        Ok(FindAllResponse {
            data: response.data.metafields,
            next_page: response.page_info.as_ref().and_then(|p| p.next.clone()),
            previous_page: response.page_info.as_ref().and_then(|p| p.previous.clone()),
        })
    }

    pub async fn all_for_resource(client: &Client, resource: &str, resource_id: i64, params: MetafieldListParams) -> Result<FindAllResponse<Self>> {
        let path = format!("{}/{}/metafields.json", resource, resource_id);
        let response = client.get_with_params::<MetafieldsWrapper, _>(&path, &params).await?;
        Ok(FindAllResponse {
            data: response.data.metafields,
            next_page: response.page_info.as_ref().and_then(|p| p.next.clone()),
            previous_page: response.page_info.as_ref().and_then(|p| p.previous.clone()),
        })
    }

    pub async fn count(client: &Client) -> Result<i64> {
        let response = client.get::<CountResponse>("metafields/count.json").await?;
        Ok(response.data.count)
    }

    pub async fn create(client: &Client, metafield: &Self) -> Result<Self> {
        let request = MetafieldRequest { metafield: metafield.clone() };
        let response = client.post::<MetafieldWrapper, _>("metafields.json", &request).await?;
        Ok(response.data.metafield)
    }

    pub async fn update(client: &Client, metafield: &Self) -> Result<Self> {
        let id = metafield.id.ok_or_else(|| crate::ShopifyError::ValidationError("Metafield ID required".to_string()))?;
        let path = format!("metafields/{}.json", id);
        let request = MetafieldRequest { metafield: metafield.clone() };
        let response = client.put::<MetafieldWrapper, _>(&path, &request).await?;
        Ok(response.data.metafield)
    }

    pub async fn delete(client: &Client, id: i64) -> Result<()> {
        let path = format!("metafields/{}.json", id);
        client.delete(&path).await
    }
}
