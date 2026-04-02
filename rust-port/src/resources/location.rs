//! Location resource

use crate::{base::{CountResponse, FindAllResponse}, client::Client, error::Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Location {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address1: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address2: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zip: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub province: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub province_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub legacy: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub localized_country_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub localized_province_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub admin_graphql_api_id: Option<String>,
}

#[derive(Debug, Deserialize)]
struct LocationWrapper { location: Location }
#[derive(Debug, Deserialize)]
struct LocationsWrapper { locations: Vec<Location> }

impl Location {
    pub async fn find(client: &Client, id: i64) -> Result<Option<Self>> {
        let path = format!("locations/{}.json", id);
        let response = client.get::<LocationWrapper>(&path).await?;
        Ok(Some(response.data.location))
    }

    pub async fn all(client: &Client) -> Result<FindAllResponse<Self>> {
        let response = client.get::<LocationsWrapper>("locations.json").await?;
        Ok(FindAllResponse {
            data: response.data.locations,
            next_page: response.page_info.as_ref().and_then(|p| p.next.clone()),
            previous_page: response.page_info.as_ref().and_then(|p| p.previous.clone()),
        })
    }

    pub async fn count(client: &Client) -> Result<i64> {
        let response = client.get::<CountResponse>("locations/count.json").await?;
        Ok(response.data.count)
    }

    pub async fn inventory_levels(client: &Client, id: i64) -> Result<serde_json::Value> {
        let path = format!("locations/{}/inventory_levels.json", id);
        let response = client.get::<serde_json::Value>(&path).await?;
        Ok(response.data)
    }
}
