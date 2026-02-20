//! Gift Card resource

use crate::{base::{CountResponse, FindAllResponse}, client::Client, error::Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GiftCard {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balance: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_item_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_client_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_on: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_suffix: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_characters: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct GiftCardListParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub since_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct GiftCardSearchParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<String>,
}

#[derive(Debug, Deserialize)]
struct GiftCardWrapper { gift_card: GiftCard }
#[derive(Debug, Deserialize)]
struct GiftCardsWrapper { gift_cards: Vec<GiftCard> }
#[derive(Debug, Serialize)]
struct GiftCardRequest { gift_card: GiftCard }

impl GiftCard {
    pub async fn find(client: &Client, id: i64) -> Result<Option<Self>> {
        let path = format!("gift_cards/{}.json", id);
        let response = client.get::<GiftCardWrapper>(&path).await?;
        Ok(Some(response.data.gift_card))
    }

    pub async fn all(client: &Client, params: GiftCardListParams) -> Result<FindAllResponse<Self>> {
        let response = client.get_with_params::<GiftCardsWrapper, _>("gift_cards.json", &params).await?;
        Ok(FindAllResponse {
            data: response.data.gift_cards,
            next_page: response.page_info.as_ref().and_then(|p| p.next.clone()),
            previous_page: response.page_info.as_ref().and_then(|p| p.previous.clone()),
        })
    }

    pub async fn count(client: &Client, status: Option<&str>) -> Result<i64> {
        let params = status.map(|s| [("status", s)]);
        let response = if let Some(p) = params {
            client.get_with_params::<CountResponse, _>("gift_cards/count.json", &p).await?
        } else {
            client.get::<CountResponse>("gift_cards/count.json").await?
        };
        Ok(response.data.count)
    }

    pub async fn search(client: &Client, params: GiftCardSearchParams) -> Result<Vec<Self>> {
        let response = client.get_with_params::<GiftCardsWrapper, _>("gift_cards/search.json", &params).await?;
        Ok(response.data.gift_cards)
    }

    pub async fn create(client: &Client, gift_card: &Self) -> Result<Self> {
        let request = GiftCardRequest { gift_card: gift_card.clone() };
        let response = client.post::<GiftCardWrapper, _>("gift_cards.json", &request).await?;
        Ok(response.data.gift_card)
    }

    pub async fn update(client: &Client, gift_card: &Self) -> Result<Self> {
        let id = gift_card.id.ok_or_else(|| crate::ShopifyError::ValidationError("Gift Card ID required".to_string()))?;
        let path = format!("gift_cards/{}.json", id);
        let request = GiftCardRequest { gift_card: gift_card.clone() };
        let response = client.put::<GiftCardWrapper, _>(&path, &request).await?;
        Ok(response.data.gift_card)
    }

    pub async fn disable(client: &Client, id: i64) -> Result<Self> {
        let path = format!("gift_cards/{}/disable.json", id);
        let response = client.post::<GiftCardWrapper, _>(&path, &serde_json::json!({})).await?;
        Ok(response.data.gift_card)
    }
}
