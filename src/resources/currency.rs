//! Currency resource

use crate::{base::FindAllResponse, client::Client, error::Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Currency {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate_updated_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

#[derive(Debug, Deserialize)]
struct CurrenciesWrapper { currencies: Vec<Currency> }

impl Currency {
    pub async fn all(client: &Client) -> Result<FindAllResponse<Self>> {
        let response = client.get::<CurrenciesWrapper>("currencies.json").await?;
        Ok(FindAllResponse {
            data: response.data.currencies,
            next_page: response.page_info.as_ref().and_then(|p| p.next.clone()),
            previous_page: response.page_info.as_ref().and_then(|p| p.previous.clone()),
        })
    }
}
