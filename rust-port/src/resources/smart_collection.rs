//! Smart Collection resource

use crate::{base::{CountResponse, FindAllResponse}, client::Client, error::Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SmartCollection {
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
    pub disjunctive: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rules: Option<Vec<SmartCollectionRule>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub published_scope: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub admin_graphql_api_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SmartCollectionRule {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relation: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct SmartCollectionListParams {
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
struct SmartCollectionWrapper { smart_collection: SmartCollection }
#[derive(Debug, Deserialize)]
struct SmartCollectionsWrapper { smart_collections: Vec<SmartCollection> }
#[derive(Debug, Serialize)]
struct SmartCollectionRequest { smart_collection: SmartCollection }

impl SmartCollection {
    pub async fn find(client: &Client, id: i64) -> Result<Option<Self>> {
        let path = format!("smart_collections/{}.json", id);
        let response = client.get::<SmartCollectionWrapper>(&path).await?;
        Ok(Some(response.data.smart_collection))
    }

    pub async fn all(client: &Client, params: SmartCollectionListParams) -> Result<FindAllResponse<Self>> {
        let response = client.get_with_params::<SmartCollectionsWrapper, _>("smart_collections.json", &params).await?;
        Ok(FindAllResponse {
            data: response.data.smart_collections,
            next_page: response.page_info.as_ref().and_then(|p| p.next.clone()),
            previous_page: response.page_info.as_ref().and_then(|p| p.previous.clone()),
        })
    }

    pub async fn count(client: &Client) -> Result<i64> {
        let response = client.get::<CountResponse>("smart_collections/count.json").await?;
        Ok(response.data.count)
    }

    pub async fn create(client: &Client, collection: &Self) -> Result<Self> {
        let request = SmartCollectionRequest { smart_collection: collection.clone() };
        let response = client.post::<SmartCollectionWrapper, _>("smart_collections.json", &request).await?;
        Ok(response.data.smart_collection)
    }

    pub async fn update(client: &Client, collection: &Self) -> Result<Self> {
        let id = collection.id.ok_or_else(|| crate::ShopifyError::ValidationError("Collection ID required".to_string()))?;
        let path = format!("smart_collections/{}.json", id);
        let request = SmartCollectionRequest { smart_collection: collection.clone() };
        let response = client.put::<SmartCollectionWrapper, _>(&path, &request).await?;
        Ok(response.data.smart_collection)
    }

    pub async fn delete(client: &Client, id: i64) -> Result<()> {
        let path = format!("smart_collections/{}.json", id);
        client.delete(&path).await
    }

    pub async fn order(client: &Client, id: i64, products: Vec<i64>) -> Result<()> {
        let path = format!("smart_collections/{}/order.json", id);
        let body = serde_json::json!({ "products": products });
        client.put::<serde_json::Value, _>(&path, &body).await?;
        Ok(())
    }
}
