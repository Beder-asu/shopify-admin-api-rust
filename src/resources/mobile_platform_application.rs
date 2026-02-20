//! Mobile Platform Application resource

use crate::{base::FindAllResponse, client::Client, error::Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MobilePlatformApplication {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sha256_cert_fingerprints: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled_universal_or_app_links: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled_shared_webcredentials: Option<bool>,
}

#[derive(Debug, Deserialize)]
struct MobilePlatformApplicationWrapper { mobile_platform_application: MobilePlatformApplication }
#[derive(Debug, Deserialize)]
struct MobilePlatformApplicationsWrapper { mobile_platform_applications: Vec<MobilePlatformApplication> }
#[derive(Debug, Serialize)]
struct MobilePlatformApplicationRequest { mobile_platform_application: MobilePlatformApplication }

impl MobilePlatformApplication {
    pub async fn find(client: &Client, id: i64) -> Result<Option<Self>> {
        let path = format!("mobile_platform_applications/{}.json", id);
        let response = client.get::<MobilePlatformApplicationWrapper>(&path).await?;
        Ok(Some(response.data.mobile_platform_application))
    }

    pub async fn all(client: &Client) -> Result<FindAllResponse<Self>> {
        let response = client.get::<MobilePlatformApplicationsWrapper>("mobile_platform_applications.json").await?;
        Ok(FindAllResponse {
            data: response.data.mobile_platform_applications,
            next_page: response.page_info.as_ref().and_then(|p| p.next.clone()),
            previous_page: response.page_info.as_ref().and_then(|p| p.previous.clone()),
        })
    }

    pub async fn create(client: &Client, app: &Self) -> Result<Self> {
        let request = MobilePlatformApplicationRequest { mobile_platform_application: app.clone() };
        let response = client.post::<MobilePlatformApplicationWrapper, _>("mobile_platform_applications.json", &request).await?;
        Ok(response.data.mobile_platform_application)
    }

    pub async fn update(client: &Client, app: &Self) -> Result<Self> {
        let id = app.id.ok_or_else(|| crate::ShopifyError::ValidationError("Mobile Platform Application ID required".to_string()))?;
        let path = format!("mobile_platform_applications/{}.json", id);
        let request = MobilePlatformApplicationRequest { mobile_platform_application: app.clone() };
        let response = client.put::<MobilePlatformApplicationWrapper, _>(&path, &request).await?;
        Ok(response.data.mobile_platform_application)
    }

    pub async fn delete(client: &Client, id: i64) -> Result<()> {
        let path = format!("mobile_platform_applications/{}.json", id);
        client.delete(&path).await
    }
}
