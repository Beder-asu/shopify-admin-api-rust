//! Gift Card Adjustment resource

use crate::{base::FindAllResponse, client::Client, error::Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GiftCardAdjustment {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gift_card_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_client_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_transaction_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_transaction_ref: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_transaction_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processed_at: Option<String>,
}

#[derive(Debug, Deserialize)]
struct GiftCardAdjustmentWrapper { adjustment: GiftCardAdjustment }
#[derive(Debug, Deserialize)]
struct GiftCardAdjustmentsWrapper { adjustments: Vec<GiftCardAdjustment> }
#[derive(Debug, Serialize)]
struct GiftCardAdjustmentRequest { adjustment: GiftCardAdjustment }

impl GiftCardAdjustment {
    pub async fn find(client: &Client, gift_card_id: i64, id: i64) -> Result<Option<Self>> {
        let path = format!("gift_cards/{}/adjustments/{}.json", gift_card_id, id);
        let response = client.get::<GiftCardAdjustmentWrapper>(&path).await?;
        Ok(Some(response.data.adjustment))
    }

    pub async fn all(client: &Client, gift_card_id: i64) -> Result<FindAllResponse<Self>> {
        let path = format!("gift_cards/{}/adjustments.json", gift_card_id);
        let response = client.get::<GiftCardAdjustmentsWrapper>(&path).await?;
        Ok(FindAllResponse {
            data: response.data.adjustments,
            next_page: response.page_info.as_ref().and_then(|p| p.next.clone()),
            previous_page: response.page_info.as_ref().and_then(|p| p.previous.clone()),
        })
    }

    pub async fn create(client: &Client, gift_card_id: i64, adjustment: &Self) -> Result<Self> {
        let path = format!("gift_cards/{}/adjustments.json", gift_card_id);
        let request = GiftCardAdjustmentRequest { adjustment: adjustment.clone() };
        let response = client.post::<GiftCardAdjustmentWrapper, _>(&path, &request).await?;
        Ok(response.data.adjustment)
    }
}
