//! Product Resource Feedback resource

use crate::{base::FindAllResponse, client::Client, error::Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProductResourceFeedback {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_updated_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub messages: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feedback_generated_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Debug, Deserialize)]
struct ResourceFeedbackWrapper { resource_feedback: ProductResourceFeedback }
#[derive(Debug, Deserialize)]
struct ResourceFeedbacksWrapper { resource_feedback: Vec<ProductResourceFeedback> }
#[derive(Debug, Serialize)]
struct ResourceFeedbackRequest { resource_feedback: ProductResourceFeedback }

impl ProductResourceFeedback {
    pub async fn all(client: &Client, product_id: i64) -> Result<FindAllResponse<Self>> {
        let path = format!("products/{}/resource_feedback.json", product_id);
        let response = client.get::<ResourceFeedbacksWrapper>(&path).await?;
        Ok(FindAllResponse {
            data: response.data.resource_feedback,
            next_page: response.page_info.as_ref().and_then(|p| p.next.clone()),
            previous_page: response.page_info.as_ref().and_then(|p| p.previous.clone()),
        })
    }

    pub async fn create(client: &Client, product_id: i64, feedback: &Self) -> Result<Self> {
        let path = format!("products/{}/resource_feedback.json", product_id);
        let request = ResourceFeedbackRequest { resource_feedback: feedback.clone() };
        let response = client.post::<ResourceFeedbackWrapper, _>(&path, &request).await?;
        Ok(response.data.resource_feedback)
    }
}
