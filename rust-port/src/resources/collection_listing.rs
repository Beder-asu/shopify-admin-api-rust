//! Collection Listing resource

use crate::{base::FindAllResponse, client::Client, error::Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CollectionListing {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body_html: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_product_image: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub handle: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub published_at: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct CollectionListingParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
}

#[derive(Debug, Deserialize)]
struct CollectionListingWrapper { collection_listing: CollectionListing }
#[derive(Debug, Deserialize)]
struct CollectionListingsWrapper { collection_listings: Vec<CollectionListing> }

impl CollectionListing {
    pub async fn find(client: &Client, collection_id: i64) -> Result<Option<Self>> {
        let path = format!("collection_listings/{}.json", collection_id);
        let response = client.get::<CollectionListingWrapper>(&path).await?;
        Ok(Some(response.data.collection_listing))
    }

    pub async fn all(client: &Client, params: CollectionListingParams) -> Result<FindAllResponse<Self>> {
        let response = client.get_with_params::<CollectionListingsWrapper, _>("collection_listings.json", &params).await?;
        Ok(FindAllResponse {
            data: response.data.collection_listings,
            next_page: response.page_info.as_ref().and_then(|p| p.next.clone()),
            previous_page: response.page_info.as_ref().and_then(|p| p.previous.clone()),
        })
    }

    pub async fn product_ids(client: &Client, collection_id: i64) -> Result<Vec<i64>> {
        let path = format!("collection_listings/{}/product_ids.json", collection_id);
        #[derive(Deserialize)]
        struct ProductIdsWrapper { product_ids: Vec<i64> }
        let response = client.get::<ProductIdsWrapper>(&path).await?;
        Ok(response.data.product_ids)
    }

    pub async fn create(client: &Client, collection_id: i64) -> Result<Self> {
        let path = format!("collection_listings/{}.json", collection_id);
        let body = serde_json::json!({"collection_listing": {"collection_id": collection_id}});
        let response = client.put::<CollectionListingWrapper, _>(&path, &body).await?;
        Ok(response.data.collection_listing)
    }

    pub async fn delete(client: &Client, collection_id: i64) -> Result<()> {
        let path = format!("collection_listings/{}.json", collection_id);
        client.delete(&path).await
    }
}
