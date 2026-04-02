//! Balance resource (Shopify Payments)

use crate::client::Client;
use crate::error::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Balance {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<String>,
}

#[derive(Debug, Deserialize)]
struct BalanceWrapper { balance: Vec<Balance> }

impl Balance {
    pub async fn all(client: &Client) -> Result<Vec<Self>> {
        let response = client.get::<BalanceWrapper>("shopify_payments/balance.json").await?;
        Ok(response.data.balance)
    }
}
