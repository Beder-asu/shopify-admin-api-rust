//! Deprecated API Call resource

use crate::{base::FindAllResponse, client::Client, error::Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DeprecatedApiCall {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub documentation_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_call_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub migration_deadline: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub graphql_schema_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Deserialize)]
struct DeprecatedApiCallsWrapper { data_updated_at: Option<String>, deprecated_api_calls: Vec<DeprecatedApiCall> }

impl DeprecatedApiCall {
    pub async fn all(client: &Client) -> Result<FindAllResponse<Self>> {
        let response = client.get::<DeprecatedApiCallsWrapper>("deprecated_api_calls.json").await?;
        Ok(FindAllResponse {
            data: response.data.deprecated_api_calls,
            next_page: response.page_info.as_ref().and_then(|p| p.next.clone()),
            previous_page: response.page_info.as_ref().and_then(|p| p.previous.clone()),
        })
    }
}
