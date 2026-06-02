//! Higher-level session management built on top of [`SessionStore`].
//!
//! Mirrors `lib/session/` from `@shopify/shopify-api`.

use crate::auth::scopes::AuthScopes;
use crate::config::Config;
use crate::error::Result;
use crate::session::Session;
use crate::session_store::SessionStore;
use std::sync::Arc;

/// Higher-level session operations backed by a [`SessionStore`].
///
/// Obtain via [`ShopifyApp::session`][crate::shopify::ShopifyApp].
///
/// Mirrors `shopify.session` from `@shopify/shopify-api`.
pub struct SessionManager<S: SessionStore> {
    config: Arc<Config>,
    store: S,
}

impl<S: SessionStore> SessionManager<S> {
    /// Create a new `SessionManager` with the given config and store.
    pub fn new(config: Arc<Config>, store: S) -> Self {
        Self { config, store }
    }

    /// Persist a session.
    pub async fn store_session(&self, session: Session) -> Result<bool> {
        self.store.store_session(session).await
    }

    /// Load a session by ID.
    pub async fn load_session(&self, id: &str) -> Result<Option<Session>> {
        self.store.load_session(id).await
    }

    /// Load a session only if it is still active (non-expired, correct scopes).
    ///
    /// Applies a 500 ms expiry buffer (matching the TS SDK default).
    ///
    /// Mirrors the pattern of `shopify.session.loadSession()` + `session.isActive()`.
    pub async fn get_active_session(&self, session_id: &str) -> Result<Option<Session>> {
        let session = self.store.load_session(session_id).await?;
        let required = self
            .config
            .scopes
            .as_ref()
            .cloned()
            .unwrap_or_else(AuthScopes::default);

        Ok(session.filter(|s| s.is_active(&required, 500)))
    }

    /// Delete a single session by ID.
    pub async fn delete_session(&self, id: &str) -> Result<bool> {
        self.store.delete_session(id).await
    }

    /// Delete all sessions for a shop (e.g. on app uninstall).
    ///
    /// Mirrors `shopify.session.deleteShopSessions(shop)`.
    pub async fn delete_shop_sessions(&self, shop: &str) -> Result<bool> {
        let sessions = self.store.find_sessions_by_shop(shop).await?;
        let ids: Vec<&str> = sessions.iter().map(|s| s.id.as_str()).collect();
        if ids.is_empty() {
            return Ok(true);
        }
        self.store.delete_sessions(&ids).await
    }

    /// Find all sessions for a shop.
    pub async fn find_sessions_by_shop(&self, shop: &str) -> Result<Vec<Session>> {
        self.store.find_sessions_by_shop(shop).await
    }

    /// The canonical offline session ID for a shop.
    ///
    /// Mirrors `shopify.session.getOfflineId(shop)`.
    pub fn offline_session_id(shop: &str) -> String {
        Session::offline_id(shop)
    }
}
