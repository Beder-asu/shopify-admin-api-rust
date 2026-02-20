//! Event resource

use crate::{base::FindAllResponse, client::Client, error::Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Event {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verb: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arguments: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct EventListParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub since_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at_min: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at_max: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verb: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<String>,
}

#[derive(Debug, Deserialize)]
struct EventWrapper { event: Event }
#[derive(Debug, Deserialize)]
struct EventsWrapper { events: Vec<Event> }

impl Event {
    pub async fn find(client: &Client, id: i64) -> Result<Option<Self>> {
        let path = format!("events/{}.json", id);
        let response = client.get::<EventWrapper>(&path).await?;
        Ok(Some(response.data.event))
    }

    pub async fn all(client: &Client, params: EventListParams) -> Result<FindAllResponse<Self>> {
        let response = client.get_with_params::<EventsWrapper, _>("events.json", &params).await?;
        Ok(FindAllResponse {
            data: response.data.events,
            next_page: response.page_info.as_ref().and_then(|p| p.next.clone()),
            previous_page: response.page_info.as_ref().and_then(|p| p.previous.clone()),
        })
    }

    pub async fn count(client: &Client, params: EventListParams) -> Result<i64> {
        #[derive(Deserialize)]
        struct CountWrapper { count: i64 }
        let response = client.get_with_params::<CountWrapper, _>("events/count.json", &params).await?;
        Ok(response.data.count)
    }

    pub async fn for_order(client: &Client, order_id: i64, params: EventListParams) -> Result<FindAllResponse<Self>> {
        let path = format!("orders/{}/events.json", order_id);
        let response = client.get_with_params::<EventsWrapper, _>(&path, &params).await?;
        Ok(FindAllResponse {
            data: response.data.events,
            next_page: response.page_info.as_ref().and_then(|p| p.next.clone()),
            previous_page: response.page_info.as_ref().and_then(|p| p.previous.clone()),
        })
    }

    pub async fn for_product(client: &Client, product_id: i64, params: EventListParams) -> Result<FindAllResponse<Self>> {
        let path = format!("products/{}/events.json", product_id);
        let response = client.get_with_params::<EventsWrapper, _>(&path, &params).await?;
        Ok(FindAllResponse {
            data: response.data.events,
            next_page: response.page_info.as_ref().and_then(|p| p.next.clone()),
            previous_page: response.page_info.as_ref().and_then(|p| p.previous.clone()),
        })
    }
}
