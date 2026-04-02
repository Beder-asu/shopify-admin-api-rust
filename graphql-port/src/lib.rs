//! Shopify Admin GraphQL API Client for Rust
//! 
//! This crate provides a type-safe Rust client for the Shopify Admin GraphQL API (version 2026-01).
//! 
//! # Example
//! 
//! ```rust,no_run
//! use shopify_graphql_api::{Session, Client, graphql::Connection, models::Product};
//! use serde::Deserialize;
//! 
//! #[derive(Deserialize, Debug)]
//! struct ProductsResponse {
//!     products: Connection<Product>,
//! }
//! 
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let session = Session::new("my-shop.myshopify.com", "access_token");
//!     let client = Client::new(session);
//!     
//!     // Issue a raw GraphQL Query using the strongly-typed Product model
//!     let query = r#"
//!         query {
//!             products(first: 10) {
//!                 pageInfo {
//!                     hasNextPage
//!                     hasPreviousPage
//!                 }
//!                 edges {
//!                     cursor
//!                     node {
//!                         id
//!                         title
//!                         handle
//!                     }
//!                 }
//!             }
//!         }
//!     "#;
//! 
//!     let response: ProductsResponse = client.graphql(query, None::<&()>).await?;
//!     println!("First product: {}", response.products.edges[0].node.title);
//! 
//!     Ok(())
//! }
//! ```

pub mod error;
pub mod session;
pub mod client;
pub mod graphql;
pub mod models;

pub use error::*;
pub use session::Session;
pub use client::Client;

/// API Version constant
pub const API_VERSION: &str = "2026-01";
