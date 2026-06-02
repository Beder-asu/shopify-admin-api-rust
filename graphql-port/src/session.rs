//! Session management for the Shopify GraphQL API client.

use url::Url;

/// The default API version to use when none is specified.
pub const DEFAULT_API_VERSION: &str = "2026-01";

/// Represents an authenticated merchant session.
///
/// Supports configurable `api_version` and `host_scheme` so the client can
/// target any platform (not just `*.myshopify.com`) and any API version.
#[derive(Debug, Clone)]
pub struct Session {
    /// The platform host (e.g. `"my-store.myplatform.com"`).
    pub shop: String,
    /// Bearer token for API authentication.
    pub access_token: String,
    /// API version (e.g. `"2026-01"`). Defaults to [`DEFAULT_API_VERSION`].
    pub api_version: String,
    /// URL scheme. Defaults to `"https"`.
    pub host_scheme: String,
}

impl Session {
    /// Create a session with default api_version and https scheme.
    ///
    /// If `shop` is a full URL (e.g. `"https://my-store.example.com"`), only
    /// the host portion is kept.
    pub fn new(shop: impl Into<String>, access_token: impl Into<String>) -> Self {
        let shop = shop.into();
        let shop = if shop.contains("://") {
            Url::parse(&shop)
                .map(|u| u.host_str().unwrap_or("").to_string())
                .unwrap_or(shop)
        } else {
            shop
        };

        Self {
            shop,
            access_token: access_token.into(),
            api_version: DEFAULT_API_VERSION.to_string(),
            host_scheme: "https".to_string(),
        }
    }

    /// Override the API version (builder-style).
    ///
    /// ```
    /// use shopify_graphql_api::Session;
    /// let session = Session::new("my-store.example.com", "token")
    ///     .with_api_version("2026-04");
    /// ```
    pub fn with_api_version(mut self, version: impl Into<String>) -> Self {
        self.api_version = version.into();
        self
    }

    /// Override the URL scheme (use `"http"` for local development only).
    pub fn with_scheme(mut self, scheme: impl Into<String>) -> Self {
        self.host_scheme = scheme.into();
        self
    }

    /// Build the full GraphQL endpoint URL for this session.
    ///
    /// Format: `{scheme}://{shop}/admin/api/{api_version}/graphql.json`
    pub fn graphql_url(&self) -> String {
        format!(
            "{}://{}/admin/api/{}/graphql.json",
            self.host_scheme, self.shop, self.api_version
        )
    }
}
