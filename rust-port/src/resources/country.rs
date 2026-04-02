//! Country resource

use crate::{base::FindAllResponse, client::Client, error::Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Country {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provinces: Option<Vec<serde_json::Value>>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct CountryListParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub since_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<String>,
}

#[derive(Debug, Deserialize)]
struct CountryWrapper { country: Country }
#[derive(Debug, Deserialize)]
struct CountriesWrapper { countries: Vec<Country> }
#[derive(Debug, Serialize)]
struct CountryRequest { country: Country }

impl Country {
    pub async fn find(client: &Client, id: i64) -> Result<Option<Self>> {
        let path = format!("countries/{}.json", id);
        let response = client.get::<CountryWrapper>(&path).await?;
        Ok(Some(response.data.country))
    }

    pub async fn all(client: &Client, params: CountryListParams) -> Result<FindAllResponse<Self>> {
        let response = client.get_with_params::<CountriesWrapper, _>("countries.json", &params).await?;
        Ok(FindAllResponse {
            data: response.data.countries,
            next_page: response.page_info.as_ref().and_then(|p| p.next.clone()),
            previous_page: response.page_info.as_ref().and_then(|p| p.previous.clone()),
        })
    }

    pub async fn count(client: &Client) -> Result<i64> {
        #[derive(Deserialize)]
        struct CountWrapper { count: i64 }
        let response = client.get::<CountWrapper>("countries/count.json").await?;
        Ok(response.data.count)
    }

    pub async fn create(client: &Client, country: &Self) -> Result<Self> {
        let request = CountryRequest { country: country.clone() };
        let response = client.post::<CountryWrapper, _>("countries.json", &request).await?;
        Ok(response.data.country)
    }

    pub async fn update(client: &Client, country: &Self) -> Result<Self> {
        let id = country.id.ok_or_else(|| crate::ShopifyError::ValidationError("Country ID required".to_string()))?;
        let path = format!("countries/{}.json", id);
        let request = CountryRequest { country: country.clone() };
        let response = client.put::<CountryWrapper, _>(&path, &request).await?;
        Ok(response.data.country)
    }

    pub async fn delete(client: &Client, id: i64) -> Result<()> {
        let path = format!("countries/{}.json", id);
        client.delete(&path).await
    }
}
