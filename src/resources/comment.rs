//! Comment resource

use crate::{base::FindAllResponse, client::Client, error::Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Comment {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body_html: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub article_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blog_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_agent: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub published_at: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct CommentListParams {
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
    pub fields: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub published_status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Deserialize)]
struct CommentWrapper { comment: Comment }
#[derive(Debug, Deserialize)]
struct CommentsWrapper { comments: Vec<Comment> }
#[derive(Debug, Serialize)]
struct CommentRequest { comment: Comment }

impl Comment {
    pub async fn find(client: &Client, id: i64) -> Result<Option<Self>> {
        let path = format!("comments/{}.json", id);
        let response = client.get::<CommentWrapper>(&path).await?;
        Ok(Some(response.data.comment))
    }

    pub async fn all(client: &Client, params: CommentListParams) -> Result<FindAllResponse<Self>> {
        let response = client.get_with_params::<CommentsWrapper, _>("comments.json", &params).await?;
        Ok(FindAllResponse {
            data: response.data.comments,
            next_page: response.page_info.as_ref().and_then(|p| p.next.clone()),
            previous_page: response.page_info.as_ref().and_then(|p| p.previous.clone()),
        })
    }

    pub async fn for_article(client: &Client, blog_id: i64, article_id: i64, params: CommentListParams) -> Result<FindAllResponse<Self>> {
        let path = format!("blogs/{}/articles/{}/comments.json", blog_id, article_id);
        let response = client.get_with_params::<CommentsWrapper, _>(&path, &params).await?;
        Ok(FindAllResponse {
            data: response.data.comments,
            next_page: response.page_info.as_ref().and_then(|p| p.next.clone()),
            previous_page: response.page_info.as_ref().and_then(|p| p.previous.clone()),
        })
    }

    pub async fn count(client: &Client, params: CommentListParams) -> Result<i64> {
        #[derive(Deserialize)]
        struct CountWrapper { count: i64 }
        let response = client.get_with_params::<CountWrapper, _>("comments/count.json", &params).await?;
        Ok(response.data.count)
    }

    pub async fn create(client: &Client, comment: &Self) -> Result<Self> {
        let request = CommentRequest { comment: comment.clone() };
        let response = client.post::<CommentWrapper, _>("comments.json", &request).await?;
        Ok(response.data.comment)
    }

    pub async fn update(client: &Client, comment: &Self) -> Result<Self> {
        let id = comment.id.ok_or_else(|| crate::ShopifyError::ValidationError("Comment ID required".to_string()))?;
        let path = format!("comments/{}.json", id);
        let request = CommentRequest { comment: comment.clone() };
        let response = client.put::<CommentWrapper, _>(&path, &request).await?;
        Ok(response.data.comment)
    }

    pub async fn spam(client: &Client, id: i64) -> Result<Self> {
        let path = format!("comments/{}/spam.json", id);
        let response = client.post::<CommentWrapper, _>(&path, &serde_json::json!({})).await?;
        Ok(response.data.comment)
    }

    pub async fn not_spam(client: &Client, id: i64) -> Result<Self> {
        let path = format!("comments/{}/not_spam.json", id);
        let response = client.post::<CommentWrapper, _>(&path, &serde_json::json!({})).await?;
        Ok(response.data.comment)
    }

    pub async fn approve(client: &Client, id: i64) -> Result<Self> {
        let path = format!("comments/{}/approve.json", id);
        let response = client.post::<CommentWrapper, _>(&path, &serde_json::json!({})).await?;
        Ok(response.data.comment)
    }

    pub async fn remove(client: &Client, id: i64) -> Result<Self> {
        let path = format!("comments/{}/remove.json", id);
        let response = client.post::<CommentWrapper, _>(&path, &serde_json::json!({})).await?;
        Ok(response.data.comment)
    }

    pub async fn restore(client: &Client, id: i64) -> Result<Self> {
        let path = format!("comments/{}/restore.json", id);
        let response = client.post::<CommentWrapper, _>(&path, &serde_json::json!({})).await?;
        Ok(response.data.comment)
    }
}
