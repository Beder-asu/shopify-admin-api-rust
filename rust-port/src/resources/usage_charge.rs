//! Usage Charge resource

use crate::{base::FindAllResponse, client::Client, error::Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UsageCharge {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_on: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balance_used: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balance_remaining: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub risk_level: Option<f64>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct UsageChargeListParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<String>,
}

#[derive(Debug, Deserialize)]
struct UsageChargeWrapper { usage_charge: UsageCharge }
#[derive(Debug, Deserialize)]
struct UsageChargesWrapper { usage_charges: Vec<UsageCharge> }
#[derive(Debug, Serialize)]
struct UsageChargeRequest { usage_charge: UsageCharge }

impl UsageCharge {
    pub async fn find(client: &Client, recurring_charge_id: i64, id: i64) -> Result<Option<Self>> {
        let path = format!("recurring_application_charges/{}/usage_charges/{}.json", recurring_charge_id, id);
        let response = client.get::<UsageChargeWrapper>(&path).await?;
        Ok(Some(response.data.usage_charge))
    }

    pub async fn all(client: &Client, recurring_charge_id: i64, params: UsageChargeListParams) -> Result<FindAllResponse<Self>> {
        let path = format!("recurring_application_charges/{}/usage_charges.json", recurring_charge_id);
        let response = client.get_with_params::<UsageChargesWrapper, _>(&path, &params).await?;
        Ok(FindAllResponse {
            data: response.data.usage_charges,
            next_page: response.page_info.as_ref().and_then(|p| p.next.clone()),
            previous_page: response.page_info.as_ref().and_then(|p| p.previous.clone()),
        })
    }

    pub async fn create(client: &Client, recurring_charge_id: i64, charge: &Self) -> Result<Self> {
        let path = format!("recurring_application_charges/{}/usage_charges.json", recurring_charge_id);
        let request = UsageChargeRequest { usage_charge: charge.clone() };
        let response = client.post::<UsageChargeWrapper, _>(&path, &request).await?;
        Ok(response.data.usage_charge)
    }
}
