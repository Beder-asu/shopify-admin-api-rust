//! Shipping Zone resource

use crate::{base::FindAllResponse, client::Client, error::Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ShippingZone {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_group_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub admin_graphql_api_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub countries: Option<Vec<ShippingCountry>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight_based_shipping_rates: Option<Vec<serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_based_shipping_rates: Option<Vec<serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub carrier_shipping_rate_providers: Option<Vec<serde_json::Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ShippingCountry {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_zone_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provinces: Option<Vec<serde_json::Value>>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct ShippingZoneListParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<String>,
}

#[derive(Debug, Deserialize)]
struct ShippingZonesWrapper { shipping_zones: Vec<ShippingZone> }

impl ShippingZone {
    pub async fn all(client: &Client, params: ShippingZoneListParams) -> Result<FindAllResponse<Self>> {
        let response = client.get_with_params::<ShippingZonesWrapper, _>("shipping_zones.json", &params).await?;
        Ok(FindAllResponse {
            data: response.data.shipping_zones,
            next_page: response.page_info.as_ref().and_then(|p| p.next.clone()),
            previous_page: response.page_info.as_ref().and_then(|p| p.previous.clone()),
        })
    }
}
