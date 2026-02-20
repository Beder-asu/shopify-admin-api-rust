//! Order Risk resource

use crate::{base::FindAllResponse, client::Client, error::Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct OrderRisk {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checkout_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommendation: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cause_cancel: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merchant_message: Option<String>,
}

#[derive(Debug, Deserialize)]
struct OrderRiskWrapper { risk: OrderRisk }
#[derive(Debug, Deserialize)]
struct OrderRisksWrapper { risks: Vec<OrderRisk> }
#[derive(Debug, Serialize)]
struct OrderRiskRequest { risk: OrderRisk }

impl OrderRisk {
    pub async fn find(client: &Client, order_id: i64, id: i64) -> Result<Option<Self>> {
        let path = format!("orders/{}/risks/{}.json", order_id, id);
        let response = client.get::<OrderRiskWrapper>(&path).await?;
        Ok(Some(response.data.risk))
    }

    pub async fn all(client: &Client, order_id: i64) -> Result<FindAllResponse<Self>> {
        let path = format!("orders/{}/risks.json", order_id);
        let response = client.get::<OrderRisksWrapper>(&path).await?;
        Ok(FindAllResponse {
            data: response.data.risks,
            next_page: response.page_info.as_ref().and_then(|p| p.next.clone()),
            previous_page: response.page_info.as_ref().and_then(|p| p.previous.clone()),
        })
    }

    pub async fn create(client: &Client, order_id: i64, risk: &Self) -> Result<Self> {
        let path = format!("orders/{}/risks.json", order_id);
        let request = OrderRiskRequest { risk: risk.clone() };
        let response = client.post::<OrderRiskWrapper, _>(&path, &request).await?;
        Ok(response.data.risk)
    }

    pub async fn update(client: &Client, order_id: i64, risk: &Self) -> Result<Self> {
        let id = risk.id.ok_or_else(|| crate::ShopifyError::ValidationError("Risk ID required".to_string()))?;
        let path = format!("orders/{}/risks/{}.json", order_id, id);
        let request = OrderRiskRequest { risk: risk.clone() };
        let response = client.put::<OrderRiskWrapper, _>(&path, &request).await?;
        Ok(response.data.risk)
    }

    pub async fn delete(client: &Client, order_id: i64, id: i64) -> Result<()> {
        let path = format!("orders/{}/risks/{}.json", order_id, id);
        client.delete(&path).await
    }
}
