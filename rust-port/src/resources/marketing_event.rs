//! Marketing Event resource

use crate::{base::FindAllResponse, client::Client, error::Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MarketingEvent {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marketing_channel: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paid: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub referring_domain: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub budget: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub budget_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduled_to_end_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ended_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub utm_campaign: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub utm_source: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub utm_medium: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manage_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preview_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marketed_resources: Option<Vec<serde_json::Value>>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct MarketingEventListParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i32>,
}

#[derive(Debug, Deserialize)]
struct MarketingEventWrapper { marketing_event: MarketingEvent }
#[derive(Debug, Deserialize)]
struct MarketingEventsWrapper { marketing_events: Vec<MarketingEvent> }
#[derive(Debug, Serialize)]
struct MarketingEventRequest { marketing_event: MarketingEvent }

impl MarketingEvent {
    pub async fn find(client: &Client, id: i64) -> Result<Option<Self>> {
        let path = format!("marketing_events/{}.json", id);
        let response = client.get::<MarketingEventWrapper>(&path).await?;
        Ok(Some(response.data.marketing_event))
    }

    pub async fn all(client: &Client, params: MarketingEventListParams) -> Result<FindAllResponse<Self>> {
        let response = client.get_with_params::<MarketingEventsWrapper, _>("marketing_events.json", &params).await?;
        Ok(FindAllResponse {
            data: response.data.marketing_events,
            next_page: response.page_info.as_ref().and_then(|p| p.next.clone()),
            previous_page: response.page_info.as_ref().and_then(|p| p.previous.clone()),
        })
    }

    pub async fn count(client: &Client) -> Result<i64> {
        #[derive(Deserialize)]
        struct CountWrapper { count: i64 }
        let response = client.get::<CountWrapper>("marketing_events/count.json").await?;
        Ok(response.data.count)
    }

    pub async fn create(client: &Client, event: &Self) -> Result<Self> {
        let request = MarketingEventRequest { marketing_event: event.clone() };
        let response = client.post::<MarketingEventWrapper, _>("marketing_events.json", &request).await?;
        Ok(response.data.marketing_event)
    }

    pub async fn update(client: &Client, event: &Self) -> Result<Self> {
        let id = event.id.ok_or_else(|| crate::ShopifyError::ValidationError("Marketing Event ID required".to_string()))?;
        let path = format!("marketing_events/{}.json", id);
        let request = MarketingEventRequest { marketing_event: event.clone() };
        let response = client.put::<MarketingEventWrapper, _>(&path, &request).await?;
        Ok(response.data.marketing_event)
    }

    pub async fn delete(client: &Client, id: i64) -> Result<()> {
        let path = format!("marketing_events/{}.json", id);
        client.delete(&path).await
    }

    pub async fn add_engagements(client: &Client, id: i64, engagements: &serde_json::Value) -> Result<serde_json::Value> {
        let path = format!("marketing_events/{}/engagements.json", id);
        let response = client.post::<serde_json::Value, _>(&path, engagements).await?;
        Ok(response.data)
    }
}
