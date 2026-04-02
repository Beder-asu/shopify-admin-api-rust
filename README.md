# Shopify Admin API - Rust Ports

🚧 **Work in Progress** 🚧

A mono-repo containing Rust ports for both the **GraphQL** and **REST** Shopify Admin APIs (API version 2026-01). This is an **ongoing project** under active development.

## Project Scope

This project is aimed at providing a 1-to-1, strongly typed Rust equivalent to the official Shopify framework SDKs. Because Shopify operates both REST and GraphQL APIs side-by-side, this mono-repo is split into two modular crates:
- **`rust-port/`**: A comprehensive wrapper around the Shopify Admin **REST API**, featuring full implementations of all 73 standard endpoints (Products, Orders, Customers, Fulfillments, etc.).
- **`graphql-port/`**: A lightweight, robust wrapper for the Shopify **GraphQL Admin API** featuring built-in cost-based rate limiting, pagination components, and strong root error handling. 

## What is a Wrapper? (How this links to Shopify)

This repository serves as an SDK (Software Development Kit) and **wrapper** around the official Shopify Admin HTTP APIs. 

Under the hood, Shopify's core data exists on their servers and is accessed over the internet via HTTP URLs (e.g., `https://my-shop.myshopify.com/admin/api/2026-01/...`). To communicate with Shopify natively, developers have to manually write boilerplate code to open HTTP connections, inject authentication tokens, serialize abstract JSON strings, and manually delay their applications when server quotas are exceeded.

This wrapper acts as a powerful translation layer between your Rust application and Shopify's servers to eliminate that manual labor. 

**Why is it useful?**
1. **Type Safety:** You interact with concrete Rust `structs` instead of guessing the shape of raw, untyped JSON objects. If you misspell a property, the Rust compiler will catch it before the app ever runs.
2. **Auto-managed Networking:** Features like HTTP headers (`X-Shopify-Access-Token`), routing, and data parsing are abstracted away into clean functions.
3. **Smart Rate Limiting:** Shopify aggressively limits how fast you can make requests. This wrapper intercepts `HTTP 429 Too Many Requests` status codes and parses Shopify's GraphQL "Call Cost" metrics to dynamically pause and retry your requests—keeping your app from crashing.
4. **Developer Velocity:** You can build complex systems exponentially faster using streamlined, native-feeling methods like `Product::find()` rather than hand-crafting convoluted HTTP requests from scratch.

## 🚧 Project Status (Work In Progress)

This SDK is currently under active development. While the foundation is extremely solid, there are key roadmap items remaining before v1.0:
1. **GraphQL Schema Generation:** The GraphQL port currently requires manual struct definition for queries. We are planning to integrate `graphql_client` to auto-generate models completely safely at compile-time directly from `.graphql` files.
2. **Partial GraphQL Payloads:** Updating the GraphQL port to return partial `data` alongside localized Field `errors` instead of failing fast.
3. **Ergonomic Request Builders:** Migrating REST parameters away from raw structs to more fluent builder patterns (e.g., `Product::list().limit(50).send()`).
4. **Comprehensive Test Coverage:** Expanding unit scenarios for the 73 REST endpoints.

Contributions, suggestions, and bug reports are highly welcome!

## Features

- **Full REST API coverage**: All 68 Admin REST resources ported
- **Async/await**: Built on `tokio` runtime with `reqwest` HTTP client
- **Type-safe**: Full Rust type definitions with `serde` serialization
- **Pagination support**: Cursor-based pagination with `FindAllResponse`
- **Error handling**: Comprehensive error types with `thiserror`

## Installation

Depending on which API you want to use, add the respective port to your `Cargo.toml`:

### REST API
```toml
[dependencies]
shopify-api = { path = "./rust-port" }
tokio = { version = "1.0", features = ["full"] }
```

### GraphQL API
```toml
[dependencies]
shopify-graphql-api = { path = "./graphql-port" }
tokio = { version = "1.0", features = ["full"] }
```

## Usage Examples

### REST API Quick Start

```rust
use shopify_api::{Client, Session};
use shopify_api::resources::{Product, ProductListParams};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a session
    let session = Session::new("my-store.myshopify.com", "access_token_here");
    
    // Create a client
    let client = Client::new(&session);
    
    // List products
    let params = ProductListParams::default();
    let response = Product::all(&client, params).await?;
    
    for product in response.data {
        println!("Product: {:?}", product.title);
    }
    
    // Get a single product
    if let Some(product) = Product::find(&client, 123456789).await? {
        println!("Found product: {:?}", product.title);
    }
    
    // Create a product
    let new_product = Product {
        title: Some("New Product".to_string()),
        body_html: Some("<p>Product description</p>".to_string()),
        vendor: Some("My Company".to_string()),
        ..Default::default()
    };
    let created = Product::create(&client, &new_product).await?;
    println!("Created product with ID: {:?}", created.id);
    
    Ok(())
}
```

### GraphQL API Quick Start

