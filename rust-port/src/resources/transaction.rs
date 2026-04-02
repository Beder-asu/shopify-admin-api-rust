//! Transaction resource

use crate::{base::{CountResponse, FindAllResponse}, client::Client, error::Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Transaction {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorization: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processed_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_details: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receipt: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct TransactionListParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub since_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<String>,
}

#[derive(Debug, Deserialize)]
struct TransactionWrapper { transaction: Transaction }
#[derive(Debug, Deserialize)]
struct TransactionsWrapper { transactions: Vec<Transaction> }
#[derive(Debug, Serialize)]
struct TransactionRequest { transaction: Transaction }

impl Transaction {
    pub async fn find(client: &Client, order_id: i64, id: i64) -> Result<Option<Self>> {
        let path = format!("orders/{}/transactions/{}.json", order_id, id);
        let response = client.get::<TransactionWrapper>(&path).await?;
        Ok(Some(response.data.transaction))
    }

    pub async fn all(client: &Client, order_id: i64, params: TransactionListParams) -> Result<FindAllResponse<Self>> {
        let path = format!("orders/{}/transactions.json", order_id);
        let response = client.get_with_params::<TransactionsWrapper, _>(&path, &params).await?;
        Ok(FindAllResponse {
            data: response.data.transactions,
            next_page: response.page_info.as_ref().and_then(|p| p.next.clone()),
            previous_page: response.page_info.as_ref().and_then(|p| p.previous.clone()),
        })
    }

    pub async fn count(client: &Client, order_id: i64) -> Result<i64> {
        let path = format!("orders/{}/transactions/count.json", order_id);
        let response = client.get::<CountResponse>(&path).await?;
        Ok(response.data.count)
    }

    pub async fn create(client: &Client, order_id: i64, transaction: &Self) -> Result<Self> {
        let path = format!("orders/{}/transactions.json", order_id);
        let request = TransactionRequest { transaction: transaction.clone() };
        let response = client.post::<TransactionWrapper, _>(&path, &request).await?;
        Ok(response.data.transaction)
    }
}
