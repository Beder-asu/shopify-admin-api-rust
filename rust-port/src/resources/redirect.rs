//! Redirect resource

use crate::{base::{CountResponse, FindAllResponse}, client::Client, error::Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Redirect {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct RedirectListParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub since_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<String>,
}

#[derive(Debug, Deserialize)]
struct RedirectWrapper { redirect: Redirect }
#[derive(Debug, Deserialize)]
struct RedirectsWrapper { redirects: Vec<Redirect> }
#[derive(Debug, Serialize)]
struct RedirectRequest { redirect: Redirect }

impl Redirect {
    pub async fn find(client: &Client, id: i64) -> Result<Option<Self>> {
        let path = format!("redirects/{}.json", id);
        let response = client.get::<RedirectWrapper>(&path).await?;
        Ok(Some(response.data.redirect))
    }

    pub async fn all(client: &Client, params: RedirectListParams) -> Result<FindAllResponse<Self>> {
        let response = client.get_with_params::<RedirectsWrapper, _>("redirects.json", &params).await?;
        Ok(FindAllResponse {
            data: response.data.redirects,
            next_page: response.page_info.as_ref().and_then(|p| p.next.clone()),
            previous_page: response.page_info.as_ref().and_then(|p| p.previous.clone()),
        })
    }

    pub async fn count(client: &Client) -> Result<i64> {
        let response = client.get::<CountResponse>("redirects/count.json").await?;
        Ok(response.data.count)
    }

    pub async fn create(client: &Client, redirect: &Self) -> Result<Self> {
        let request = RedirectRequest { redirect: redirect.clone() };
        let response = client.post::<RedirectWrapper, _>("redirects.json", &request).await?;
        Ok(response.data.redirect)
    }

    pub async fn update(client: &Client, redirect: &Self) -> Result<Self> {
        let id = redirect.id.ok_or_else(|| crate::ShopifyError::ValidationError("Redirect ID required".to_string()))?;
        let path = format!("redirects/{}.json", id);
        let request = RedirectRequest { redirect: redirect.clone() };
        let response = client.put::<RedirectWrapper, _>(&path, &request).await?;
        Ok(response.data.redirect)
    }

    pub async fn delete(client: &Client, id: i64) -> Result<()> {
        let path = format!("redirects/{}.json", id);
        client.delete(&path).await
    }
}