```rust
use shopify_graphql_api::{Session, Client, graphql::Connection, models::Product};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct ProductsResponse {
    products: Connection<Product>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let session = Session::new("my-shop.myshopify.com", "access_token_here");
    let client = Client::new(session);
    
    let query = r#"
        query {
            products(first: 5) {
                pageInfo { hasNextPage }
                edges {
                    node {
                        id
                        title
                        vendor
                    }
                }
            }
        }
    "#;

    // Issue the strongly-typed query. Rate limiting constraints
    // (HTTP 429 & GraphQL Cost Throttling) are handled automatically!
    let response: ProductsResponse = client.graphql(query, None::<&()>).await?;
    
    for edge in response.products.edges {
        println!("GraphQL Product: {}", edge.node.title);
    }

    Ok(())
}
```

## Available Resources

### Products
- `Product` - Products management
- `Variant` - Product variants
- `Image` - Product images
- `CustomCollection` - Custom collections
- `SmartCollection` - Smart collections
- `Collection` - Collections (read-only)
- `Collect` - Product-collection relationships

### Orders
- `Order` - Orders management
- `Transaction` - Order transactions
- `Refund` - Refunds
- `DraftOrder` - Draft orders
- `OrderRisk` - Fraud analysis

### Customers
- `Customer` - Customer management
- `CustomerAddress` - Customer addresses (via Customer methods)

### Inventory
- `InventoryItem` - Inventory items
- `InventoryLevel` - Stock levels
- `Location` - Shop locations

### Fulfillment
- `Fulfillment` - Fulfillments
- `FulfillmentOrder` - Fulfillment orders
- `FulfillmentRequest` - Fulfillment requests
- `FulfillmentEvent` - Tracking events
- `FulfillmentService` - Third-party fulfillment services
- `CarrierService` - Custom carrier services
- `AssignedFulfillmentOrder` - Assigned fulfillment orders
- `CancellationRequest` - Cancellation requests
- `LocationsForMove` - Available move locations

### Billing
- `ApplicationCharge` - One-time app charges
- `RecurringApplicationCharge` - Subscription charges
- `UsageCharge` - Usage-based billing
- `ApplicationCredit` - App credits

### Shipping
- `ShippingZone` - Shipping zones

### Checkout
- `Checkout` - Checkouts
- `AbandonedCheckout` - Abandoned checkouts
- `Payment` - Payments
- `PaymentGateway` - Payment gateways
- `PaymentTransaction` - Payment transactions

### Discounts
- `PriceRule` - Price rules
- `DiscountCode` - Discount codes

### Gift Cards
- `GiftCard` - Gift cards
- `GiftCardAdjustment` - Gift card adjustments

### Online Store
- `Theme` - Themes
- `Asset` - Theme assets
- `Blog` - Blogs
- `Article` - Blog articles
- `Page` - Pages
- `Redirect` - URL redirects
- `ScriptTag` - Script tags
- `Comment` - Blog comments

### Marketing
- `MarketingEvent` - Marketing events

### Metafields
- `Metafield` - Metafields (for any resource)

### Sales Channels
- `ProductListing` - Product listings
- `CollectionListing` - Collection listings
- `ProductResourceFeedback` - Product feedback
- `ResourceFeedback` - General resource feedback
- `StorefrontAccessToken` - Storefront API tokens

### Store Info
- `Shop` - Shop information
- `Policy` - Legal policies
- `Country` - Shipping countries
- `Province` - Provinces/states
- `Currency` - Enabled currencies

### Shopify Payments
- `Balance` - Account balance
- `Payout` - Payouts
- `Dispute` - Disputes/chargebacks
- `DisputeEvidence` - Dispute evidence
- `DisputeFileUpload` - Dispute file uploads
- `TenderTransaction` - Tender transactions

### Webhooks
- `Webhook` - Webhook subscriptions

### Events
- `Event` - Store events

### Access
- `AccessScope` - OAuth scopes
- `User` - Staff accounts

### Mobile
- `MobilePlatformApplication` - Mobile app configuration
- `ApplePayCertificate` - Apple Pay certificates

### Deprecated APIs
- `DeprecatedApiCall` - Deprecated API usage

## Pagination

```rust
use shopify_api::resources::{Order, OrderListParams};

async fn paginate_orders(client: &Client) -> Result<(), ShopifyError> {
    let mut params = OrderListParams { limit: Some(50), ..Default::default() };
    
    loop {
        let response = Order::all(&client, params.clone()).await?;
        
        for order in response.data {
            println!("Order: {:?}", order.name);
        }
        
        match response.next_page {
            Some(cursor) => params.page_info = Some(cursor),
            None => break,
        }
    }
    
    Ok(())
}
```

## Error Handling

```rust
use shopify_api::error::ShopifyError;

match Product::find(&client, 123).await {
    Ok(Some(product)) => println!("Found: {:?}", product.title),
    Ok(None) => println!("Product not found"),
    Err(ShopifyError::RateLimited { retry_after }) => {
        println!("Rate limited, retry after {:?}", retry_after);
    }
    Err(ShopifyError::HttpError { status, message }) => {
        println!("HTTP error {}: {}", status, message);
    }
    Err(e) => println!("Error: {}", e),
}
```

## License

MIT

## Credits

Ported from the official [Shopify App JS](https://github.com/Shopify/shopify-app-js) SDK.
