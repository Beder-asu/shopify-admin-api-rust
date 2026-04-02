//! Carrier Service resource

use crate::{base::FindAllResponse, client::Client, error::Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CarrierService {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_discovery: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub carrier_service_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub admin_graphql_api_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback_url: Option<String>,
}

#[derive(Debug, Deserialize)]
struct CarrierServiceWrapper { carrier_service: CarrierService }
#[derive(Debug, Deserialize)]
struct CarrierServicesWrapper { carrier_services: Vec<CarrierService> }
#[derive(Debug, Serialize)]
struct CarrierServiceRequest { carrier_service: CarrierService }

impl CarrierService {
    pub async fn find(client: &Client, id: i64) -> Result<Option<Self>> {
        let path = format!("carrier_services/{}.json", id);
        let response = client.get::<CarrierServiceWrapper>(&path).await?;
        Ok(Some(response.data.carrier_service))
    }

    pub async fn all(client: &Client) -> Result<FindAllResponse<Self>> {
        let response = client.get::<CarrierServicesWrapper>("carrier_services.json").await?;
        Ok(FindAllResponse {
            data: response.data.carrier_services,
            next_page: response.page_info.as_ref().and_then(|p| p.next.clone()),
            previous_page: response.page_info.as_ref().and_then(|p| p.previous.clone()),
        })
    }

    pub async fn create(client: &Client, service: &Self) -> Result<Self> {
        let request = CarrierServiceRequest { carrier_service: service.clone() };
        let response = client.post::<CarrierServiceWrapper, _>("carrier_services.json", &request).await?;
        Ok(response.data.carrier_service)
    }

    pub async fn update(client: &Client, service: &Self) -> Result<Self> {
        let id = service.id.ok_or_else(|| crate::ShopifyError::ValidationError("Carrier Service ID required".to_string()))?;
        let path = format!("carrier_services/{}.json", id);
        let request = CarrierServiceRequest { carrier_service: service.clone() };
        let response = client.put::<CarrierServiceWrapper, _>(&path, &request).await?;
        Ok(response.data.carrier_service)
    }

    pub async fn delete(client: &Client, id: i64) -> Result<()> {
        let path = format!("carrier_services/{}.json", id);
        client.delete(&path).await
    }
}
