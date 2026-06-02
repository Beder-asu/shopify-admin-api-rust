//! SDK configuration types and validation.
//!
//! Mirrors `lib/config.ts` and `lib/base-types.ts` from `@shopify/shopify-api`.

use crate::auth::scopes::AuthScopes;
use crate::error::{Result, ShopifyError};
use std::fmt;

/// URL scheme for API requests.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Scheme {
    /// Plain HTTP (for local development only).
    Http,
    /// HTTPS (default, required in production).
    #[default]
    Https,
}

impl fmt::Display for Scheme {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Scheme::Http => write!(f, "http"),
            Scheme::Https => write!(f, "https"),
        }
    }
}

/// Minimum log severity level.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub enum LogLevel {
    /// Verbose tracing output.
    Debug,
    /// Normal operational messages (default).
    #[default]
    Info,
    /// Non-fatal warnings.
    Warning,
    /// Errors only.
    Error,
}

/// Logger configuration.
#[derive(Debug, Clone)]
pub struct LogConfig {
    /// Minimum severity level to emit.
    pub level: LogLevel,
    /// Whether to log each HTTP request.
    pub log_http_requests: bool,
    /// Whether to prepend timestamps to log output.
    pub timestamps: bool,
}

impl Default for LogConfig {
    fn default() -> Self {
        Self {
            level: LogLevel::Info,
            log_http_requests: false,
            timestamps: false,
        }
    }
}

/// Input parameters for initializing the SDK.
///
/// Mirrors `ConfigParams` from `@shopify/shopify-api`. After validation
/// via `validate_config()`, produces an immutable [`Config`].
#[derive(Debug, Clone, Default)]
pub struct ConfigParams {
    /// OAuth client ID (API key). Required for public apps.
    pub api_key: String,
    /// OAuth client secret. Always required.
    pub api_secret_key: String,
    /// Your platform's host name (e.g. `"app.yourfirm.com"`).
    /// Trailing slashes are stripped automatically.
    pub host_name: String,
    /// HTTP or HTTPS (default: HTTPS).
    pub host_scheme: Option<Scheme>,
    /// API version string, e.g. `"2026-01"`.
    pub api_version: String,
    /// OAuth scopes your app requests (e.g. `["read_products","write_orders"]`).
    pub scopes: Option<AuthScopes>,
    /// Whether the app is embedded in the platform's admin UI.
    pub is_embedded_app: bool,
    /// Whether this is a custom store app with a static admin token.
    pub is_custom_store_app: bool,
    /// Static admin API access token for custom store apps.
    pub admin_api_access_token: Option<String>,
    /// Additional shop domain patterns to allow (beyond the platform default).
    pub custom_shop_domains: Vec<String>,
    /// Logger configuration.
    pub logger: Option<LogConfig>,
}

/// Validated, immutable SDK configuration.
///
/// Produced by [`validate_config`]. Mirrors `ConfigInterface` from `@shopify/shopify-api`.
#[derive(Debug, Clone)]
pub struct Config {
    /// OAuth client ID.
    pub api_key: String,
    /// OAuth client secret (used for HMAC validation).
    pub api_secret_key: String,
    /// Platform host, e.g. `"app.yourfirm.com"` (no trailing slash).
    pub host_name: String,
    /// URL scheme (http or https).
    pub host_scheme: Scheme,
    /// API version, e.g. `"2026-01"`.
    pub api_version: String,
    /// Parsed OAuth scopes.
    pub scopes: Option<AuthScopes>,
    /// Whether the app is embedded in the admin UI.
    pub is_embedded_app: bool,
    /// Whether this is a custom store app.
    pub is_custom_store_app: bool,
    /// Static admin token for custom store apps.
    pub admin_api_access_token: Option<String>,
    /// Allowed shop domain patterns.
    pub custom_shop_domains: Vec<String>,
    /// Logger settings.
    pub logger: LogConfig,
}

impl Config {
    /// Build the base REST API URL for this configuration.
    ///
    /// Format: `{scheme}://{host_name}/admin/api/{api_version}`
    pub fn rest_base_url(&self) -> String {
        format!(
            "{}://{}/admin/api/{}",
            self.host_scheme, self.host_name, self.api_version
        )
    }

    /// Build the GraphQL API endpoint URL.
    ///
    /// Format: `{scheme}://{host_name}/admin/api/{api_version}/graphql.json`
    pub fn graphql_url(&self) -> String {
        format!(
            "{}://{}/admin/api/{}/graphql.json",
            self.host_scheme, self.host_name, self.api_version
        )
    }
}

/// Validate input parameters and produce an immutable [`Config`].
///
/// Returns `Err(ShopifyError::InvalidConfig)` listing all missing required fields.
///
/// Mirrors `validateConfig()` from `@shopify/shopify-api`.
pub fn validate_config(params: ConfigParams) -> Result<Config> {
    let mut missing: Vec<&str> = Vec::new();

    if params.api_secret_key.is_empty() {
        missing.push("api_secret_key");
    }
    if params.host_name.is_empty() {
        missing.push("host_name");
    }
    if params.api_version.is_empty() {
        missing.push("api_version");
    }
    if !params.is_custom_store_app && params.api_key.is_empty() {
        missing.push("api_key");
    }
    if params.is_custom_store_app
        && params
            .admin_api_access_token
            .as_deref()
            .unwrap_or("")
            .is_empty()
    {
        missing.push("admin_api_access_token");
    }

    if !missing.is_empty() {
        return Err(ShopifyError::InvalidConfig(missing.join(", ")));
    }

    Ok(Config {
        api_key: params.api_key,
        api_secret_key: params.api_secret_key,
        host_name: params.host_name.trim_end_matches('/').to_string(),
        host_scheme: params.host_scheme.unwrap_or(Scheme::Https),
        api_version: params.api_version,
        scopes: params.scopes,
        is_embedded_app: params.is_embedded_app,
        is_custom_store_app: params.is_custom_store_app,
        admin_api_access_token: params.admin_api_access_token,
        custom_shop_domains: params.custom_shop_domains,
        logger: params.logger.unwrap_or_default(),
    })
}
