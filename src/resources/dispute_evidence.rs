//! Dispute Evidence resource

use crate::client::Client;
use crate::error::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DisputeEvidence {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dispute_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payments_dispute_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_activity_log: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_address: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancellation_policy_disclosure: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancellation_rebuttal: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_email_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_first_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_last_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund_policy_disclosure: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund_refusal_explanation: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_address: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uncategorized_text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fulfillments: Option<Vec<serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dispute_evidence_files: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submitted: Option<bool>,
}

#[derive(Debug, Deserialize)]
struct DisputeEvidenceWrapper { dispute_evidence: DisputeEvidence }
#[derive(Debug, Serialize)]
struct DisputeEvidenceRequest { dispute_evidence: DisputeEvidence }

impl DisputeEvidence {
    pub async fn find(client: &Client, dispute_id: i64) -> Result<Option<Self>> {
        let path = format!("shopify_payments/disputes/{}/dispute_evidences.json", dispute_id);
        let response = client.get::<DisputeEvidenceWrapper>(&path).await?;
        Ok(Some(response.data.dispute_evidence))
    }

    pub async fn update(client: &Client, dispute_id: i64, evidence: &Self) -> Result<Self> {
        let path = format!("shopify_payments/disputes/{}/dispute_evidences.json", dispute_id);
        let request = DisputeEvidenceRequest { dispute_evidence: evidence.clone() };
        let response = client.put::<DisputeEvidenceWrapper, _>(&path, &request).await?;
        Ok(response.data.dispute_evidence)
    }
}
