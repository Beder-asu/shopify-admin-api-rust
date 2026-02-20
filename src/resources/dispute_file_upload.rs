//! Dispute File Upload resource

use crate::client::Client;
use crate::error::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DisputeFileUpload {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dispute_evidence_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dispute_evidence_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub original_filename: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shop_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

#[derive(Debug, Deserialize)]
struct DisputeFileUploadWrapper { dispute_file_upload: DisputeFileUpload }
#[derive(Debug, Serialize)]
struct DisputeFileUploadRequest { dispute_file_upload: DisputeFileUpload }

impl DisputeFileUpload {
    pub async fn create(client: &Client, dispute_id: i64, file: &Self) -> Result<Self> {
        let path = format!("shopify_payments/disputes/{}/dispute_file_uploads.json", dispute_id);
        let request = DisputeFileUploadRequest { dispute_file_upload: file.clone() };
        let response = client.post::<DisputeFileUploadWrapper, _>(&path, &request).await?;
        Ok(response.data.dispute_file_upload)
    }

    pub async fn delete(client: &Client, dispute_id: i64, id: i64) -> Result<()> {
        let path = format!("shopify_payments/disputes/{}/dispute_file_uploads/{}.json", dispute_id, id);
        client.delete(&path).await
    }
}
