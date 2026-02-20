# Shopify REST Admin API - Rust SDK

A Rust port of the Shopify REST Admin API SDK (API version 2026-01).

## Features

- **Full REST API coverage**: All 68 Admin REST resources ported
- **Async/await**: Built on `tokio` runtime with `reqwest` HTTP client
- **Type-safe**: Full Rust type definitions with `serde` serialization
- **Pagination support**: Cursor-based pagination with `FindAllResponse`
- **Error handling**: Comprehensive error types with `thiserror`

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
shopify-api = { path = "./port" }
tokio = { version = "1.0", features = ["full"] }
```

## Quick Start

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
