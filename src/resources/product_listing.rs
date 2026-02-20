//! Product Listing resource

use crate::{base::FindAllResponse, client::Client, error::Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProductListing {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body_html: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub handle: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vendor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub available: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub published_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variants: Option<Vec<serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub images: Option<Vec<serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<serde_json::Value>>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct ProductListingParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_ids: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at_min: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub handle: Option<String>,
}

#[derive(Debug, Deserialize)]
struct ProductListingWrapper { product_listing: ProductListing }
#[derive(Debug, Deserialize)]
struct ProductListingsWrapper { product_listings: Vec<ProductListing> }

impl ProductListing {
    pub async fn find(client: &Client, product_id: i64) -> Result<Option<Self>> {
        let path = format!("product_listings/{}.json", product_id);
        let response = client.get::<ProductListingWrapper>(&path).await?;
        Ok(Some(response.data.product_listing))
    }

    pub async fn all(client: &Client, params: ProductListingParams) -> Result<FindAllResponse<Self>> {
        let response = client.get_with_params::<ProductListingsWrapper, _>("product_listings.json", &params).await?;
        Ok(FindAllResponse {
            data: response.data.product_listings,
            next_page: response.page_info.as_ref().and_then(|p| p.next.clone()),
            previous_page: response.page_info.as_ref().and_then(|p| p.previous.clone()),
        })
    }

    pub async fn count(client: &Client) -> Result<i64> {
        #[derive(Deserialize)]
        struct CountWrapper { count: i64 }
        let response = client.get::<CountWrapper>("product_listings/count.json").await?;
        Ok(response.data.count)
    }

    pub async fn product_ids(client: &Client) -> Result<Vec<i64>> {
        #[derive(Deserialize)]
        struct ProductIdsWrapper { product_ids: Vec<i64> }
        let response = client.get::<ProductIdsWrapper>("product_listings/product_ids.json").await?;
        Ok(response.data.product_ids)
    }

    pub async fn create(client: &Client, product_id: i64) -> Result<Self> {
        let path = format!("product_listings/{}.json", product_id);
        let body = serde_json::json!({"product_listing": {"product_id": product_id}});
        let response = client.put::<ProductListingWrapper, _>(&path, &body).await?;
        Ok(response.data.product_listing)
    }

    pub async fn delete(client: &Client, product_id: i64) -> Result<()> {
        let path = format!("product_listings/{}.json", product_id);
        client.delete(&path).await
    }
}
