//! Image resource

use crate::{base::FindAllResponse, client::Client, error::Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Image {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alt: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub src: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variant_ids: Option<Vec<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub admin_graphql_api_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct ImageListParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub since_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<String>,
}

#[derive(Debug, Deserialize)]
struct ImageWrapper { image: Image }
#[derive(Debug, Deserialize)]
struct ImagesWrapper { images: Vec<Image> }
#[derive(Debug, Serialize)]
struct ImageRequest { image: Image }

impl Image {
    pub async fn find(client: &Client, product_id: i64, id: i64) -> Result<Option<Self>> {
        let path = format!("products/{}/images/{}.json", product_id, id);
        let response = client.get::<ImageWrapper>(&path).await?;
        Ok(Some(response.data.image))
    }

    pub async fn all(client: &Client, product_id: i64, params: ImageListParams) -> Result<FindAllResponse<Self>> {
        let path = format!("products/{}/images.json", product_id);
        let response = client.get_with_params::<ImagesWrapper, _>(&path, &params).await?;
        Ok(FindAllResponse {
            data: response.data.images,
            next_page: response.page_info.as_ref().and_then(|p| p.next.clone()),
            previous_page: response.page_info.as_ref().and_then(|p| p.previous.clone()),
        })
    }

    pub async fn create(client: &Client, product_id: i64, image: &Self) -> Result<Self> {
        let path = format!("products/{}/images.json", product_id);
        let request = ImageRequest { image: image.clone() };
        let response = client.post::<ImageWrapper, _>(&path, &request).await?;
        Ok(response.data.image)
    }

    pub async fn update(client: &Client, product_id: i64, image: &Self) -> Result<Self> {
        let id = image.id.ok_or_else(|| crate::ShopifyError::ValidationError("Image ID required".to_string()))?;
        let path = format!("products/{}/images/{}.json", product_id, id);
        let request = ImageRequest { image: image.clone() };
        let response = client.put::<ImageWrapper, _>(&path, &request).await?;
        Ok(response.data.image)
    }

    pub async fn delete(client: &Client, product_id: i64, id: i64) -> Result<()> {
        let path = format!("products/{}/images/{}.json", product_id, id);
        client.delete(&path).await
    }
}
