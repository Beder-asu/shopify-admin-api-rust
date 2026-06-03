//! Authentication module — OAuth flow and scope types.
//!
//! Mirrors `lib/auth/` from `@shopify/shopify-api`.

pub mod nonce;
pub mod oauth;
pub mod scopes;

use crate::config::Config;
use crate::error::Result;
use std::sync::Arc;

pub use oauth::{BeginParams, CallbackParams, CallbackResponse, OAuthState, STATE_COOKIE_NAME};
pub use scopes::AuthScopes;

/// OAuth and authentication operations.
///
/// Obtain via [`ShopifyApp::auth`][crate::shopify::ShopifyApp].
///
/// Mirrors `shopify.auth` from `@shopify/shopify-api`.
pub struct AuthModule {
    config: Arc<Config>,
    http: reqwest::Client,
}

impl AuthModule {
    /// Create a new `AuthModule` bound to the given config.
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            config,
            http: reqwest::Client::new(),
        }
    }

    /// Build the OAuth authorization redirect URL.
    ///
    /// Mirrors `shopify.auth.begin()`.
    pub fn begin(&self, params: BeginParams) -> Result<OAuthState> {
        oauth::begin(&self.config, params)
    }

    /// Validate the OAuth callback and exchange the code for a session.
    ///
    /// Mirrors `shopify.auth.callback()`.
    pub async fn callback(
        &self,
        params: CallbackParams,
        state_cookie: &str,
    ) -> Result<CallbackResponse> {
        oauth::callback(&self.config, &self.http, params, state_cookie).await
    }
}
