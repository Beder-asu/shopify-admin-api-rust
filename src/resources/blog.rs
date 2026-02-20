//! Blog resource

use crate::{base::{CountResponse, FindAllResponse}, client::Client, error::Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Blog {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub handle: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commentable: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feedburner: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feedburner_location: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_suffix: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub admin_graphql_api_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct BlogListParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub since_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub handle: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<String>,
}

#[derive(Debug, Deserialize)]
struct BlogWrapper { blog: Blog }
#[derive(Debug, Deserialize)]
struct BlogsWrapper { blogs: Vec<Blog> }
#[derive(Debug, Serialize)]
struct BlogRequest { blog: Blog }

impl Blog {
    pub async fn find(client: &Client, id: i64) -> Result<Option<Self>> {
        let path = format!("blogs/{}.json", id);
        let response = client.get::<BlogWrapper>(&path).await?;
        Ok(Some(response.data.blog))
    }

    pub async fn all(client: &Client, params: BlogListParams) -> Result<FindAllResponse<Self>> {
        let response = client.get_with_params::<BlogsWrapper, _>("blogs.json", &params).await?;
        Ok(FindAllResponse {
            data: response.data.blogs,
            next_page: response.page_info.as_ref().and_then(|p| p.next.clone()),
            previous_page: response.page_info.as_ref().and_then(|p| p.previous.clone()),
        })
    }

    pub async fn count(client: &Client) -> Result<i64> {
        let response = client.get::<CountResponse>("blogs/count.json").await?;
        Ok(response.data.count)
    }

    pub async fn create(client: &Client, blog: &Self) -> Result<Self> {
        let request = BlogRequest { blog: blog.clone() };
        let response = client.post::<BlogWrapper, _>("blogs.json", &request).await?;
        Ok(response.data.blog)
    }

    pub async fn update(client: &Client, blog: &Self) -> Result<Self> {
        let id = blog.id.ok_or_else(|| crate::ShopifyError::ValidationError("Blog ID required".to_string()))?;
        let path = format!("blogs/{}.json", id);
        let request = BlogRequest { blog: blog.clone() };
        let response = client.put::<BlogWrapper, _>(&path, &request).await?;
        Ok(response.data.blog)
    }

    pub async fn delete(client: &Client, id: i64) -> Result<()> {
        let path = format!("blogs/{}.json", id);
        client.delete(&path).await
    }
}
