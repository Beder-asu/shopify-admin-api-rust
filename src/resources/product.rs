//! Product resource
//! 
//! Represents a product in the Shopify store.

use crate::{
    base::{CountResponse, FindAllResponse},
    client::Client,
    error::Result,
};
use serde::{Deserialize, Serialize};

use super::image::Image;
use super::variant::Variant;

/// Product resource
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Product {
    /// Unique identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// Product title
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Product description (HTML)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body_html: Option<String>,
    /// Vendor name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vendor: Option<String>,
    /// Product type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_type: Option<String>,
    /// When product was created
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// URL-friendly handle
    #[serde(skip_serializing_if = "Option::is_none")]
    pub handle: Option<String>,
    /// When product was last updated
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    /// When product was published
    #[serde(skip_serializing_if = "Option::is_none")]
    pub published_at: Option<String>,
    /// Template suffix
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_suffix: Option<String>,
    /// Publishing scope
    #[serde(skip_serializing_if = "Option::is_none")]
    pub published_scope: Option<String>,
    /// Tags (comma-separated)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<String>,
    /// Product status (active, archived, draft)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Admin GraphQL API ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub admin_graphql_api_id: Option<String>,
    /// Product variants
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variants: Option<Vec<Variant>>,
    /// Product options
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<ProductOption>>,
    /// Product images
    #[serde(skip_serializing_if = "Option::is_none")]
    pub images: Option<Vec<Image>>,
    /// Featured image
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<Image>,
}

/// Product option
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProductOption {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

/// Parameters for listing products
#[derive(Debug, Clone, Default, Serialize)]
pub struct ProductListParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ids: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub since_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vendor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub handle: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at_min: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at_max: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at_min: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at_max: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub published_at_min: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub published_at_max: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub published_status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presentment_currencies: Option<String>,
}

/// Parameters for counting products
#[derive(Debug, Clone, Default, Serialize)]
pub struct ProductCountParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vendor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at_min: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at_max: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at_min: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at_max: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub published_at_min: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub published_at_max: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub published_status: Option<String>,
}

/// Wrapper for API responses
#[derive(Debug, Deserialize)]
struct ProductWrapper {
    product: Product,
}

#[derive(Debug, Deserialize)]
struct ProductsWrapper {
    products: Vec<Product>,
}

#[derive(Debug, Serialize)]
struct ProductRequest {
    product: Product,
}

impl Product {
    /// Find a product by ID
    pub async fn find(client: &Client, id: i64) -> Result<Option<Self>> {
        let path = format!("products/{}.json", id);
        let response = client.get::<ProductWrapper>(&path).await?;
        Ok(Some(response.data.product))
    }

    /// Find a product by ID with specific fields
    pub async fn find_with_fields(client: &Client, id: i64, fields: &str) -> Result<Option<Self>> {
        let path = format!("products/{}.json?fields={}", id, fields);
        let response = client.get::<ProductWrapper>(&path).await?;
        Ok(Some(response.data.product))
    }

    /// List all products
    pub async fn all(client: &Client, params: ProductListParams) -> Result<FindAllResponse<Self>> {
        let response = client
            .get_with_params::<ProductsWrapper, _>("products.json", &params)
            .await?;
        
        Ok(FindAllResponse {
            data: response.data.products,
            next_page: response.page_info.as_ref().and_then(|p| p.next.clone()),
            previous_page: response.page_info.as_ref().and_then(|p| p.previous.clone()),
        })
    }

    /// Count products
    pub async fn count(client: &Client, params: ProductCountParams) -> Result<i64> {
        let response = client
            .get_with_params::<CountResponse, _>("products/count.json", &params)
            .await?;
        Ok(response.data.count)
    }

    /// Create a new product
    pub async fn create(client: &Client, product: &Self) -> Result<Self> {
        let request = ProductRequest {
            product: product.clone(),
        };
        let response = client
            .post::<ProductWrapper, _>("products.json", &request)
            .await?;
        Ok(response.data.product)
    }

    /// Update an existing product
    pub async fn update(client: &Client, product: &Self) -> Result<Self> {
        let id = product.id.ok_or_else(|| {
            crate::ShopifyError::ValidationError("Product ID is required for update".to_string())
        })?;
        let path = format!("products/{}.json", id);
        let request = ProductRequest {
            product: product.clone(),
        };
        let response = client.put::<ProductWrapper, _>(&path, &request).await?;
        Ok(response.data.product)
    }

    /// Save the product (create or update)
    pub async fn save(client: &Client, product: &Self) -> Result<Self> {
        if product.id.is_some() {
            Self::update(client, product).await
        } else {
            Self::create(client, product).await
        }
    }

    /// Delete a product
    pub async fn delete(client: &Client, id: i64) -> Result<()> {
        let path = format!("products/{}.json", id);
        client.delete(&path).await
    }
}
