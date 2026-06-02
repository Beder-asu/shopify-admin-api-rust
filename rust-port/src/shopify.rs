//! Top-level SDK entry point.
//!
//! Mirrors `shopifyApi()` / `Shopify` interface from `lib/index.ts` in `@shopify/shopify-api`.

use crate::auth::AuthModule;
use crate::client::Client;
use crate::config::{validate_config, Config, ConfigParams};
use crate::error::Result;
use crate::logger::Logger;
use crate::session::Session;
use crate::session_manager::SessionManager;
use crate::session_store::SessionStore;
use crate::webhooks;
use crate::webhooks::WebhookValidation;
use reqwest::header::HeaderMap;
use std::sync::Arc;

/// Webhook operations module, accessible via [`ShopifyApp::webhooks`].
pub struct WebhooksModule {
    config: Arc<Config>,
}

impl WebhooksModule {
    fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    /// Validate an incoming webhook's HMAC-SHA256 signature using constant-time comparison.
    ///
    /// - `raw_body` — the raw POST body bytes (before JSON parsing)
    /// - `hmac_header` — value of the `X-Shopify-Hmac-Sha256` header
    ///
    /// Mirrors `shopify.webhooks.validate()`.
    pub fn validate_hmac(&self, raw_body: &[u8], hmac_header: &str) -> bool {
        webhooks::validate_hmac(&self.config.api_secret_key, raw_body, hmac_header)
    }

    /// Full webhook validation: HMAC check + required header extraction.
    ///
    /// Returns a [`WebhookValidation`] with `valid`, `fields`, and `error` fields.
    pub fn validate(&self, raw_body: &[u8], headers: &HeaderMap) -> WebhookValidation {
        webhooks::validate(&self.config.api_secret_key, raw_body, headers)
    }
}

/// The top-level SDK instance.
///
/// Create one at application startup, then use it to obtain HTTP clients,
/// run the OAuth flow, validate webhooks, and manage sessions.
///
/// ```rust,no_run
/// use shopify_admin_api::{ShopifyApp, ConfigParams, MemorySessionStore};
///
/// # async fn example() -> shopify_admin_api::error::Result<()> {
/// let shopify = ShopifyApp::new(
///     ConfigParams {
///         api_key: "your_client_id".into(),
///         api_secret_key: "your_secret".into(),
///         host_name: "app.yourplatform.com".into(),
///         api_version: "2026-01".into(),
///         ..Default::default()
///     },
///     MemorySessionStore::new(),
/// )?;
///
/// // Begin OAuth for a merchant
/// let oauth = shopify.auth.begin(shopify_admin_api::auth::BeginParams {
///     shop: "my-store.myplatform.com".into(),
///     callback_path: "/auth/callback".into(),
///     is_online: false,
/// })?;
/// // Redirect the merchant to oauth.redirect_url
/// # Ok(())
/// # }
/// ```
///
/// Mirrors the `Shopify` interface and `shopifyApi()` function from `@shopify/shopify-api`.
pub struct ShopifyApp<S: SessionStore> {
    /// Validated, immutable SDK configuration.
    pub config: Arc<Config>,
    /// OAuth flow: `begin()` and `callback()`.
    pub auth: AuthModule,
    /// Webhook validation.
    pub webhooks: WebhooksModule,
    /// Session storage and management.
    pub session: SessionManager<S>,
    /// Structured logger.
    pub logger: Logger,
}

impl<S: SessionStore> ShopifyApp<S> {
    /// Initialize the SDK from `ConfigParams` and a [`SessionStore`] implementation.
    ///
    /// Validates all required configuration fields — returns
    /// `Err(ShopifyError::InvalidConfig)` if any are missing.
    ///
    /// Mirrors `shopifyApi(config)` from `@shopify/shopify-api`.
    pub fn new(params: ConfigParams, store: S) -> Result<Self> {
        let config = Arc::new(validate_config(params)?);
        let logger = Logger::new(Arc::new(config.logger.clone()));

        logger.info(&format!(
            "shopify-admin-api v{} initialized (host: {}, api_version: {})",
            env!("CARGO_PKG_VERSION"),
            config.host_name,
            config.api_version,
        ));

        Ok(Self {
            auth: AuthModule::new(Arc::clone(&config)),
            webhooks: WebhooksModule::new(Arc::clone(&config)),
            session: SessionManager::new(Arc::clone(&config), store),
            logger,
            config,
        })
    }

    /// Create a REST API client authenticated with the given session.
    ///
    /// Mirrors `new shopify.clients.Rest({ session })` from `@shopify/shopify-api`.
    ///
    /// # Panics
    /// Does not panic — if `session.access_token` is `None`, requests will return
    /// 401 Unauthorized from the server.
    pub fn rest_client(&self, session: &Session) -> Client {
        Client::new(
            Arc::clone(&self.config),
            session.access_token.clone().unwrap_or_default(),
        )
    }
}
