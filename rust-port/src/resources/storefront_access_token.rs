//! Storefront Access Token resource

use crate::{base::FindAllResponse, client::Client, error::Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct StorefrontAccessToken {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_scope: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

#[derive(Debug, Deserialize)]
struct StorefrontAccessTokenWrapper { storefront_access_token: StorefrontAccessToken }
#[derive(Debug, Deserialize)]
struct StorefrontAccessTokensWrapper { storefront_access_tokens: Vec<StorefrontAccessToken> }
#[derive(Debug, Serialize)]
struct StorefrontAccessTokenRequest { storefront_access_token: StorefrontAccessToken }

impl StorefrontAccessToken {
    pub async fn all(client: &Client) -> Result<FindAllResponse<Self>> {
        let response = client.get::<StorefrontAccessTokensWrapper>("storefront_access_tokens.json").await?;
        Ok(FindAllResponse {
            data: response.data.storefront_access_tokens,
            next_page: response.page_info.as_ref().and_then(|p| p.next.clone()),
            previous_page: response.page_info.as_ref().and_then(|p| p.previous.clone()),
        })
    }

    pub async fn create(client: &Client, token: &Self) -> Result<Self> {
        let request = StorefrontAccessTokenRequest { storefront_access_token: token.clone() };
        let response = client.post::<StorefrontAccessTokenWrapper, _>("storefront_access_tokens.json", &request).await?;
        Ok(response.data.storefront_access_token)
    }

    pub async fn delete(client: &Client, id: i64) -> Result<()> {
        let path = format!("storefront_access_tokens/{}.json", id);
        client.delete(&path).await
    }
}
