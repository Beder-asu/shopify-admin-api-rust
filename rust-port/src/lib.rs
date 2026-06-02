//! Shopify-compatible Admin API SDK for Rust
//!
//! Provides a type-safe, idiomatic Rust SDK equivalent to `@shopify/shopify-api`
//! for building Shopify-like platforms. Covers the full Admin REST API (73 resources)
//! plus GraphQL, OAuth, webhooks, and session management.
//!
//! # Quick Start
//!
//! ```rust,no_run
//! use shopify_admin_api::{ShopifyApp, ConfigParams, MemorySessionStore};
//! use shopify_admin_api::auth::BeginParams;
//! use shopify_admin_api::resources::Product;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // 1. Initialize the SDK (mirrors shopifyApi() in TypeScript)
//!     let shopify = ShopifyApp::new(
//!         ConfigParams {
//!             api_key: "your_client_id".into(),
//!             api_secret_key: "your_secret".into(),
//!             host_name: "app.yourplatform.com".into(),
//!             api_version: "2026-01".into(),
//!             ..Default::default()
//!         },
//!         MemorySessionStore::new(),
//!     )?;
//!
//!     // 2. Begin OAuth for a merchant
//!     let oauth_state = shopify.auth.begin(BeginParams {
//!         shop: "merchant.yourplatform.com".into(),
//!         callback_path: "/auth/callback".into(),
//!         is_online: false,
//!     })?;
//!     // → Redirect merchant to oauth_state.redirect_url
//!
//!     // 3. After callback: load session and make API calls
//!     // let session = shopify.session.load_session("offline_merchant.yourplatform.com").await?;
//!     // let client = shopify.rest_client(&session.unwrap());
//!     // let products = Product::all(&client, Default::default()).await?;
//!
//!     Ok(())
//! }
//! ```

#![deny(unsafe_code)]

// ── Core modules ────────────────────────────────────────────────────────────────
pub mod base;
pub mod client;
pub mod config;
pub mod error;
pub mod logger;
pub mod resources;
pub mod session;
pub mod session_manager;
pub mod session_store;

// ── Feature modules ─────────────────────────────────────────────────────────────
pub mod auth;
pub mod webhooks;

// ── Top-level entry point ────────────────────────────────────────────────────────
pub mod shopify;

// ── Re-exports (public API surface) ─────────────────────────────────────────────
pub use base::{CountParams, CountResponse, FieldsParam, FindAllResponse, ListParams};
pub use client::{ApiResponse, Client, PageInfo};
pub use config::{Config, ConfigParams, LogConfig, LogLevel, Scheme};
pub use error::{Result, ShopifyError};
pub use session::Session;
pub use session_store::{MemorySessionStore, SessionStore};
pub use shopify::ShopifyApp;
pub use auth::AuthScopes;
