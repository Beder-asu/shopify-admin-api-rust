//! Article resource

use crate::{base::{CountResponse, FindAllResponse}, client::Client, error::Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Article {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body_html: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blog_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub published_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary_html: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_suffix: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub handle: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub admin_graphql_api_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct ArticleListParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub since_id: Option<i64>,
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
    pub handle: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<String>,
}

#[derive(Debug, Deserialize)]
struct ArticleWrapper { article: Article }
#[derive(Debug, Deserialize)]
struct ArticlesWrapper { articles: Vec<Article> }
#[derive(Debug, Serialize)]
struct ArticleRequest { article: Article }

impl Article {
    pub async fn find(client: &Client, blog_id: i64, id: i64) -> Result<Option<Self>> {
        let path = format!("blogs/{}/articles/{}.json", blog_id, id);
        let response = client.get::<ArticleWrapper>(&path).await?;
        Ok(Some(response.data.article))
    }

    pub async fn all(client: &Client, blog_id: i64, params: ArticleListParams) -> Result<FindAllResponse<Self>> {
        let path = format!("blogs/{}/articles.json", blog_id);
        let response = client.get_with_params::<ArticlesWrapper, _>(&path, &params).await?;
        Ok(FindAllResponse {
            data: response.data.articles,
            next_page: response.page_info.as_ref().and_then(|p| p.next.clone()),
            previous_page: response.page_info.as_ref().and_then(|p| p.previous.clone()),
        })
    }

    pub async fn count(client: &Client, blog_id: i64) -> Result<i64> {
        let path = format!("blogs/{}/articles/count.json", blog_id);
        let response = client.get::<CountResponse>(&path).await?;
        Ok(response.data.count)
    }

    pub async fn create(client: &Client, blog_id: i64, article: &Self) -> Result<Self> {
        let path = format!("blogs/{}/articles.json", blog_id);
        let request = ArticleRequest { article: article.clone() };
        let response = client.post::<ArticleWrapper, _>(&path, &request).await?;
        Ok(response.data.article)
    }

    pub async fn update(client: &Client, blog_id: i64, article: &Self) -> Result<Self> {
        let id = article.id.ok_or_else(|| crate::ShopifyError::ValidationError("Article ID required".to_string()))?;
        let path = format!("blogs/{}/articles/{}.json", blog_id, id);
        let request = ArticleRequest { article: article.clone() };
        let response = client.put::<ArticleWrapper, _>(&path, &request).await?;
        Ok(response.data.article)
    }

    pub async fn delete(client: &Client, blog_id: i64, id: i64) -> Result<()> {
        let path = format!("blogs/{}/articles/{}.json", blog_id, id);
        client.delete(&path).await
    }

    pub async fn authors(client: &Client) -> Result<Vec<String>> {
        #[derive(Debug, Deserialize)]
        struct AuthorsResponse { authors: Vec<String> }
        let response = client.get::<AuthorsResponse>("articles/authors.json").await?;
        Ok(response.data.authors)
    }

    pub async fn tags(client: &Client) -> Result<Vec<String>> {
        #[derive(Debug, Deserialize)]
        struct TagsResponse { tags: Vec<String> }
        let response = client.get::<TagsResponse>("articles/tags.json").await?;
        Ok(response.data.tags)
    }
}
