//! Asset resource

use crate::{base::FindAllResponse, client::Client, error::Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Asset {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checksum: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub theme_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachment: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct AssetListParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<String>,
}

#[derive(Debug, Deserialize)]
struct AssetWrapper { asset: Asset }
#[derive(Debug, Deserialize)]
struct AssetsWrapper { assets: Vec<Asset> }
#[derive(Debug, Serialize)]
struct AssetRequest { asset: Asset }

impl Asset {
    pub async fn find(client: &Client, theme_id: i64, key: &str) -> Result<Option<Self>> {
        let path = format!("themes/{}/assets.json?asset[key]={}", theme_id, key);
        let response = client.get::<AssetWrapper>(&path).await?;
        Ok(Some(response.data.asset))
    }

    pub async fn all(client: &Client, theme_id: i64, params: AssetListParams) -> Result<FindAllResponse<Self>> {
        let path = format!("themes/{}/assets.json", theme_id);
        let response = client.get_with_params::<AssetsWrapper, _>(&path, &params).await?;
        Ok(FindAllResponse {
            data: response.data.assets,
            next_page: response.page_info.as_ref().and_then(|p| p.next.clone()),
            previous_page: response.page_info.as_ref().and_then(|p| p.previous.clone()),
        })
    }

    pub async fn create_or_update(client: &Client, theme_id: i64, asset: &Self) -> Result<Self> {
        let path = format!("themes/{}/assets.json", theme_id);
        let request = AssetRequest { asset: asset.clone() };
        let response = client.put::<AssetWrapper, _>(&path, &request).await?;
        Ok(response.data.asset)
    }

    pub async fn delete(client: &Client, theme_id: i64, key: &str) -> Result<()> {
        let path = format!("themes/{}/assets.json?asset[key]={}", theme_id, key);
        client.delete(&path).await
    }
}
