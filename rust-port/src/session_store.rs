//! Pluggable session storage trait and in-memory implementation.
//!
//! Mirrors the TypeScript `SessionStorage` interface from `@shopify/shopify-api`.

use crate::error::Result;
use crate::session::Session;
use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// A pluggable storage backend for [`Session`] objects.
///
/// Implement this trait with your database adapter (PostgreSQL, Redis, etc.).
/// The [`MemorySessionStore`] is provided for development and testing only.
///
/// Mirrors the `SessionStorage` interface from `@shopify/shopify-api`.
#[async_trait]
pub trait SessionStore: Send + Sync {
    /// Persist a session. Returns `true` if successful.
    async fn store_session(&self, session: Session) -> Result<bool>;

    /// Load a session by its unique ID. Returns `None` if not found.
    async fn load_session(&self, id: &str) -> Result<Option<Session>>;

    /// Delete a session by ID. Returns `true` if it existed.
    async fn delete_session(&self, id: &str) -> Result<bool>;

    /// Delete multiple sessions in a single operation.
    ///
    /// Used for bulk cleanup (e.g., on app uninstall).
    async fn delete_sessions(&self, ids: &[&str]) -> Result<bool>;

    /// Find all sessions associated with a given shop domain.
    async fn find_sessions_by_shop(&self, shop: &str) -> Result<Vec<Session>>;
}

/// Thread-safe in-memory session store.
///
/// **Do not use in production** — sessions are not persisted between restarts.
/// Suitable for unit tests, integration tests, and local development.
#[derive(Default, Clone)]
pub struct MemorySessionStore {
    sessions: Arc<RwLock<HashMap<String, Session>>>,
}

impl MemorySessionStore {
    /// Create a new empty in-memory store.
    pub fn new() -> Self {
        Self::default()
    }
}

#[async_trait]
impl SessionStore for MemorySessionStore {
    async fn store_session(&self, session: Session) -> Result<bool> {
        let mut store = self.sessions.write().await;
        store.insert(session.id.clone(), session);
        Ok(true)
    }

    async fn load_session(&self, id: &str) -> Result<Option<Session>> {
        let store = self.sessions.read().await;
        Ok(store.get(id).cloned())
    }

    async fn delete_session(&self, id: &str) -> Result<bool> {
        let mut store = self.sessions.write().await;
        Ok(store.remove(id).is_some())
    }

    async fn delete_sessions(&self, ids: &[&str]) -> Result<bool> {
        let mut store = self.sessions.write().await;
        for id in ids {
            store.remove(*id);
        }
        Ok(true)
    }

    async fn find_sessions_by_shop(&self, shop: &str) -> Result<Vec<Session>> {
        let store = self.sessions.read().await;
        Ok(store.values().filter(|s| s.shop == shop).cloned().collect())
    }
}
