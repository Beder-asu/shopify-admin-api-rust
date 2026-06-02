//! HTTP client for making REST API requests.
//!
//! The client reads its base URL and configuration from [`Config`], so it
//! works with any host — not just `*.myshopify.com`. (Problems 7 & 8 fix)

use crate::config::Config;
use crate::error::{Result, ShopifyError};
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
use serde::{de::DeserializeOwned, Serialize};
use std::sync::Arc;
use std::time::Duration;
use tokio::time::sleep;

/// Maximum number of automatic retries on HTTP 429 responses.
const MAX_RETRIES: u32 = 3;

/// Default back-off when the `Retry-After` header is absent.
const DEFAULT_RETRY_SECS: f64 = 2.0;

/// Authenticated REST API client.
///
/// Obtain via [`ShopifyApp::rest_client`][crate::shopify::ShopifyApp::rest_client].
///
/// URL format: `{scheme}://{host_name}/admin/api/{api_version}/{path}`
#[derive(Debug, Clone)]
pub struct Client {
    config: Arc<Config>,
    access_token: String,
    http_client: reqwest::Client,
}

/// Response wrapper carrying the parsed data and optional pagination info.
#[derive(Debug, Clone)]
pub struct ApiResponse<T> {
    /// The deserialized response body.
    pub data: T,
    /// Link-header pagination info, if present.
    pub page_info: Option<PageInfo>,
}

/// Pagination links parsed from the `Link` response header.
#[derive(Debug, Clone, Default)]
pub struct PageInfo {
    /// Cursor URL for the next page of results.
    pub next: Option<String>,
    /// Cursor URL for the previous page of results.
    pub previous: Option<String>,
}

impl Client {
    /// Create a new `Client` from a validated [`Config`] and an access token.
    ///
    /// Prefer using [`ShopifyApp::rest_client`][crate::shopify::ShopifyApp::rest_client]
    /// which extracts the token from a [`Session`][crate::session::Session] automatically.
    pub fn new(config: Arc<Config>, access_token: String) -> Self {
        let http_client = reqwest::Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .expect("Failed to create HTTP client");

        Self {
            config,
            access_token,
            http_client,
        }
    }

    /// Build the full URL for an API path.
    fn url(&self, path: &str) -> String {
        format!("{}/{}", self.config.rest_base_url(), path)
    }

