//! Refund resource

use crate::{base::FindAllResponse, client::Client, error::Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Refund {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processed_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restock: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duties: Option<Vec<serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund_line_items: Option<Vec<RefundLineItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transactions: Option<Vec<serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_adjustments: Option<Vec<serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping: Option<RefundShipping>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notify: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RefundLineItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_item_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restock_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subtotal: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_tax: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RefundShipping {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub full_refund: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct RefundListParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<String>,
}

#[derive(Debug, Deserialize)]
struct RefundWrapper { refund: Refund }
#[derive(Debug, Deserialize)]
struct RefundsWrapper { refunds: Vec<Refund> }
#[derive(Debug, Serialize)]
struct RefundRequest { refund: Refund }

impl Refund {
    pub async fn find(client: &Client, order_id: i64, id: i64) -> Result<Option<Self>> {
        let path = format!("orders/{}/refunds/{}.json", order_id, id);
        let response = client.get::<RefundWrapper>(&path).await?;
        Ok(Some(response.data.refund))
    }

    pub async fn all(client: &Client, order_id: i64, params: RefundListParams) -> Result<FindAllResponse<Self>> {
        let path = format!("orders/{}/refunds.json", order_id);
        let response = client.get_with_params::<RefundsWrapper, _>(&path, &params).await?;
        Ok(FindAllResponse {
            data: response.data.refunds,
            next_page: response.page_info.as_ref().and_then(|p| p.next.clone()),
            previous_page: response.page_info.as_ref().and_then(|p| p.previous.clone()),
        })
    }

    pub async fn create(client: &Client, order_id: i64, refund: &Self) -> Result<Self> {
        let path = format!("orders/{}/refunds.json", order_id);
        let request = RefundRequest { refund: refund.clone() };
        let response = client.post::<RefundWrapper, _>(&path, &request).await?;
        Ok(response.data.refund)
    }

    pub async fn calculate(client: &Client, order_id: i64, refund: &Self) -> Result<Refund> {
        let path = format!("orders/{}/refunds/calculate.json", order_id);
        let request = RefundRequest { refund: refund.clone() };
        let response = client.post::<RefundWrapper, _>(&path, &request).await?;
        Ok(response.data.refund)
    }
}
