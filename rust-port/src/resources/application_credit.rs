//! Application Credit resource

use crate::{base::FindAllResponse, client::Client, error::Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ApplicationCredit {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct ApplicationCreditListParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<String>,
}

#[derive(Debug, Deserialize)]
struct ApplicationCreditWrapper { application_credit: ApplicationCredit }
#[derive(Debug, Deserialize)]
struct ApplicationCreditsWrapper { application_credits: Vec<ApplicationCredit> }
#[derive(Debug, Serialize)]
struct ApplicationCreditRequest { application_credit: ApplicationCredit }

impl ApplicationCredit {
    pub async fn find(client: &Client, id: i64) -> Result<Option<Self>> {
        let path = format!("application_credits/{}.json", id);
        let response = client.get::<ApplicationCreditWrapper>(&path).await?;
        Ok(Some(response.data.application_credit))
    }

    pub async fn all(client: &Client, params: ApplicationCreditListParams) -> Result<FindAllResponse<Self>> {
        let response = client.get_with_params::<ApplicationCreditsWrapper, _>("application_credits.json", &params).await?;
        Ok(FindAllResponse {
            data: response.data.application_credits,
            next_page: response.page_info.as_ref().and_then(|p| p.next.clone()),
            previous_page: response.page_info.as_ref().and_then(|p| p.previous.clone()),
        })
    }

    pub async fn create(client: &Client, credit: &Self) -> Result<Self> {
        let request = ApplicationCreditRequest { application_credit: credit.clone() };
        let response = client.post::<ApplicationCreditWrapper, _>("application_credits.json", &request).await?;
        Ok(response.data.application_credit)
    }
}
