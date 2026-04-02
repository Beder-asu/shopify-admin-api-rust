//! ScriptTag resource

use crate::{base::{CountResponse, FindAllResponse}, client::Client, error::Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ScriptTag {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub src: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_scope: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct ScriptTagListParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub since_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub src: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<String>,
}

#[derive(Debug, Deserialize)]
struct ScriptTagWrapper { script_tag: ScriptTag }
#[derive(Debug, Deserialize)]
struct ScriptTagsWrapper { script_tags: Vec<ScriptTag> }
#[derive(Debug, Serialize)]
struct ScriptTagRequest { script_tag: ScriptTag }

impl ScriptTag {
    pub async fn find(client: &Client, id: i64) -> Result<Option<Self>> {
        let path = format!("script_tags/{}.json", id);
        let response = client.get::<ScriptTagWrapper>(&path).await?;
        Ok(Some(response.data.script_tag))
    }

    pub async fn all(client: &Client, params: ScriptTagListParams) -> Result<FindAllResponse<Self>> {
        let response = client.get_with_params::<ScriptTagsWrapper, _>("script_tags.json", &params).await?;
        Ok(FindAllResponse {
            data: response.data.script_tags,
            next_page: response.page_info.as_ref().and_then(|p| p.next.clone()),
            previous_page: response.page_info.as_ref().and_then(|p| p.previous.clone()),
        })
    }

    pub async fn count(client: &Client) -> Result<i64> {
        let response = client.get::<CountResponse>("script_tags/count.json").await?;
        Ok(response.data.count)
    }

    pub async fn create(client: &Client, script_tag: &Self) -> Result<Self> {
        let request = ScriptTagRequest { script_tag: script_tag.clone() };
        let response = client.post::<ScriptTagWrapper, _>("script_tags.json", &request).await?;
        Ok(response.data.script_tag)
    }

    pub async fn update(client: &Client, script_tag: &Self) -> Result<Self> {
        let id = script_tag.id.ok_or_else(|| crate::ShopifyError::ValidationError("ScriptTag ID required".to_string()))?;
        let path = format!("script_tags/{}.json", id);
        let request = ScriptTagRequest { script_tag: script_tag.clone() };
        let response = client.put::<ScriptTagWrapper, _>(&path, &request).await?;
        Ok(response.data.script_tag)
    }

    pub async fn delete(client: &Client, id: i64) -> Result<()> {
        let path = format!("script_tags/{}.json", id);
        client.delete(&path).await
    }
}
