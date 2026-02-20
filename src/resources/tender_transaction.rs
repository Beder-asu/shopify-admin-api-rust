//! Tender Transaction resource

use crate::{base::FindAllResponse, client::Client, error::Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TenderTransaction {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processed_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_reference: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_details: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct TenderTransactionListParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub since_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processed_at_min: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processed_at_max: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processed_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<String>,
}

#[derive(Debug, Deserialize)]
struct TenderTransactionsWrapper { tender_transactions: Vec<TenderTransaction> }

impl TenderTransaction {
    pub async fn all(client: &Client, params: TenderTransactionListParams) -> Result<FindAllResponse<Self>> {
        let response = client.get_with_params::<TenderTransactionsWrapper, _>("tender_transactions.json", &params).await?;
        Ok(FindAllResponse {
            data: response.data.tender_transactions,
            next_page: response.page_info.as_ref().and_then(|p| p.next.clone()),
            previous_page: response.page_info.as_ref().and_then(|p| p.previous.clone()),
        })
    }
}
