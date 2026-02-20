//! Theme resource

use crate::{base::FindAllResponse, client::Client, error::Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Theme {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub theme_store_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previewable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processing: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub admin_graphql_api_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct ThemeListParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<String>,
}

#[derive(Debug, Deserialize)]
struct ThemeWrapper { theme: Theme }
#[derive(Debug, Deserialize)]
struct ThemesWrapper { themes: Vec<Theme> }
#[derive(Debug, Serialize)]
struct ThemeRequest { theme: Theme }

impl Theme {
    pub async fn find(client: &Client, id: i64) -> Result<Option<Self>> {
        let path = format!("themes/{}.json", id);
        let response = client.get::<ThemeWrapper>(&path).await?;
        Ok(Some(response.data.theme))
    }

    pub async fn all(client: &Client, params: ThemeListParams) -> Result<FindAllResponse<Self>> {
        let response = client.get_with_params::<ThemesWrapper, _>("themes.json", &params).await?;
        Ok(FindAllResponse {
            data: response.data.themes,
            next_page: response.page_info.as_ref().and_then(|p| p.next.clone()),
            previous_page: response.page_info.as_ref().and_then(|p| p.previous.clone()),
        })
    }

    pub async fn create(client: &Client, theme: &Self) -> Result<Self> {
        let request = ThemeRequest { theme: theme.clone() };
        let response = client.post::<ThemeWrapper, _>("themes.json", &request).await?;
        Ok(response.data.theme)
    }

    pub async fn update(client: &Client, theme: &Self) -> Result<Self> {
        let id = theme.id.ok_or_else(|| crate::ShopifyError::ValidationError("Theme ID required".to_string()))?;
        let path = format!("themes/{}.json", id);
        let request = ThemeRequest { theme: theme.clone() };
        let response = client.put::<ThemeWrapper, _>(&path, &request).await?;
        Ok(response.data.theme)
    }

    pub async fn delete(client: &Client, id: i64) -> Result<()> {
        let path = format!("themes/{}.json", id);
        client.delete(&path).await
    }
}
