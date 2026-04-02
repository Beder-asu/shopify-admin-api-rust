//! Resource Feedback resource

use crate::{base::FindAllResponse, client::Client, error::Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ResourceFeedback {
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
struct ResourceFeedbackWrapper { resource_feedback: ResourceFeedback }
#[derive(Debug, Deserialize)]
struct ResourceFeedbacksWrapper { resource_feedback: Vec<ResourceFeedback> }
#[derive(Debug, Serialize)]
struct ResourceFeedbackRequest { resource_feedback: ResourceFeedback }

impl ResourceFeedback {
    pub async fn all(client: &Client) -> Result<FindAllResponse<Self>> {
        let response = client.get::<ResourceFeedbacksWrapper>("resource_feedback.json").await?;
        Ok(FindAllResponse {
            data: response.data.resource_feedback,
            next_page: response.page_info.as_ref().and_then(|p| p.next.clone()),
            previous_page: response.page_info.as_ref().and_then(|p| p.previous.clone()),
        })
    }

    pub async fn create(client: &Client, feedback: &Self) -> Result<Self> {
        let request = ResourceFeedbackRequest { resource_feedback: feedback.clone() };
        let response = client.post::<ResourceFeedbackWrapper, _>("resource_feedback.json", &request).await?;
        Ok(response.data.resource_feedback)
    }
}
