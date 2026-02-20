//! Draft Order resource

use crate::{base::{CountResponse, FindAllResponse}, client::Client, error::Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DraftOrder {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub taxes_included: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_sent_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_exempt: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_items: Option<Vec<DraftOrderLineItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_address: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_address: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub applied_discount: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_line: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_lines: Option<Vec<serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subtotal_price: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_tax: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_price: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DraftOrderLineItem {
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
    pub custom: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct DraftOrderListParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub since_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at_min: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at_max: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ids: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<String>,
}

#[derive(Debug, Deserialize)]
struct DraftOrderWrapper { draft_order: DraftOrder }
#[derive(Debug, Deserialize)]
struct DraftOrdersWrapper { draft_orders: Vec<DraftOrder> }
#[derive(Debug, Serialize)]
struct DraftOrderRequest { draft_order: DraftOrder }

impl DraftOrder {
    pub async fn find(client: &Client, id: i64) -> Result<Option<Self>> {
        let path = format!("draft_orders/{}.json", id);
        let response = client.get::<DraftOrderWrapper>(&path).await?;
        Ok(Some(response.data.draft_order))
    }

    pub async fn all(client: &Client, params: DraftOrderListParams) -> Result<FindAllResponse<Self>> {
        let response = client.get_with_params::<DraftOrdersWrapper, _>("draft_orders.json", &params).await?;
        Ok(FindAllResponse {
            data: response.data.draft_orders,
            next_page: response.page_info.as_ref().and_then(|p| p.next.clone()),
            previous_page: response.page_info.as_ref().and_then(|p| p.previous.clone()),
        })
    }

    pub async fn count(client: &Client) -> Result<i64> {
        let response = client.get::<CountResponse>("draft_orders/count.json").await?;
        Ok(response.data.count)
    }

    pub async fn create(client: &Client, draft_order: &Self) -> Result<Self> {
        let request = DraftOrderRequest { draft_order: draft_order.clone() };
        let response = client.post::<DraftOrderWrapper, _>("draft_orders.json", &request).await?;
        Ok(response.data.draft_order)
    }

    pub async fn update(client: &Client, draft_order: &Self) -> Result<Self> {
        let id = draft_order.id.ok_or_else(|| crate::ShopifyError::ValidationError("Draft Order ID required".to_string()))?;
        let path = format!("draft_orders/{}.json", id);
        let request = DraftOrderRequest { draft_order: draft_order.clone() };
        let response = client.put::<DraftOrderWrapper, _>(&path, &request).await?;
        Ok(response.data.draft_order)
    }

    pub async fn delete(client: &Client, id: i64) -> Result<()> {
        let path = format!("draft_orders/{}.json", id);
        client.delete(&path).await
    }

    pub async fn complete(client: &Client, id: i64, payment_pending: Option<bool>) -> Result<Self> {
        let path = format!("draft_orders/{}/complete.json", id);
        let body = payment_pending.map(|p| serde_json::json!({"payment_pending": p})).unwrap_or(serde_json::json!({}));
        let response = client.put::<DraftOrderWrapper, _>(&path, &body).await?;
        Ok(response.data.draft_order)
    }

    pub async fn send_invoice(client: &Client, id: i64, invoice: Option<serde_json::Value>) -> Result<serde_json::Value> {
        let path = format!("draft_orders/{}/send_invoice.json", id);
        let body = invoice.unwrap_or(serde_json::json!({}));
        let response = client.post::<serde_json::Value, _>(&path, &body).await?;
        Ok(response.data)
    }
}
