//! Fulfillment Service resource

use crate::{base::FindAllResponse, client::Client, error::Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FulfillmentService {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub handle: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fulfillment_orders_opt_in: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_pending_stock: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracking_support: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inventory_management: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub admin_graphql_api_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct FulfillmentServiceListParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
}

#[derive(Debug, Deserialize)]
struct FulfillmentServiceWrapper { fulfillment_service: FulfillmentService }
#[derive(Debug, Deserialize)]
struct FulfillmentServicesWrapper { fulfillment_services: Vec<FulfillmentService> }
#[derive(Debug, Serialize)]
struct FulfillmentServiceRequest { fulfillment_service: FulfillmentService }

impl FulfillmentService {
    pub async fn find(client: &Client, id: i64) -> Result<Option<Self>> {
        let path = format!("fulfillment_services/{}.json", id);
        let response = client.get::<FulfillmentServiceWrapper>(&path).await?;
        Ok(Some(response.data.fulfillment_service))
    }

    pub async fn all(client: &Client, params: FulfillmentServiceListParams) -> Result<FindAllResponse<Self>> {
        let response = client.get_with_params::<FulfillmentServicesWrapper, _>("fulfillment_services.json", &params).await?;
        Ok(FindAllResponse {
            data: response.data.fulfillment_services,
            next_page: response.page_info.as_ref().and_then(|p| p.next.clone()),
            previous_page: response.page_info.as_ref().and_then(|p| p.previous.clone()),
        })
    }

    pub async fn create(client: &Client, service: &Self) -> Result<Self> {
        let request = FulfillmentServiceRequest { fulfillment_service: service.clone() };
        let response = client.post::<FulfillmentServiceWrapper, _>("fulfillment_services.json", &request).await?;
        Ok(response.data.fulfillment_service)
    }

    pub async fn update(client: &Client, service: &Self) -> Result<Self> {
        let id = service.id.ok_or_else(|| crate::ShopifyError::ValidationError("Fulfillment Service ID required".to_string()))?;
        let path = format!("fulfillment_services/{}.json", id);
        let request = FulfillmentServiceRequest { fulfillment_service: service.clone() };
        let response = client.put::<FulfillmentServiceWrapper, _>(&path, &request).await?;
        Ok(response.data.fulfillment_service)
    }

    pub async fn delete(client: &Client, id: i64) -> Result<()> {
        let path = format!("fulfillment_services/{}.json", id);
        client.delete(&path).await
    }
}
