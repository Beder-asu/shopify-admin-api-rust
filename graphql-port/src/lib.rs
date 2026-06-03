//! Shopify Admin GraphQL API SDK for Rust
//!
//! Provides a type-safe, idiomatic Rust client for the Shopify Admin GraphQL API
//! with automatic rate-limit retries (both HTTP 429 and GraphQL-level Throttled),
//! Relay-style Connection support, and strongly-typed domain models.
//!
//! # Example
//!
//! ```rust,no_run
//! use shopify_graphql_api::{Session, Client};
//! use shopify_graphql_api::graphql::Connection;
//! use shopify_graphql_api::models::Product;
//! use serde::Deserialize;
//!
//! #[derive(Deserialize, Debug)]
//! struct ProductsResponse {
//!     products: Connection<Product>,
//! }
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let session = Session::new("my-store.myplatform.com", "shpat_xxx")
//!         .with_api_version("2026-01");
//!     let client = Client::new(session);
//!
//!     let data = client.graphql_data::<ProductsResponse, _>(
//!         "query { products(first: 10) { edges { cursor node { id title } } pageInfo { hasNextPage } } }",
//!         None::<&()>,
//!     ).await?;
//!
//!     println!("First product: {:?}", data.products.edges[0].node.title);
//!     Ok(())
//! }
//! ```

#![deny(unsafe_code)]

pub mod client;
pub mod generated;
pub mod graphql;
pub mod models;

pub use client::Client;
pub use shopify_core::error::{Result, ShopifyError};
pub use shopify_core::session::Session;
