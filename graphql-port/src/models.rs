//! Strongly-typed GraphQL domain models.
//!
//! These structs map to the corresponding Shopify Admin GraphQL API types.
//! Use them as type parameters to `Client::graphql_data<YourResponseType, _>(...)`.

use serde::{Deserialize, Serialize};

// ── Money ─────────────────────────────────────────────────────────────────────

/// A monetary value with an ISO 4217 currency code.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Money {
    /// Decimal string representation (e.g. `"10.00"`).
    pub amount: String,
    /// ISO 4217 currency code (e.g. `"USD"`).
    #[serde(rename = "currencyCode")]
    pub currency_code: String,
}

/// A pair of presentment + shop money values.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MoneyBag {
    #[serde(rename = "shopMoney")]
    pub shop_money: Money,
    #[serde(rename = "presentmentMoney")]
    pub presentment_money: Option<Money>,
}

// ── Image ─────────────────────────────────────────────────────────────────────

/// A Shopify-hosted image.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Image {
    /// Publicly accessible URL.
    pub url: String,
    /// Alt text for accessibility.
    pub alt_text: Option<String>,
    /// Width in pixels.
    pub width: Option<i32>,
    /// Height in pixels.
    pub height: Option<i32>,
}

// ── Product ───────────────────────────────────────────────────────────────────

/// Shopify Admin GraphQL `ProductVariant` node.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ProductVariant {
    /// Global ID (e.g. `"gid://shopify/ProductVariant/123"`).
    pub id: String,
    pub title: Option<String>,
    pub sku: Option<String>,
    pub price: Option<String>,
    #[serde(rename = "compareAtPrice")]
    pub compare_at_price: Option<String>,
    #[serde(rename = "inventoryQuantity")]
    pub inventory_quantity: Option<i32>,
    #[serde(rename = "requiresShipping")]
    pub requires_shipping: Option<bool>,
    pub weight: Option<f64>,
    #[serde(rename = "weightUnit")]
    pub weight_unit: Option<String>,
    #[serde(rename = "featuredImage")]
    pub featured_image: Option<Image>,
}

/// Shopify Admin GraphQL `Product` node.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Product {
    /// Global ID (e.g. `"gid://shopify/Product/123"`).
    pub id: String,
    pub title: String,
    pub handle: Option<String>,
    pub vendor: Option<String>,
    #[serde(rename = "productType")]
    pub product_type: Option<String>,
    pub status: Option<String>,
    pub tags: Option<Vec<String>>,
    pub description: Option<String>,
    #[serde(rename = "descriptionHtml")]
    pub description_html: Option<String>,
    #[serde(rename = "createdAt")]
    pub created_at: Option<String>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<String>,
    #[serde(rename = "publishedAt")]
    pub published_at: Option<String>,
    #[serde(rename = "featuredImage")]
    pub featured_image: Option<Image>,
}

// ── Customer ──────────────────────────────────────────────────────────────────

/// Shopify Admin GraphQL `MailingAddress` type.
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct MailingAddress {
    pub address1: Option<String>,
    pub address2: Option<String>,
    pub city: Option<String>,
    pub province: Option<String>,
    pub country: Option<String>,
    pub zip: Option<String>,
    pub phone: Option<String>,
    pub name: Option<String>,
    #[serde(rename = "countryCodeV2")]
    pub country_code: Option<String>,
}

/// Shopify Admin GraphQL `Customer` node.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Customer {
    pub id: String,
    #[serde(rename = "firstName")]
    pub first_name: Option<String>,
    #[serde(rename = "lastName")]
    pub last_name: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    #[serde(rename = "acceptsMarketing")]
    pub accepts_marketing: Option<bool>,
    pub tags: Option<Vec<String>>,
    pub note: Option<String>,
    pub state: Option<String>,
    #[serde(rename = "taxExempt")]
    pub tax_exempt: Option<bool>,
    #[serde(rename = "verifiedEmail")]
    pub verified_email: Option<bool>,
    #[serde(rename = "createdAt")]
    pub created_at: Option<String>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<String>,
    #[serde(rename = "defaultAddress")]
    pub default_address: Option<MailingAddress>,
}

// ── Order ─────────────────────────────────────────────────────────────────────

/// Shopify Admin GraphQL `LineItem` node.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LineItem {
    pub id: String,
    pub title: String,
    pub quantity: i32,
    #[serde(rename = "originalTotalSet")]
    pub original_total_set: Option<MoneyBag>,
    #[serde(rename = "variantTitle")]
    pub variant_title: Option<String>,
    pub sku: Option<String>,
    pub vendor: Option<String>,
}

/// Shopify Admin GraphQL `Order` node.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Order {
    pub id: String,
    /// e.g. `"#1001"`
    pub name: String,
    pub email: Option<String>,
    pub phone: Option<String>,
    #[serde(rename = "financialStatus")]
    pub financial_status: Option<String>,
    #[serde(rename = "fulfillmentStatus")]
    pub fulfillment_status: Option<String>,
    #[serde(rename = "totalPriceSet")]
    pub total_price_set: Option<MoneyBag>,
    #[serde(rename = "subtotalPriceSet")]
    pub subtotal_price_set: Option<MoneyBag>,
    #[serde(rename = "totalTaxSet")]
    pub total_tax_set: Option<MoneyBag>,
    pub tags: Option<Vec<String>>,
    pub note: Option<String>,
    #[serde(rename = "createdAt")]
    pub created_at: Option<String>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<String>,
    #[serde(rename = "processedAt")]
    pub processed_at: Option<String>,
    pub customer: Option<Customer>,
    #[serde(rename = "shippingAddress")]
    pub shipping_address: Option<MailingAddress>,
}

// ── Fulfillment ───────────────────────────────────────────────────────────────

/// Shopify Admin GraphQL `Fulfillment` node.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Fulfillment {
    pub id: String,
    pub status: Option<String>,
    #[serde(rename = "createdAt")]
    pub created_at: Option<String>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<String>,
    #[serde(rename = "trackingInfo")]
    pub tracking_info: Option<Vec<TrackingInfo>>,
}

/// Tracking number + carrier info for a fulfillment.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TrackingInfo {
    pub number: Option<String>,
    pub url: Option<String>,
    pub company: Option<String>,
}

// ── Metafield ─────────────────────────────────────────────────────────────────

/// Shopify Admin GraphQL `Metafield` node.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Metafield {
    pub id: String,
    pub namespace: String,
    pub key: String,
    pub value: String,
    #[serde(rename = "type")]
    pub value_type: String,
    pub description: Option<String>,
    #[serde(rename = "createdAt")]
    pub created_at: Option<String>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<String>,
}
