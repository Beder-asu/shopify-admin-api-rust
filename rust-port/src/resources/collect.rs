//! Collect resource

use crate::{base::FindAllResponse, client::Client, error::Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Collect {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_value: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct CollectListParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub since_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<String>,
}

#[derive(Debug, Deserialize)]
struct CollectWrapper { collect: Collect }
#[derive(Debug, Deserialize)]
struct CollectsWrapper { collects: Vec<Collect> }
#[derive(Debug, Serialize)]
struct CollectRequest { collect: Collect }

impl Collect {
    pub async fn find(client: &Client, id: i64) -> Result<Option<Self>> {
        let path = format!("collects/{}.json", id);
        let response = client.get::<CollectWrapper>(&path).await?;
        Ok(Some(response.data.collect))
    }

    pub async fn all(client: &Client, params: CollectListParams) -> Result<FindAllResponse<Self>> {
        let response = client.get_with_params::<CollectsWrapper, _>("collects.json", &params).await?;
        Ok(FindAllResponse {
            data: response.data.collects,
            next_page: response.page_info.as_ref().and_then(|p| p.next.clone()),
            previous_page: response.page_info.as_ref().and_then(|p| p.previous.clone()),
        })
    }

    pub async fn count(client: &Client, collection_id: Option<i64>, product_id: Option<i64>) -> Result<i64> {
        let mut params = vec![];
        if let Some(cid) = collection_id {
            params.push(format!("collection_id={}", cid));
        }
        if let Some(pid) = product_id {
            params.push(format!("product_id={}", pid));
        }
        let query = if params.is_empty() { String::new() } else { format!("?{}", params.join("&")) };
        let path = format!("collects/count.json{}", query);
        #[derive(Deserialize)]
        struct CountWrapper { count: i64 }
        let response = client.get::<CountWrapper>(&path).await?;
        Ok(response.data.count)
    }

    pub async fn create(client: &Client, collect: &Self) -> Result<Self> {
        let request = CollectRequest { collect: collect.clone() };
        let response = client.post::<CollectWrapper, _>("collects.json", &request).await?;
        Ok(response.data.collect)
    }

    pub async fn delete(client: &Client, id: i64) -> Result<()> {
        let path = format!("collects/{}.json", id);
        client.delete(&path).await
    }
}
