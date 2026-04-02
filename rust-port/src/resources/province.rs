//! Province resource

use crate::{base::FindAllResponse, client::Client, error::Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Province {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_zone_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_percentage: Option<f64>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct ProvinceListParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub since_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<String>,
}

#[derive(Debug, Deserialize)]
struct ProvinceWrapper { province: Province }
#[derive(Debug, Deserialize)]
struct ProvincesWrapper { provinces: Vec<Province> }
#[derive(Debug, Serialize)]
struct ProvinceRequest { province: Province }

impl Province {
    pub async fn find(client: &Client, country_id: i64, id: i64) -> Result<Option<Self>> {
        let path = format!("countries/{}/provinces/{}.json", country_id, id);
        let response = client.get::<ProvinceWrapper>(&path).await?;
        Ok(Some(response.data.province))
    }

    pub async fn all(client: &Client, country_id: i64, params: ProvinceListParams) -> Result<FindAllResponse<Self>> {
        let path = format!("countries/{}/provinces.json", country_id);
        let response = client.get_with_params::<ProvincesWrapper, _>(&path, &params).await?;
        Ok(FindAllResponse {
            data: response.data.provinces,
            next_page: response.page_info.as_ref().and_then(|p| p.next.clone()),
            previous_page: response.page_info.as_ref().and_then(|p| p.previous.clone()),
        })
    }

    pub async fn count(client: &Client, country_id: i64) -> Result<i64> {
        let path = format!("countries/{}/provinces/count.json", country_id);
        #[derive(Deserialize)]
        struct CountWrapper { count: i64 }
        let response = client.get::<CountWrapper>(&path).await?;
        Ok(response.data.count)
    }

    pub async fn update(client: &Client, country_id: i64, province: &Self) -> Result<Self> {
        let id = province.id.ok_or_else(|| crate::ShopifyError::ValidationError("Province ID required".to_string()))?;
        let path = format!("countries/{}/provinces/{}.json", country_id, id);
        let request = ProvinceRequest { province: province.clone() };
        let response = client.put::<ProvinceWrapper, _>(&path, &request).await?;
        Ok(response.data.province)
    }
}
