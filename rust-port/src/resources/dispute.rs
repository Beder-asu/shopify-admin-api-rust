//! Dispute resource (Shopify Payments)

use crate::{base::FindAllResponse, client::Client, error::Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Dispute {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_reason_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evidence_due_by: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evidence_sent_on: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finalized_on: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initiated_at: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct DisputeListParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub since_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initiated_at: Option<String>,
}

#[derive(Debug, Deserialize)]
struct DisputeWrapper { dispute: Dispute }
#[derive(Debug, Deserialize)]
struct DisputesWrapper { disputes: Vec<Dispute> }

impl Dispute {
    pub async fn find(client: &Client, id: i64) -> Result<Option<Self>> {
        let path = format!("shopify_payments/disputes/{}.json", id);
        let response = client.get::<DisputeWrapper>(&path).await?;
        Ok(Some(response.data.dispute))
    }

    pub async fn all(client: &Client, params: DisputeListParams) -> Result<FindAllResponse<Self>> {
        let response = client.get_with_params::<DisputesWrapper, _>("shopify_payments/disputes.json", &params).await?;
        Ok(FindAllResponse {
            data: response.data.disputes,
            next_page: response.page_info.as_ref().and_then(|p| p.next.clone()),
            previous_page: response.page_info.as_ref().and_then(|p| p.previous.clone()),
        })
    }
}
