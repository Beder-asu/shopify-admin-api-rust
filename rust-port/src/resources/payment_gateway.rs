//! Payment Gateway resource

use crate::{base::FindAllResponse, client::Client, error::Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PaymentGateway {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_network_tokenization: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test_mode: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sandbox: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processing_method: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachment: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currencies: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credential1: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credential2: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credential3: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credential4: Option<String>,
}

#[derive(Debug, Deserialize)]
struct PaymentGatewaysWrapper { payment_gateways: Vec<PaymentGateway> }

impl PaymentGateway {
    pub async fn all(client: &Client) -> Result<FindAllResponse<Self>> {
        let response = client.get::<PaymentGatewaysWrapper>("payment_gateways.json").await?;
        Ok(FindAllResponse {
            data: response.data.payment_gateways,
            next_page: response.page_info.as_ref().and_then(|p| p.next.clone()),
            previous_page: response.page_info.as_ref().and_then(|p| p.previous.clone()),
        })
    }
}
