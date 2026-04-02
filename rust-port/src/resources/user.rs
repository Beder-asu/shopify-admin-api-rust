//! User resource

use crate::{base::FindAllResponse, client::Client, error::Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct User {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub im: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub screen_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bio: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_owner: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receive_announcements: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub admin_graphql_api_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tfa_enabled: Option<bool>,
}

#[derive(Debug, Deserialize)]
struct UserWrapper { user: User }
#[derive(Debug, Deserialize)]
struct UsersWrapper { users: Vec<User> }

impl User {
    pub async fn find(client: &Client, id: i64) -> Result<Option<Self>> {
        let path = format!("users/{}.json", id);
        let response = client.get::<UserWrapper>(&path).await?;
        Ok(Some(response.data.user))
    }

    pub async fn all(client: &Client) -> Result<FindAllResponse<Self>> {
        let response = client.get::<UsersWrapper>("users.json").await?;
        Ok(FindAllResponse {
            data: response.data.users,
            next_page: response.page_info.as_ref().and_then(|p| p.next.clone()),
            previous_page: response.page_info.as_ref().and_then(|p| p.previous.clone()),
        })
    }

    pub async fn current(client: &Client) -> Result<Self> {
        let response = client.get::<UserWrapper>("users/current.json").await?;
        Ok(response.data.user)
    }
}