    /// Build the standard authentication headers.
    fn build_headers(&self) -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert(
            "X-Shopify-Access-Token",
            HeaderValue::from_str(&self.access_token)
                .unwrap_or_else(|_| HeaderValue::from_static("")),
        );
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        headers
    }

    /// Parse the `Link` response header into a [`PageInfo`].
    pub fn parse_page_info(headers: &HeaderMap) -> Option<PageInfo> {
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

    /// Extract the `Retry-After` header value in seconds, falling back to `DEFAULT_RETRY_SECS`.
    fn retry_after(headers: &HeaderMap) -> f64 {
        headers
            .get("Retry-After")
            .and_then(|v| v.to_str().ok())
            .and_then(|s| s.parse::<f64>().ok())
            .unwrap_or(DEFAULT_RETRY_SECS)
    }

    // ── HTTP Methods ────────────────────────────────────────────────────────────

    /// `GET {base_url}/{path}` with automatic 429 retry.
    pub async fn get<T: DeserializeOwned>(&self, path: &str) -> Result<ApiResponse<T>> {
        let url = self.url(path);
        let mut retries = 0u32;
        loop {
            let response = self
                .http_client
                .get(&url)
                .headers(self.build_headers())
                .send()
                .await?;

            let status = response.status().as_u16();

            // Problem 8 fix: retry on 429 up to MAX_RETRIES, parsing Retry-After header
            if status == 429 {
                if retries >= MAX_RETRIES {
                    let retry_after = Self::retry_after(response.headers()) as u64;
                    return Err(ShopifyError::RateLimited { retry_after });
                }
                let wait = Self::retry_after(response.headers());
                retries += 1;
                sleep(Duration::from_secs_f64(wait)).await;
                continue;
            }

            let page_info = Self::parse_page_info(response.headers());
            let body = response.text().await?;

            return if status >= 200 && status < 300 {
                let data: T = serde_json::from_str(&body)?;
                Ok(ApiResponse { data, page_info })
            } else {
                Err(ShopifyError::from_response(status, &body))
            };
        }
    }

    /// `GET {base_url}/{path}?{params}` with automatic 429 retry.
    pub async fn get_with_params<T: DeserializeOwned, P: Serialize>(
        &self,
        path: &str,
        params: &P,
    ) -> Result<ApiResponse<T>> {
        let url = self.url(path);
        let mut retries = 0u32;
        loop {
            let response = self
                .http_client
                .get(&url)
                .headers(self.build_headers())
                .query(params)
                .send()
                .await?;

            let status = response.status().as_u16();

            if status == 429 {
                if retries >= MAX_RETRIES {
                    let retry_after = Self::retry_after(response.headers()) as u64;
                    return Err(ShopifyError::RateLimited { retry_after });
                }
                let wait = Self::retry_after(response.headers());
                retries += 1;
                sleep(Duration::from_secs_f64(wait)).await;
                continue;
            }

            let page_info = Self::parse_page_info(response.headers());
            let body = response.text().await?;

            return if status >= 200 && status < 300 {
                let data: T = serde_json::from_str(&body)?;
                Ok(ApiResponse { data, page_info })
            } else {
                Err(ShopifyError::from_response(status, &body))
            };
        }
    }

    /// `POST {base_url}/{path}` with automatic 429 retry.
    pub async fn post<T: DeserializeOwned, B: Serialize>(
        &self,
        path: &str,
        body: &B,
    ) -> Result<ApiResponse<T>> {
        let url = self.url(path);
        let mut retries = 0u32;
        loop {
            let response = self
                .http_client
                .post(&url)
                .headers(self.build_headers())
                .json(body)
                .send()
                .await?;

            let status = response.status().as_u16();

            if status == 429 {
                if retries >= MAX_RETRIES {
                    let retry_after = Self::retry_after(response.headers()) as u64;
                    return Err(ShopifyError::RateLimited { retry_after });
                }
                let wait = Self::retry_after(response.headers());
                retries += 1;
                sleep(Duration::from_secs_f64(wait)).await;
                continue;
            }

            let page_info = Self::parse_page_info(response.headers());
            let body_text = response.text().await?;

            return if status >= 200 && status < 300 {
                let data: T = serde_json::from_str(&body_text)?;
                Ok(ApiResponse { data, page_info })
            } else {
                Err(ShopifyError::from_response(status, &body_text))
            };
        }
    }

    /// `PUT {base_url}/{path}` with automatic 429 retry.
    pub async fn put<T: DeserializeOwned, B: Serialize>(
        &self,
        path: &str,
        body: &B,
    ) -> Result<ApiResponse<T>> {
        let url = self.url(path);
        let mut retries = 0u32;
        loop {
            let response = self
                .http_client
                .put(&url)
                .headers(self.build_headers())
                .json(body)
                .send()
                .await?;

            let status = response.status().as_u16();

            if status == 429 {
                if retries >= MAX_RETRIES {
                    let retry_after = Self::retry_after(response.headers()) as u64;
                    return Err(ShopifyError::RateLimited { retry_after });
                }
                let wait = Self::retry_after(response.headers());
                retries += 1;
                sleep(Duration::from_secs_f64(wait)).await;
                continue;
            }

            let page_info = Self::parse_page_info(response.headers());
            let body_text = response.text().await?;

            return if status >= 200 && status < 300 {
                let data: T = serde_json::from_str(&body_text)?;
                Ok(ApiResponse { data, page_info })
            } else {
                Err(ShopifyError::from_response(status, &body_text))
            };
        }
    }

    /// `DELETE {base_url}/{path}` with automatic 429 retry.
    pub async fn delete(&self, path: &str) -> Result<()> {
        let url = self.url(path);
        let mut retries = 0u32;
        loop {
            let response = self
                .http_client
                .delete(&url)
                .headers(self.build_headers())
                .send()
                .await?;

            let status = response.status().as_u16();

            if status == 429 {
                if retries >= MAX_RETRIES {
                    let retry_after = Self::retry_after(response.headers()) as u64;
                    return Err(ShopifyError::RateLimited { retry_after });
                }
                let wait = Self::retry_after(response.headers());
                retries += 1;
                sleep(Duration::from_secs_f64(wait)).await;
                continue;
            }

            return if status >= 200 && status < 300 {
                Ok(())
            } else {
                let body = response.text().await?;
                Err(ShopifyError::from_response(status, &body))
            };
        }
    }

    /// `DELETE {base_url}/{path}?{params}` with automatic 429 retry.
    pub async fn delete_with_params<P: Serialize>(&self, path: &str, params: &P) -> Result<()> {
        let url = self.url(path);
        let mut retries = 0u32;
        loop {
            let response = self
                .http_client
                .delete(&url)
                .headers(self.build_headers())
                .query(params)
                .send()
                .await?;

            let status = response.status().as_u16();

            if status == 429 {
                if retries >= MAX_RETRIES {
                    let retry_after = Self::retry_after(response.headers()) as u64;
                    return Err(ShopifyError::RateLimited { retry_after });
                }
                let wait = Self::retry_after(response.headers());
                retries += 1;
                sleep(Duration::from_secs_f64(wait)).await;
                continue;
            }

            return if status >= 200 && status < 300 {
                Ok(())
            } else {
                let body = response.text().await?;
                Err(ShopifyError::from_response(status, &body))
            };
        }
    }
}
