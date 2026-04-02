//! Recurring Application Charge resource (subscriptions)

use crate::{base::FindAllResponse, client::Client, error::Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RecurringApplicationCharge {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_client_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_on: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activated_on: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancelled_on: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_days: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_ends_on: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capped_amount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balance_used: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balance_remaining: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub risk_level: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confirmation_url: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct RecurringChargeListParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub since_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<String>,
}

#[derive(Debug, Deserialize)]
struct RecurringChargeWrapper { recurring_application_charge: RecurringApplicationCharge }
#[derive(Debug, Deserialize)]
struct RecurringChargesWrapper { recurring_application_charges: Vec<RecurringApplicationCharge> }
#[derive(Debug, Serialize)]
struct RecurringChargeRequest { recurring_application_charge: RecurringApplicationCharge }

impl RecurringApplicationCharge {
    pub async fn find(client: &Client, id: i64) -> Result<Option<Self>> {
        let path = format!("recurring_application_charges/{}.json", id);
        let response = client.get::<RecurringChargeWrapper>(&path).await?;
        Ok(Some(response.data.recurring_application_charge))
    }

    pub async fn all(client: &Client, params: RecurringChargeListParams) -> Result<FindAllResponse<Self>> {
        let response = client.get_with_params::<RecurringChargesWrapper, _>("recurring_application_charges.json", &params).await?;
        Ok(FindAllResponse {
            data: response.data.recurring_application_charges,
            next_page: response.page_info.as_ref().and_then(|p| p.next.clone()),
            previous_page: response.page_info.as_ref().and_then(|p| p.previous.clone()),
        })
    }

    pub async fn create(client: &Client, charge: &Self) -> Result<Self> {
        let request = RecurringChargeRequest { recurring_application_charge: charge.clone() };
        let response = client.post::<RecurringChargeWrapper, _>("recurring_application_charges.json", &request).await?;
        Ok(response.data.recurring_application_charge)
    }

    pub async fn delete(client: &Client, id: i64) -> Result<()> {
        let path = format!("recurring_application_charges/{}.json", id);
        client.delete(&path).await
    }

    pub async fn customize(client: &Client, id: i64, capped_amount: &str) -> Result<Self> {
        let path = format!("recurring_application_charges/{}/customize.json?recurring_application_charge[capped_amount]={}", id, capped_amount);
        let response = client.put::<RecurringChargeWrapper, _>(&path, &serde_json::json!({})).await?;
        Ok(response.data.recurring_application_charge)
    }
}
