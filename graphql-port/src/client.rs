//! HTTP client for making Shopify API requests

use crate::{
    error::{Result, ShopifyError},
    graphql::{GraphQLRequest, GraphQLResponse},
    session::Session,
    API_VERSION,
};
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
use serde::{de::DeserializeOwned, Serialize};

/// HTTP client for Shopify GraphQL API
#[derive(Debug, Clone)]
pub struct Client {
    pub session: Session,
    http_client: reqwest::Client,
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

    /// Execute a GraphQL query or mutation, with automatic retries on rate limits
    pub async fn graphql<T: DeserializeOwned, V: Serialize>(
        &self,
        query: &str,
        variables: Option<&V>,
    ) -> Result<T> {
        let url = self.session.graphql_url(API_VERSION);
        let mut retries = 0;
        let max_retries = 3;

        loop {
            let request_payload = GraphQLRequest { query, variables };

            let response = self
                .http_client
                .post(&url)
                .headers(self.build_headers())
                .json(&request_payload)
                .send()
                .await?;

            let status = response.status().as_u16();
            let body_text = response.text().await?;

            if status == 429 && retries < max_retries {
                retries += 1;
                tokio::time::sleep(std::time::Duration::from_secs(2)).await;
                continue;
            }

            if status >= 200 && status < 300 {
                // It was a valid HTTP request, parse the GraphQL Response Envelope
                let gql_response: GraphQLResponse<T> = serde_json::from_str(&body_text)?;

                // If GraphQL itself reported errors, trap them here.
                if let Some(errs) = &gql_response.errors {
                    // Check for Throttled in GraphQL level
                    if errs.iter().any(|e| e.message.contains("Throttled") || e.message.contains("throttled")) && retries < max_retries {
                        retries += 1;
                        let sleep_time = if let Some(ext) = &gql_response.extensions {
                            if let Some(cost) = &ext.cost {
                                // Default backoff math based on restore rate (minimum 1 point)
                                let rate = cost.throttle_status.restore_rate.max(1.0);
                                std::time::Duration::from_secs_f64((1.0 / rate) + 0.5)
                            } else {
                                std::time::Duration::from_secs(2)
                            }
                        } else {
                            std::time::Duration::from_secs(2)
                        };

                        tokio::time::sleep(sleep_time).await;
                        continue;
                    }

                    if !errs.is_empty() {
                        return Err(ShopifyError::GraphQLError(errs.clone()));
                    }
                }

                // Return the unboxed data wrapper
                if let Some(data) = gql_response.data {
                    return Ok(data);
                } else {
                    return Err(ShopifyError::ValidationError(
                        "GraphQL Response contained neither data nor errors array".to_string(),
                    ));
                }
            } else {
                return Err(ShopifyError::from_response(status, &body_text));
            }
        }
    }
}
