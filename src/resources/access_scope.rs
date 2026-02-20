//! Access Scope resource

use crate::{base::FindAllResponse, client::Client, error::Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AccessScope {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub handle: Option<String>,
}

#[derive(Debug, Deserialize)]
struct AccessScopesWrapper { access_scopes: Vec<AccessScope> }

impl AccessScope {
    pub async fn all(client: &Client) -> Result<FindAllResponse<Self>> {
        let response = client.get::<AccessScopesWrapper>("oauth/access_scopes.json").await?;
        Ok(FindAllResponse {
            data: response.data.access_scopes,
            next_page: response.page_info.as_ref().and_then(|p| p.next.clone()),
            previous_page: response.page_info.as_ref().and_then(|p| p.previous.clone()),
        })
    }
}
