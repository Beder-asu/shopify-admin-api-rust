//! Collection resource

use crate::{client::Client, error::Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Collection {
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

#[derive(Debug, Deserialize)]
struct CollectionWrapper { collection: Collection }

impl Collection {
    pub async fn find(client: &Client, id: i64) -> Result<Option<Self>> {
        let path = format!("collections/{}.json", id);
        let response = client.get::<CollectionWrapper>(&path).await?;
        Ok(Some(response.data.collection))
    }

    pub async fn products(client: &Client, id: i64, limit: Option<i32>) -> Result<serde_json::Value> {
        let path = format!("collections/{}/products.json", id);
        if let Some(limit) = limit {
            let response = client.get_with_params::<serde_json::Value, _>(&path, &[("limit", limit.to_string())]).await?;
            Ok(response.data)
        } else {
            let response = client.get::<serde_json::Value>(&path).await?;
            Ok(response.data)
        }
    }
}
