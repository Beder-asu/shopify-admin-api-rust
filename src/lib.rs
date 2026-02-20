//! Shopify Admin REST API Client for Rust
//! 
//! This crate provides a type-safe Rust client for the Shopify Admin REST API (version 2026-01).
//! 
//! # Example
//! 
//! ```rust,no_run
//! use shopify_admin_api::{Session, Client, resources::Customer};
//! 
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let session = Session::new("my-shop.myshopify.com", "access_token");
//!     let client = Client::new(session);
//!     
//!     // Find a customer
//!     let customer = Customer::find(&client, 123456789).await?;
//!     
//!     // List all customers
//!     let customers = Customer::all(&client, Default::default()).await?;
//!     
//!     Ok(())
//! }
//! ```

pub mod base;
pub mod error;
pub mod session;
pub mod client;
pub mod resources;

pub use base::*;
pub use error::*;
pub use session::Session;
pub use client::Client;

/// API Version constant
pub const API_VERSION: &str = "2026-01";
