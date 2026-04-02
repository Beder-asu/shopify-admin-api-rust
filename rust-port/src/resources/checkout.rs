//! Checkout resource

use crate::{base::FindAllResponse, client::Client, error::Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Checkout {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cart_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buyer_accepts_marketing: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub landing_site: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note_attributes: Option<Vec<serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub referring_site: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_lines: Option<Vec<serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subtotal_price: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub taxes_included: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_discounts: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_line_items_price: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_price: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_tax: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_weight: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub closed_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_identifier: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_locale: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_items: Option<Vec<serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub abandoned_checkout_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discount_codes: Option<Vec<serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_lines: Option<Vec<serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presentment_currency: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_address: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_address: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct CheckoutListParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub since_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at_min: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at_max: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at_min: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at_max: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Deserialize)]
struct CheckoutsWrapper { checkouts: Vec<Checkout> }

impl Checkout {
    pub async fn all(client: &Client, params: CheckoutListParams) -> Result<FindAllResponse<Self>> {
        let response = client.get_with_params::<CheckoutsWrapper, _>("checkouts.json", &params).await?;
        Ok(FindAllResponse {
            data: response.data.checkouts,
            next_page: response.page_info.as_ref().and_then(|p| p.next.clone()),
            previous_page: response.page_info.as_ref().and_then(|p| p.previous.clone()),
        })
    }

    pub async fn count(client: &Client) -> Result<i64> {
        #[derive(Deserialize)]
        struct CountWrapper { count: i64 }
        let response = client.get::<CountWrapper>("checkouts/count.json").await?;
        Ok(response.data.count)
    }
}
