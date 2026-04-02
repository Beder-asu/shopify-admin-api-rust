//! Session management for Shopify API
use url::Url;

/// Represents a merchant's Shopify session
#[derive(Debug, Clone)]
pub struct Session {
    /// The shop domain (e.g. `shop.myshopify.com`)
    pub shop: String,
    /// The access token (from OAuth or private app)
    pub access_token: String,
}

impl Session {
    /// Create a new session
    pub fn new(shop: impl Into<String>, access_token: impl Into<String>) -> Self {
        let shop = shop.into();
        let shop = if shop.contains("://") {
            Url::parse(&shop).map(|u| u.host_str().unwrap_or("").to_string()).unwrap_or(shop)
        } else {
            shop
        };

        Self {
            shop,
            access_token: access_token.into(),
        }
    }

    /// Build the base URL for the GraphQL API
    pub fn graphql_url(&self, api_version: &str) -> String {
        format!("https://{}/admin/api/{}/graphql.json", self.shop, api_version)
    }
}
