//! Page resource

use crate::{base::{CountResponse, FindAllResponse}, client::Client, error::Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Page {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shop_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub handle: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body_html: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub published_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_suffix: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub admin_graphql_api_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct PageListParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub since_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub handle: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at_min: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at_max: Option<String>,
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
struct PageWrapper { page: Page }
#[derive(Debug, Deserialize)]
struct PagesWrapper { pages: Vec<Page> }
#[derive(Debug, Serialize)]
struct PageRequest { page: Page }

impl Page {
    pub async fn find(client: &Client, id: i64) -> Result<Option<Self>> {
        let path = format!("pages/{}.json", id);
        let response = client.get::<PageWrapper>(&path).await?;
        Ok(Some(response.data.page))
    }

    pub async fn all(client: &Client, params: PageListParams) -> Result<FindAllResponse<Self>> {
        let response = client.get_with_params::<PagesWrapper, _>("pages.json", &params).await?;
        Ok(FindAllResponse {
            data: response.data.pages,
            next_page: response.page_info.as_ref().and_then(|p| p.next.clone()),
            previous_page: response.page_info.as_ref().and_then(|p| p.previous.clone()),
        })
    }

    pub async fn count(client: &Client) -> Result<i64> {
        let response = client.get::<CountResponse>("pages/count.json").await?;
        Ok(response.data.count)
    }

    pub async fn create(client: &Client, page: &Self) -> Result<Self> {
        let request = PageRequest { page: page.clone() };
        let response = client.post::<PageWrapper, _>("pages.json", &request).await?;
        Ok(response.data.page)
    }

    pub async fn update(client: &Client, page: &Self) -> Result<Self> {
        let id = page.id.ok_or_else(|| crate::ShopifyError::ValidationError("Page ID required".to_string()))?;
        let path = format!("pages/{}.json", id);
        let request = PageRequest { page: page.clone() };
        let response = client.put::<PageWrapper, _>(&path, &request).await?;
        Ok(response.data.page)
    }

    pub async fn delete(client: &Client, id: i64) -> Result<()> {
        let path = format!("pages/{}.json", id);
        client.delete(&path).await
    }
}
