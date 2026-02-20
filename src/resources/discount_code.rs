//! Discount Code resource

use crate::{base::FindAllResponse, client::Client, error::Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DiscountCode {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_rule_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage_count: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct DiscountCodeListParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_id: Option<i64>,
}

#[derive(Debug, Deserialize)]
struct DiscountCodeWrapper { discount_code: DiscountCode }
#[derive(Debug, Deserialize)]
struct DiscountCodesWrapper { discount_codes: Vec<DiscountCode> }
#[derive(Debug, Serialize)]
struct DiscountCodeRequest { discount_code: DiscountCode }

impl DiscountCode {
    pub async fn find(client: &Client, price_rule_id: i64, id: i64) -> Result<Option<Self>> {
        let path = format!("price_rules/{}/discount_codes/{}.json", price_rule_id, id);
        let response = client.get::<DiscountCodeWrapper>(&path).await?;
        Ok(Some(response.data.discount_code))
    }

    pub async fn lookup(client: &Client, code: &str) -> Result<Option<Self>> {
        let path = format!("discount_codes/lookup.json?code={}", code);
        let response = client.get::<DiscountCodeWrapper>(&path).await?;
        Ok(Some(response.data.discount_code))
    }

    pub async fn all(client: &Client, price_rule_id: i64, params: DiscountCodeListParams) -> Result<FindAllResponse<Self>> {
        let path = format!("price_rules/{}/discount_codes.json", price_rule_id);
        let response = client.get_with_params::<DiscountCodesWrapper, _>(&path, &params).await?;
        Ok(FindAllResponse {
            data: response.data.discount_codes,
            next_page: response.page_info.as_ref().and_then(|p| p.next.clone()),
            previous_page: response.page_info.as_ref().and_then(|p| p.previous.clone()),
        })
    }

    pub async fn count(client: &Client, price_rule_id: i64) -> Result<i64> {
        let path = format!("price_rules/{}/discount_codes/count.json", price_rule_id);
        #[derive(Deserialize)]
        struct CountWrapper { count: i64 }
        let response = client.get::<CountWrapper>(&path).await?;
        Ok(response.data.count)
    }

    pub async fn create(client: &Client, price_rule_id: i64, discount_code: &Self) -> Result<Self> {
        let path = format!("price_rules/{}/discount_codes.json", price_rule_id);
        let request = DiscountCodeRequest { discount_code: discount_code.clone() };
        let response = client.post::<DiscountCodeWrapper, _>(&path, &request).await?;
        Ok(response.data.discount_code)
    }

    pub async fn update(client: &Client, price_rule_id: i64, discount_code: &Self) -> Result<Self> {
        let id = discount_code.id.ok_or_else(|| crate::ShopifyError::ValidationError("Discount Code ID required".to_string()))?;
        let path = format!("price_rules/{}/discount_codes/{}.json", price_rule_id, id);
        let request = DiscountCodeRequest { discount_code: discount_code.clone() };
        let response = client.put::<DiscountCodeWrapper, _>(&path, &request).await?;
        Ok(response.data.discount_code)
    }

    pub async fn delete(client: &Client, price_rule_id: i64, id: i64) -> Result<()> {
        let path = format!("price_rules/{}/discount_codes/{}.json", price_rule_id, id);
        client.delete(&path).await
    }

    pub async fn batch_create(client: &Client, price_rule_id: i64, codes: &[Self]) -> Result<serde_json::Value> {
        let path = format!("price_rules/{}/batch.json", price_rule_id);
        let body = serde_json::json!({"discount_codes": codes});
        let response = client.post::<serde_json::Value, _>(&path, &body).await?;
        Ok(response.data)
    }

    pub async fn get_batch(client: &Client, price_rule_id: i64, batch_id: i64) -> Result<serde_json::Value> {
        let path = format!("price_rules/{}/batch/{}.json", price_rule_id, batch_id);
        let response = client.get::<serde_json::Value>(&path).await?;
        Ok(response.data)
    }
}
