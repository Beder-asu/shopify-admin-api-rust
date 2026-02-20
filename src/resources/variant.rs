//! Variant resource

use crate::{
    base::{CountResponse, FindAllResponse},
    client::Client,
    error::Result,
};
use serde::{Deserialize, Serialize};

/// Product variant resource
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Variant {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sku: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inventory_policy: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compare_at_price: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fulfillment_service: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inventory_management: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub option1: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub option2: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub option3: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub taxable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub barcode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grams: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight_unit: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inventory_item_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inventory_quantity: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub old_inventory_quantity: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requires_shipping: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub admin_graphql_api_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct VariantListParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub since_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<String>,
}

#[derive(Debug, Deserialize)]
struct VariantWrapper { variant: Variant }
#[derive(Debug, Deserialize)]
struct VariantsWrapper { variants: Vec<Variant> }
#[derive(Debug, Serialize)]
struct VariantRequest { variant: Variant }

impl Variant {
    pub async fn find(client: &Client, id: i64) -> Result<Option<Self>> {
        let path = format!("variants/{}.json", id);
        let response = client.get::<VariantWrapper>(&path).await?;
        Ok(Some(response.data.variant))
    }

    pub async fn all_for_product(client: &Client, product_id: i64, params: VariantListParams) -> Result<FindAllResponse<Self>> {
        let path = format!("products/{}/variants.json", product_id);
        let response = client.get_with_params::<VariantsWrapper, _>(&path, &params).await?;
        Ok(FindAllResponse {
            data: response.data.variants,
            next_page: response.page_info.as_ref().and_then(|p| p.next.clone()),
            previous_page: response.page_info.as_ref().and_then(|p| p.previous.clone()),
        })
    }

    pub async fn count(client: &Client, product_id: i64) -> Result<i64> {
        let path = format!("products/{}/variants/count.json", product_id);
        let response = client.get::<CountResponse>(&path).await?;
        Ok(response.data.count)
    }

    pub async fn create(client: &Client, product_id: i64, variant: &Self) -> Result<Self> {
        let path = format!("products/{}/variants.json", product_id);
        let request = VariantRequest { variant: variant.clone() };
        let response = client.post::<VariantWrapper, _>(&path, &request).await?;
        Ok(response.data.variant)
    }

    pub async fn update(client: &Client, variant: &Self) -> Result<Self> {
        let id = variant.id.ok_or_else(|| crate::ShopifyError::ValidationError("Variant ID required".to_string()))?;
        let path = format!("variants/{}.json", id);
        let request = VariantRequest { variant: variant.clone() };
        let response = client.put::<VariantWrapper, _>(&path, &request).await?;
        Ok(response.data.variant)
    }

    pub async fn delete(client: &Client, product_id: i64, id: i64) -> Result<()> {
        let path = format!("products/{}/variants/{}.json", product_id, id);
        client.delete(&path).await
    }
}
