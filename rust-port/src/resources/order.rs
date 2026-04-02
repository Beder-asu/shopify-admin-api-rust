//! Order resource
//! 
//! Represents an order in the Shopify store.

use crate::{
    base::{CountResponse, FindAllResponse},
    client::Client,
    error::Result,
};
use serde::{Deserialize, Serialize};

use super::customer::Customer;

/// Order resource
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Order {
    /// Unique identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// Admin URL for the order
    #[serde(skip_serializing_if = "Option::is_none")]
    pub admin_graphql_api_id: Option<String>,
    /// App that created the order
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id: Option<i64>,
    /// Billing address
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_address: Option<Address>,
    /// Browser IP
    #[serde(skip_serializing_if = "Option::is_none")]
    pub browser_ip: Option<String>,
    /// Whether buyer accepted marketing
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buyer_accepts_marketing: Option<bool>,
    /// Reason for cancellation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancel_reason: Option<String>,
    /// When order was cancelled
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancelled_at: Option<String>,
    /// Cart token
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cart_token: Option<String>,
    /// Checkout ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checkout_id: Option<i64>,
    /// Checkout token
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checkout_token: Option<String>,
    /// When order was closed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub closed_at: Option<String>,
    /// Whether order is confirmed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confirmed: Option<bool>,
    /// When order was created
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// Currency code
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    /// Current subtotal price
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_subtotal_price: Option<String>,
    /// Current total discounts
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_total_discounts: Option<String>,
    /// Current total price
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_total_price: Option<String>,
    /// Current total tax
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_total_tax: Option<String>,
    /// Associated customer
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<Customer>,
    /// Customer locale
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_locale: Option<String>,
    /// Discount applications
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discount_applications: Option<Vec<serde_json::Value>>,
    /// Discount codes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discount_codes: Option<Vec<DiscountCode>>,
    /// Email address
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// Whether taxes are estimated
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_taxes: Option<bool>,
    /// Financial status
    #[serde(skip_serializing_if = "Option::is_none")]
    pub financial_status: Option<String>,
    /// Fulfillments
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fulfillments: Option<Vec<serde_json::Value>>,
    /// Fulfillment status
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fulfillment_status: Option<String>,
    /// Gateway
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway: Option<String>,
    /// Landing site
    #[serde(skip_serializing_if = "Option::is_none")]
    pub landing_site: Option<String>,
    /// Line items
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_items: Option<Vec<LineItem>>,
    /// Location ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_id: Option<i64>,
    /// Order name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Note
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    /// Note attributes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note_attributes: Option<Vec<NoteAttribute>>,
    /// Order number
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number: Option<i64>,
    /// Order number (formatted)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_number: Option<i64>,
    /// Order status URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_status_url: Option<String>,
    /// Payment gateway names
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_gateway_names: Option<Vec<String>>,
    /// Phone
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    /// Presentment currency
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presentment_currency: Option<String>,
    /// When order was processed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processed_at: Option<String>,
    /// Processing method
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processing_method: Option<String>,
    /// Referring site
    #[serde(skip_serializing_if = "Option::is_none")]
    pub referring_site: Option<String>,
    /// Refunds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refunds: Option<Vec<serde_json::Value>>,
    /// Shipping address
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_address: Option<Address>,
    /// Shipping lines
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_lines: Option<Vec<ShippingLine>>,
    /// Source name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_name: Option<String>,
    /// Subtotal price
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subtotal_price: Option<String>,
    /// Tags
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<String>,
    /// Tax lines
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_lines: Option<Vec<TaxLine>>,
    /// Whether taxes are included
    #[serde(skip_serializing_if = "Option::is_none")]
    pub taxes_included: Option<bool>,
    /// Test order
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test: Option<bool>,
    /// Token
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
    /// Total discounts
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_discounts: Option<String>,
    /// Total line items price
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_line_items_price: Option<String>,
    /// Total price
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_price: Option<String>,
    /// Total shipping price
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_shipping_price_set: Option<PriceSet>,
    /// Total tax
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_tax: Option<String>,
    /// Total tip received
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_tip_received: Option<String>,
    /// Total weight
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_weight: Option<i64>,
    /// When order was updated
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    /// User ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
}

/// Address structure
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Address {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address1: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address2: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub province: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zip: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub province_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latitude: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub longitude: Option<f64>,
}

/// Line item
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LineItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variant_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variant_title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sku: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vendor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grams: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requires_shipping: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub taxable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gift_card: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fulfillment_status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_discount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fulfillable_quantity: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fulfillment_service: Option<String>,
}

