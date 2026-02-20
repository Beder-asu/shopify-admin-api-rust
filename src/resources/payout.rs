//! Payout resource (Shopify Payments)

use crate::{base::FindAllResponse, client::Client, error::Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Payout {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct PayoutListParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub since_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_min: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_max: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Deserialize)]
struct PayoutWrapper { payout: Payout }
#[derive(Debug, Deserialize)]
struct PayoutsWrapper { payouts: Vec<Payout> }

impl Payout {
    pub async fn find(client: &Client, id: i64) -> Result<Option<Self>> {
        let path = format!("shopify_payments/payouts/{}.json", id);
        let response = client.get::<PayoutWrapper>(&path).await?;
        Ok(Some(response.data.payout))
    }

    pub async fn all(client: &Client, params: PayoutListParams) -> Result<FindAllResponse<Self>> {
        let response = client.get_with_params::<PayoutsWrapper, _>("shopify_payments/payouts.json", &params).await?;
        Ok(FindAllResponse {
            data: response.data.payouts,
            next_page: response.page_info.as_ref().and_then(|p| p.next.clone()),
            previous_page: response.page_info.as_ref().and_then(|p| p.previous.clone()),
        })
    }
}
