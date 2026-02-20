//! Price Rule resource

use crate::{base::FindAllResponse, client::Client, error::Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PriceRule {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_selection: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_selection: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allocation_method: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allocation_limit: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub once_per_customer: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage_limit: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starts_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ends_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entitled_product_ids: Option<Vec<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entitled_variant_ids: Option<Vec<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entitled_collection_ids: Option<Vec<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entitled_country_ids: Option<Vec<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prerequisite_product_ids: Option<Vec<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prerequisite_variant_ids: Option<Vec<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prerequisite_collection_ids: Option<Vec<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prerequisite_customer_ids: Option<Vec<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prerequisite_subtotal_range: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prerequisite_quantity_range: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prerequisite_shipping_price_range: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prerequisite_to_entitlement_quantity_ratio: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prerequisite_to_entitlement_purchase: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub admin_graphql_api_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct PriceRuleListParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub since_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at_min: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at_max: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at_min: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at_max: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starts_at_min: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starts_at_max: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ends_at_min: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ends_at_max: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub times_used: Option<i32>,
}

#[derive(Debug, Deserialize)]
struct PriceRuleWrapper { price_rule: PriceRule }
#[derive(Debug, Deserialize)]
struct PriceRulesWrapper { price_rules: Vec<PriceRule> }
#[derive(Debug, Serialize)]
struct PriceRuleRequest { price_rule: PriceRule }

impl PriceRule {
    pub async fn find(client: &Client, id: i64) -> Result<Option<Self>> {
        let path = format!("price_rules/{}.json", id);
        let response = client.get::<PriceRuleWrapper>(&path).await?;
        Ok(Some(response.data.price_rule))
    }

    pub async fn all(client: &Client, params: PriceRuleListParams) -> Result<FindAllResponse<Self>> {
        let response = client.get_with_params::<PriceRulesWrapper, _>("price_rules.json", &params).await?;
        Ok(FindAllResponse {
            data: response.data.price_rules,
            next_page: response.page_info.as_ref().and_then(|p| p.next.clone()),
            previous_page: response.page_info.as_ref().and_then(|p| p.previous.clone()),
        })
    }

    pub async fn count(client: &Client) -> Result<i64> {
        #[derive(Deserialize)]
        struct CountWrapper { count: i64 }
        let response = client.get::<CountWrapper>("price_rules/count.json").await?;
        Ok(response.data.count)
    }

    pub async fn create(client: &Client, rule: &Self) -> Result<Self> {
        let request = PriceRuleRequest { price_rule: rule.clone() };
        let response = client.post::<PriceRuleWrapper, _>("price_rules.json", &request).await?;
        Ok(response.data.price_rule)
    }

    pub async fn update(client: &Client, rule: &Self) -> Result<Self> {
        let id = rule.id.ok_or_else(|| crate::ShopifyError::ValidationError("Price Rule ID required".to_string()))?;
        let path = format!("price_rules/{}.json", id);
        let request = PriceRuleRequest { price_rule: rule.clone() };
        let response = client.put::<PriceRuleWrapper, _>(&path, &request).await?;
        Ok(response.data.price_rule)
    }

    pub async fn delete(client: &Client, id: i64) -> Result<()> {
        let path = format!("price_rules/{}.json", id);
        client.delete(&path).await
    }
}
