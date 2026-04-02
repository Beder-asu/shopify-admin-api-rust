//! Application Charge resource (one-time charges)

use crate::{base::FindAllResponse, client::Client, error::Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ApplicationCharge {
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
    pub test: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confirmation_url: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct ApplicationChargeListParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub since_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<String>,
}

#[derive(Debug, Deserialize)]
struct ApplicationChargeWrapper { application_charge: ApplicationCharge }
#[derive(Debug, Deserialize)]
struct ApplicationChargesWrapper { application_charges: Vec<ApplicationCharge> }
#[derive(Debug, Serialize)]
struct ApplicationChargeRequest { application_charge: ApplicationCharge }

impl ApplicationCharge {
    pub async fn find(client: &Client, id: i64) -> Result<Option<Self>> {
        let path = format!("application_charges/{}.json", id);
        let response = client.get::<ApplicationChargeWrapper>(&path).await?;
        Ok(Some(response.data.application_charge))
    }

    pub async fn all(client: &Client, params: ApplicationChargeListParams) -> Result<FindAllResponse<Self>> {
        let response = client.get_with_params::<ApplicationChargesWrapper, _>("application_charges.json", &params).await?;
        Ok(FindAllResponse {
            data: response.data.application_charges,
            next_page: response.page_info.as_ref().and_then(|p| p.next.clone()),
            previous_page: response.page_info.as_ref().and_then(|p| p.previous.clone()),
        })
    }

    pub async fn create(client: &Client, charge: &Self) -> Result<Self> {
        let request = ApplicationChargeRequest { application_charge: charge.clone() };
        let response = client.post::<ApplicationChargeWrapper, _>("application_charges.json", &request).await?;
        Ok(response.data.application_charge)
    }
}