/// Discount code
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DiscountCode {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

/// Note attribute
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NoteAttribute {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// Shipping line
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ShippingLine {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub carrier_identifier: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested_fulfillment_service_id: Option<String>,
}

/// Tax line
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TaxLine {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate: Option<f64>,
}

/// Price set (shop and presentment money)
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PriceSet {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shop_money: Option<Money>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presentment_money: Option<Money>,
}

/// Money
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Money {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_code: Option<String>,
}

/// Parameters for listing orders
#[derive(Debug, Clone, Default, Serialize)]
pub struct OrderListParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ids: Option<String>,
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
    pub processed_at_min: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processed_at_max: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribution_app_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub financial_status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fulfillment_status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<String>,
}

/// Parameters for counting orders
#[derive(Debug, Clone, Default, Serialize)]
pub struct OrderCountParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at_min: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at_max: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at_min: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at_max: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub financial_status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fulfillment_status: Option<String>,
}

/// Parameters for cancelling an order
#[derive(Debug, Clone, Default, Serialize)]
pub struct OrderCancelParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restock: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund: Option<serde_json::Value>,
}

/// Wrapper for API responses
#[derive(Debug, Deserialize)]
struct OrderWrapper {
    order: Order,
}

#[derive(Debug, Deserialize)]
struct OrdersWrapper {
    orders: Vec<Order>,
}

#[derive(Debug, Serialize)]
struct OrderRequest {
    order: Order,
}

impl Order {
    /// Find an order by ID
    pub async fn find(client: &Client, id: i64) -> Result<Option<Self>> {
        let path = format!("orders/{}.json", id);
        let response = client.get::<OrderWrapper>(&path).await?;
        Ok(Some(response.data.order))
    }

    /// Find an order by ID with specific fields
    pub async fn find_with_fields(client: &Client, id: i64, fields: &str) -> Result<Option<Self>> {
        let path = format!("orders/{}.json?fields={}", id, fields);
        let response = client.get::<OrderWrapper>(&path).await?;
        Ok(Some(response.data.order))
    }

    /// List all orders
    pub async fn all(client: &Client, params: OrderListParams) -> Result<FindAllResponse<Self>> {
        let response = client
            .get_with_params::<OrdersWrapper, _>("orders.json", &params)
            .await?;
        
        Ok(FindAllResponse {
            data: response.data.orders,
            next_page: response.page_info.as_ref().and_then(|p| p.next.clone()),
            previous_page: response.page_info.as_ref().and_then(|p| p.previous.clone()),
        })
    }

    /// Count orders
    pub async fn count(client: &Client, params: OrderCountParams) -> Result<i64> {
        let response = client
            .get_with_params::<CountResponse, _>("orders/count.json", &params)
            .await?;
        Ok(response.data.count)
    }

    /// Create a new order
    pub async fn create(client: &Client, order: &Self) -> Result<Self> {
        let request = OrderRequest {
            order: order.clone(),
        };
        let response = client
            .post::<OrderWrapper, _>("orders.json", &request)
            .await?;
        Ok(response.data.order)
    }

    /// Update an existing order
    pub async fn update(client: &Client, order: &Self) -> Result<Self> {
        let id = order.id.ok_or_else(|| {
            crate::ShopifyError::ValidationError("Order ID is required for update".to_string())
        })?;
        let path = format!("orders/{}.json", id);
        let request = OrderRequest {
            order: order.clone(),
        };
        let response = client.put::<OrderWrapper, _>(&path, &request).await?;
        Ok(response.data.order)
    }

    /// Delete an order
    pub async fn delete(client: &Client, id: i64) -> Result<()> {
        let path = format!("orders/{}.json", id);
        client.delete(&path).await
    }

    /// Cancel an order
    pub async fn cancel(client: &Client, id: i64, params: OrderCancelParams) -> Result<Self> {
        let path = format!("orders/{}/cancel.json", id);
        let response = client.post::<OrderWrapper, _>(&path, &params).await?;
        Ok(response.data.order)
    }

    /// Close an order
    pub async fn close(client: &Client, id: i64) -> Result<Self> {
        let path = format!("orders/{}/close.json", id);
        let response = client
            .post::<OrderWrapper, _>(&path, &serde_json::json!({}))
            .await?;
        Ok(response.data.order)
    }

    /// Re-open a closed order
    pub async fn open(client: &Client, id: i64) -> Result<Self> {
        let path = format!("orders/{}/open.json", id);
        let response = client
            .post::<OrderWrapper, _>(&path, &serde_json::json!({}))
            .await?;
        Ok(response.data.order)
    }
}
