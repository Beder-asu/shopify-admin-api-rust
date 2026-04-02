//! HTTP client for making Shopify API requests

use crate::{error::Result, session::Session, ShopifyError};
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
use serde::{de::DeserializeOwned, Serialize};

/// HTTP client for Shopify API requests
#[derive(Debug, Clone)]
pub struct Client {
    session: Session,
    http_client: reqwest::Client,
}

/// Response wrapper with pagination info
#[derive(Debug, Clone)]
pub struct ApiResponse<T> {
    /// The response data
    pub data: T,
    /// Link header for pagination
    pub page_info: Option<PageInfo>,
}

/// Pagination information from Link headers
#[derive(Debug, Clone, Default)]
pub struct PageInfo {
    /// URL for the next page
    pub next: Option<String>,
    /// URL for the previous page
    pub previous: Option<String>,
}

impl Client {
    /// Create a new client with the given session
    pub fn new(session: Session) -> Self {
        let http_client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .expect("Failed to create HTTP client");

        Self {
            session,
            http_client,
        }
    }

    /// Get the session
    pub fn session(&self) -> &Session {
        &self.session
    }

    /// Build common headers for API requests
    fn build_headers(&self) -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert(
            "X-Shopify-Access-Token",
            HeaderValue::from_str(&self.session.access_token).unwrap(),
        );
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        headers
    }

    /// Parse pagination from Link header
    fn parse_page_info(headers: &reqwest::header::HeaderMap) -> Option<PageInfo> {
        let link = headers.get("link")?.to_str().ok()?;
        let mut page_info = PageInfo::default();

        for part in link.split(',') {
            let parts: Vec<&str> = part.split(';').collect();
            if parts.len() == 2 {
                let url = parts[0].trim().trim_matches(|c| c == '<' || c == '>');
                let rel = parts[1].trim();
                if rel.contains("next") {
                    page_info.next = Some(url.to_string());
                } else if rel.contains("previous") {
                    page_info.previous = Some(url.to_string());
                }
            }
        }

        Some(page_info)
    }

    /// Make a GET request
    pub async fn get<T: DeserializeOwned>(&self, path: &str) -> Result<ApiResponse<T>> {
        let url = format!("{}/{}", self.session.api_url(), path);
        let response = self
            .http_client
            .get(&url)
            .headers(self.build_headers())
            .send()
            .await?;

        let status = response.status().as_u16();
        let page_info = Self::parse_page_info(response.headers());
        let body = response.text().await?;

        if status >= 200 && status < 300 {
            let data: T = serde_json::from_str(&body)?;
            Ok(ApiResponse { data, page_info })
        } else {
            Err(ShopifyError::from_response(status, &body))
        }
    }

    /// Make a GET request with query parameters
    pub async fn get_with_params<T: DeserializeOwned, P: Serialize>(
        &self,
        path: &str,
        params: &P,
    ) -> Result<ApiResponse<T>> {
        let url = format!("{}/{}", self.session.api_url(), path);
        let response = self
            .http_client
            .get(&url)
            .headers(self.build_headers())
            .query(params)
            .send()
            .await?;

        let status = response.status().as_u16();
        let page_info = Self::parse_page_info(response.headers());
        let body = response.text().await?;

        if status >= 200 && status < 300 {
            let data: T = serde_json::from_str(&body)?;
            Ok(ApiResponse { data, page_info })
        } else {
            Err(ShopifyError::from_response(status, &body))
        }
    }

    /// Make a POST request
    pub async fn post<T: DeserializeOwned, B: Serialize>(
        &self,
        path: &str,
        body: &B,
    ) -> Result<ApiResponse<T>> {
        let url = format!("{}/{}", self.session.api_url(), path);
        let response = self
            .http_client
            .post(&url)
            .headers(self.build_headers())
            .json(body)
            .send()
            .await?;

        let status = response.status().as_u16();
        let page_info = Self::parse_page_info(response.headers());
        let body_text = response.text().await?;

        if status >= 200 && status < 300 {
            let data: T = serde_json::from_str(&body_text)?;
            Ok(ApiResponse {
                data,
                page_info,
            })
        } else {
            Err(ShopifyError::from_response(status, &body_text))
        }
    }

    /// Make a PUT request
    pub async fn put<T: DeserializeOwned, B: Serialize>(
        &self,
        path: &str,
        body: &B,
    ) -> Result<ApiResponse<T>> {
        let url = format!("{}/{}", self.session.api_url(), path);
        let response = self
            .http_client
            .put(&url)
            .headers(self.build_headers())
            .json(body)
            .send()
            .await?;

        let status = response.status().as_u16();
        let page_info = Self::parse_page_info(response.headers());
        let body_text = response.text().await?;

        if status >= 200 && status < 300 {
            let data: T = serde_json::from_str(&body_text)?;
            Ok(ApiResponse {
                data,
                page_info,
            })
        } else {
            Err(ShopifyError::from_response(status, &body_text))
        }
    }

    /// Make a DELETE request
    pub async fn delete(&self, path: &str) -> Result<()> {
        let url = format!("{}/{}", self.session.api_url(), path);
        let response = self
            .http_client
            .delete(&url)
            .headers(self.build_headers())
            .send()
            .await?;

        let status = response.status().as_u16();
        if status >= 200 && status < 300 {
            Ok(())
        } else {
            let body = response.text().await?;
            Err(ShopifyError::from_response(status, &body))
        }
    }

    /// Make a DELETE request with query parameters
    pub async fn delete_with_params<P: Serialize>(&self, path: &str, params: &P) -> Result<()> {
        let url = format!("{}/{}", self.session.api_url(), path);
        let response = self
            .http_client
            .delete(&url)
            .headers(self.build_headers())
            .query(params)
            .send()
            .await?;

        let status = response.status().as_u16();
        if status >= 200 && status < 300 {
            Ok(())
        } else {
            let body = response.text().await?;
            Err(ShopifyError::from_response(status, &body))
        }
    }
}
