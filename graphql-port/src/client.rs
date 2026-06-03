//! HTTP client for making Shopify GraphQL API requests.
//!
//! The `API_VERSION` constant has been removed; the URL is now built from
//! `Session::graphql_url()` which reads `api_version` from the session.
//! The 429 retry loop reads the `Retry-After` header (Problem 8 mirror for graphql-port).

use crate::graphql::{GraphQLRequest, GraphQLResponse};
use shopify_core::error::{Result, ShopifyError};
use shopify_core::session::Session;
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
use serde::{de::DeserializeOwned, Serialize};
use std::time::Duration;
use tokio::time::sleep;

/// Maximum automatic retries on HTTP 429 or GraphQL Throttled.
const MAX_RETRIES: u32 = 3;

/// Authenticated GraphQL API client.
///
/// # Example
/// ```rust,no_run
/// use shopify_graphql_api::{Client, Session};
///
/// # async fn example() {
/// let session = Session::new("my-shop.myplatform.com", "shpat_xxx")
///     .with_api_version("2026-01");
/// let client = Client::new(session);
/// # }
/// ```
#[derive(Debug, Clone)]
pub struct Client {
    /// The authenticated session used to build request URLs and headers.
    pub session: Session,
    /// API version to use for GraphQL requests.
    pub api_version: String,
    http_client: reqwest::Client,
}

impl Client {
    /// Create a new `Client` with the given [`Session`].
    pub fn new(session: Session, api_version: impl Into<String>) -> Self {
        let http_client = reqwest::Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .expect("Failed to create HTTP client");

        Self {
            session,
            api_version: api_version.into(),
            http_client,
        }
    }

    /// Override the default 30-second HTTP request timeout.
    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.http_client = reqwest::Client::builder()
            .timeout(timeout)
            .build()
            .expect("Failed to create HTTP client");
        self
    }

    /// Build the full GraphQL endpoint URL for this session.
    fn graphql_url(&self) -> String {
        format!(
            "https://{}/admin/api/{}/graphql.json",
            self.session.shop, self.api_version
        )
    }

    fn build_headers(&self) -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert(
            "X-Shopify-Access-Token",
            HeaderValue::from_str(self.session.access_token.as_deref().unwrap_or(""))
                .unwrap_or_else(|_| HeaderValue::from_static("")),
        );
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        headers
    }

    /// Extract `Retry-After` seconds from response headers, defaulting to 2.0.
    fn retry_after(headers: &reqwest::header::HeaderMap) -> f64 {
        headers
            .get("Retry-After")
            .and_then(|v| v.to_str().ok())
            .and_then(|s| s.parse::<f64>().ok())
            .unwrap_or(2.0)
    }

    /// Execute a GraphQL query or mutation, with automatic retries on rate limits.
    ///
    /// Returns the full [`GraphQLResponse`] envelope so callers can inspect
    /// both `data` and `errors` (for partial-success responses).
    ///
    /// Use [`graphql_data`] if you want errors to be surfaced as `Err`.
    pub async fn graphql<T: DeserializeOwned, V: Serialize>(
        &self,
        query: &str,
        variables: Option<&V>,
    ) -> Result<GraphQLResponse<T>> {
        let url = self.graphql_url();
        let mut retries = 0u32;

        loop {
            let request_payload = GraphQLRequest { query, variables };

            let response = self
                .http_client
                .post(&url)
                .headers(self.build_headers())
                .json(&request_payload)
                .send()
                .await?;

            // HTTP-level rate limit
            if response.status().as_u16() == 429 {
                if retries >= MAX_RETRIES {
                    let retry_after = Self::retry_after(response.headers()) as u64;
                    return Err(ShopifyError::RateLimited { retry_after });
                }
                let wait = Self::retry_after(response.headers());
                retries += 1;
                sleep(Duration::from_secs_f64(wait)).await;
                continue;
            }

            let status = response.status().as_u16();
            let body_text = response.text().await?;

            if status >= 200 && status < 300 {
                let gql_response: GraphQLResponse<T> = serde_json::from_str(&body_text)?;

                // GraphQL-level Throttled error — retry with cost-based backoff
                if let Some(errs) = &gql_response.errors {
                    let is_throttled = errs.iter().any(|e: &shopify_core::error::GraphQLErrorDetail| {
                        e.message.to_lowercase().contains("throttled")
                    });
                    if is_throttled && retries < MAX_RETRIES {
                        retries += 1;
                        let sleep_time = gql_response
                            .extensions
                            .as_ref()
                            .and_then(|ext| ext.cost.as_ref())
                            .map(|cost| {
                                let rate = cost.throttle_status.restore_rate.max(1.0);
                                Duration::from_secs_f64((1.0 / rate) + 0.5)
                            })
                            .unwrap_or(Duration::from_secs(2));
                        sleep(sleep_time).await;
                        continue;
                    }
                }

                return Ok(gql_response);
            } else {
                return Err(ShopifyError::from_response(status, &body_text));
            }
        }
    }

    /// Execute a GraphQL query and extract the `data` field.
    ///
    /// Returns `Err(ShopifyError::GraphQLError)` if the response contains
    /// errors AND no data, or `Err(ShopifyError::NoData)` if the response
    /// has neither data nor errors.
    ///
    /// For partial-success responses (data + errors), prefer [`graphql`]
    /// which returns the full envelope.
    pub async fn graphql_data<T: DeserializeOwned, V: Serialize>(
        &self,
        query: &str,
        variables: Option<&V>,
    ) -> Result<T> {
        let response = self.graphql::<T, V>(query, variables).await?;

        match (response.data, response.errors) {
            // Data present — return it (ignore any partial errors)
            (Some(data), _) => Ok(data),
            // No data, but errors present — surface them
            (None, Some(errors)) => Err(ShopifyError::GraphQLError(errors)),
            // Nothing at all
            (None, None) => Err(ShopifyError::NoData),
        }
    }
}
