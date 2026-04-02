//! Payment Transaction resource

use crate::{base::FindAllResponse, client::Client, error::Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PaymentTransaction {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payout_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payout_status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fee: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub net: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_order_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_order_transaction_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processed_at: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct PaymentTransactionListParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub since_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payout_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payout_status: Option<String>,
}

#[derive(Debug, Deserialize)]
struct TransactionsWrapper { transactions: Vec<PaymentTransaction> }

impl PaymentTransaction {
    pub async fn all(client: &Client, params: PaymentTransactionListParams) -> Result<FindAllResponse<Self>> {
        let response = client.get_with_params::<TransactionsWrapper, _>("shopify_payments/balance/transactions.json", &params).await?;
        Ok(FindAllResponse {
            data: response.data.transactions,
            next_page: response.page_info.as_ref().and_then(|p| p.next.clone()),
            previous_page: response.page_info.as_ref().and_then(|p| p.previous.clone()),
        })
    }
}
