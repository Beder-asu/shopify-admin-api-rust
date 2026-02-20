//! Payment resource

use crate::{base::FindAllResponse, client::Client, error::Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Payment {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unique_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_processing_error_message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_action: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fraudulent: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credit_card: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checkout: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize)]
struct PaymentWrapper { payment: Payment }
#[derive(Debug, Deserialize)]
struct PaymentsWrapper { payments: Vec<Payment> }
#[derive(Debug, Serialize)]
struct PaymentRequest { payment: serde_json::Value }

impl Payment {
    pub async fn find(client: &Client, checkout_token: &str, id: i64) -> Result<Option<Self>> {
        let path = format!("checkouts/{}/payments/{}.json", checkout_token, id);
        let response = client.get::<PaymentWrapper>(&path).await?;
        Ok(Some(response.data.payment))
    }

    pub async fn all(client: &Client, checkout_token: &str) -> Result<FindAllResponse<Self>> {
        let path = format!("checkouts/{}/payments.json", checkout_token);
        let response = client.get::<PaymentsWrapper>(&path).await?;
        Ok(FindAllResponse {
            data: response.data.payments,
            next_page: response.page_info.as_ref().and_then(|p| p.next.clone()),
            previous_page: response.page_info.as_ref().and_then(|p| p.previous.clone()),
        })
    }

    pub async fn count(client: &Client, checkout_token: &str) -> Result<i64> {
        let path = format!("checkouts/{}/payments/count.json", checkout_token);
        #[derive(Deserialize)]
        struct CountWrapper { count: i64 }
        let response = client.get::<CountWrapper>(&path).await?;
        Ok(response.data.count)
    }

    pub async fn create(client: &Client, checkout_token: &str, payment: &serde_json::Value) -> Result<Self> {
        let path = format!("checkouts/{}/payments.json", checkout_token);
        let request = PaymentRequest { payment: payment.clone() };
        let response = client.post::<PaymentWrapper, _>(&path, &request).await?;
        Ok(response.data.payment)
    }
}
