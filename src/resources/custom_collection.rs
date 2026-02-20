//! Custom Collection resource

use crate::{base::{CountResponse, FindAllResponse}, client::Client, error::Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CustomCollection {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub handle: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body_html: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub published_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_suffix: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub published_scope: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub admin_graphql_api_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct CustomCollectionListParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ids: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub since_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub handle: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at_min: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at_max: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub published_at_min: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub published_at_max: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub published_status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<String>,
}

#[derive(Debug, Deserialize)]
struct CustomCollectionWrapper { custom_collection: CustomCollection }
#[derive(Debug, Deserialize)]
struct CustomCollectionsWrapper { custom_collections: Vec<CustomCollection> }
#[derive(Debug, Serialize)]
struct CustomCollectionRequest { custom_collection: CustomCollection }

impl CustomCollection {
    pub async fn find(client: &Client, id: i64) -> Result<Option<Self>> {
        let path = format!("custom_collections/{}.json", id);
        let response = client.get::<CustomCollectionWrapper>(&path).await?;
        Ok(Some(response.data.custom_collection))
    }

    pub async fn all(client: &Client, params: CustomCollectionListParams) -> Result<FindAllResponse<Self>> {
        let response = client.get_with_params::<CustomCollectionsWrapper, _>("custom_collections.json", &params).await?;
        Ok(FindAllResponse {
            data: response.data.custom_collections,
            next_page: response.page_info.as_ref().and_then(|p| p.next.clone()),
            previous_page: response.page_info.as_ref().and_then(|p| p.previous.clone()),
        })
    }

    pub async fn count(client: &Client) -> Result<i64> {
        let response = client.get::<CountResponse>("custom_collections/count.json").await?;
        Ok(response.data.count)
    }

    pub async fn create(client: &Client, collection: &Self) -> Result<Self> {
        let request = CustomCollectionRequest { custom_collection: collection.clone() };
        let response = client.post::<CustomCollectionWrapper, _>("custom_collections.json", &request).await?;
        Ok(response.data.custom_collection)
    }

    pub async fn update(client: &Client, collection: &Self) -> Result<Self> {
        let id = collection.id.ok_or_else(|| crate::ShopifyError::ValidationError("Collection ID required".to_string()))?;
        let path = format!("custom_collections/{}.json", id);
        let request = CustomCollectionRequest { custom_collection: collection.clone() };
        let response = client.put::<CustomCollectionWrapper, _>(&path, &request).await?;
        Ok(response.data.custom_collection)
    }

    pub async fn delete(client: &Client, id: i64) -> Result<()> {
        let path = format!("custom_collections/{}.json", id);
        client.delete(&path).await
    }
}
