# GraphQL Port Implementation Guide

> **Audience:** Developers familiar with general programming concepts but not necessarily Rust.
> This document explains the full architecture and implementation of the Shopify Admin GraphQL API SDK, a standalone Rust crate for querying Shopify's GraphQL endpoint with automatic rate limiting.

---

## Table of Contents

1. [What This Port Does](#1-what-this-port-does)
2. [How It Differs From the REST Port](#2-how-it-differs-from-the-rest-port)
3. [High-Level Architecture](#3-high-level-architecture)
4. [Project Structure](#4-project-structure)
5. [Module Walkthrough](#5-module-walkthrough)
6. [Request Lifecycle](#6-request-lifecycle)
7. [Rate Limiting — The Standout Feature](#7-rate-limiting--the-standout-feature)
8. [Using the Client](#8-using-the-client)
9. [Error Handling](#9-error-handling)
10. [Design Decisions & Trade-offs](#10-design-decisions--trade-offs)

---

## 1. What This Port Does

This crate (`shopify-graphql-api`) is a lightweight Rust client for Shopify's Admin **GraphQL** API. You provide a GraphQL query string and a response type — the client handles authentication, rate limiting, and deserialization:

```
// You write the query and define what the response looks like:
let query = "query { shop { name } }";
let data: ShopResponse = client.graphql_data(query, None::<&()>).await?;
println!("{}", data.shop.name);
```

Unlike the REST port (which pre-defines 73 resource types), the GraphQL port is a **"bring your own query"** client — it sends whatever query you write and deserializes into whatever struct you define.

---

## 2. How It Differs From the REST Port

| Aspect | REST Port | GraphQL Port |
|---|---|---|
| **Approach** | Pre-built methods per resource (`Product::find()`) | Generic client for any query |
| **Type definitions** | 73 resource structs built-in | 12 convenience models; you define the rest |
| **Endpoints** | One URL per resource (`/products.json`, `/orders.json`) | Single URL (`/graphql.json`) for everything |
| **Rate limiting** | HTTP 429 + `Retry-After` header only | HTTP 429 **AND** GraphQL-level `"Throttled"` with cost-based backoff |
| **Pagination** | `Link` header cursors | Relay-style `Connection` / `Edge` / `PageInfo` |
| **Config** | Full `ConfigParams` → `Config` validation | Simple `Session` struct (no separate config) |
| **Authentication** | Shares `ShopifyApp` with OAuth, webhooks, sessions | Standalone — just needs shop domain + access token |

---

## 3. High-Level Architecture

```
┌──────────────────────────────────────────────────────┐
│                     Session                           │
│  • shop domain ("my-store.myshopify.com")            │
│  • access_token ("shpat_xxx")                        │
│  • api_version ("2026-01")                           │
│  • host_scheme ("https")                             │
│                                                      │
│  Builds: https://my-store.../admin/api/2026-01/      │
│          graphql.json                                │
└──────────────────┬───────────────────────────────────┘
                   │ passed to
┌──────────────────▼───────────────────────────────────┐
│                     Client                            │
│                                                      │
│  graphql(query, variables)                           │
│  └── Returns full GraphQLResponse<T>                 │
│      (data + errors + extensions)                    │
│                                                      │
│  graphql_data(query, variables)                      │
│  └── Returns just T (errors become Err)              │
│                                                      │
│  Rate Limit Engine:                                  │
│  ├── HTTP 429 → read Retry-After → sleep → retry    │
│  └── GraphQL "Throttled" → read cost.restoreRate     │
│      → compute dynamic backoff → sleep → retry       │
└──────────────────┬───────────────────────────────────┘
                   │ deserializes into
┌──────────────────▼───────────────────────────────────┐
│              GraphQLResponse<T>                       │
│  ┌──────────┐  ┌──────────────┐  ┌───────────────┐  │
│  │ data: T  │  │ errors: Vec  │  │ extensions:   │  │
│  │ (your    │  │ (GraphQL     │  │  Cost {       │  │
│  │  struct) │  │  errors)     │  │   restoreRate │  │
│  │          │  │              │  │   available   │  │
│  └──────────┘  └──────────────┘  └───────────────┘  │
└──────────────────────────────────────────────────────┘
```

---

## 4. Project Structure

```
graphql-port/
├── Cargo.toml           # Dependencies and metadata
├── src/
│   ├── lib.rs           # Crate root — declares modules, re-exports
│   ├── client.rs        # GraphQL HTTP client with dual rate limiting
│   ├── session.rs       # Session struct (shop + token + version)
│   ├── error.rs         # Error types
│   ├── graphql.rs       # Core GraphQL types (Response, Connection, Edge)
│   └── models.rs        # Pre-built domain models (Product, Order, etc.)
│
└── tests/
    └── live_test.rs     # Live Shopify API test (needs credentials)
```

Only **6 source files** — intentionally minimal. The power is in the client's rate limiting logic.

---

## 5. Module Walkthrough

### 5.1 `session.rs` — Session

**Purpose:** Hold the credentials and build the GraphQL endpoint URL.

```rust
let session = Session::new("my-store.myshopify.com", "shpat_xxx")
    .with_api_version("2026-01");    // optional, defaults to "2026-01"

// Internally produces:
// https://my-store.myshopify.com/admin/api/2026-01/graphql.json
```

Smart URL handling: if you pass a full URL like `"https://my-store.myshopify.com"`, it extracts just the host portion automatically.

**Compared to REST port:** The REST port uses a full `Config` object with validation, secret keys, OAuth settings, etc. The GraphQL port is simpler — just domain, token, and version.

### 5.2 `client.rs` — The GraphQL Client

**Purpose:** Send GraphQL queries with automatic retries on rate limits.

Two public methods:

```
graphql(query, variables) → GraphQLResponse<T>
  Returns the FULL envelope: { data, errors, extensions }
  Use when you want to inspect errors alongside partial data.

graphql_data(query, variables) → T
  Extracts just the data field.
  If there's no data but there are errors → returns Err.
  If there's neither → returns Err(NoData).
  Use for simple queries where you just want the result.
```

### 5.3 `graphql.rs` — Core GraphQL Types

**Purpose:** Define the standard GraphQL response envelope and Relay pagination types.

**Request envelope** (what gets sent to Shopify):
```json
{
    "query": "query { shop { name } }",
    "variables": null
}
```

**Response envelope** (what comes back):
```json
{
    "data": { "shop": { "name": "My Store" } },
    "errors": null,
    "extensions": {
        "cost": {
            "requestedQueryCost": 1,
            "actualQueryCost": 1,
            "throttleStatus": {
                "maximumAvailable": 1000,
                "currentlyAvailable": 999,
                "restoreRate": 50
            }
        }
    }
}
```

**Relay pagination types** (for list queries):

```
Connection<T>               A page of results
├── edges: Vec<Edge<T>>     The items
│   ├── cursor: String      Position marker for this item
│   └── node: T             The actual data
└── pageInfo: PageInfo
    ├── hasNextPage: bool
    └── hasPreviousPage: bool
```

Usage example:
```rust
#[derive(Deserialize)]
struct Response {
    products: Connection<Product>,  // A page of products
}

// Access items:
for edge in response.products.edges {
    println!("Product: {}", edge.node.title);
}

// Check for more pages:
if response.products.page_info.has_next_page {
    // Send query again with `after: "last_cursor"` variable
}
```

### 5.4 `models.rs` — Pre-built Domain Models

**Purpose:** Convenience structs so you don't have to define common types yourself.

Available models:
- `Product`, `ProductVariant` — Products with variants, images
- `Customer` — Customer profiles with addresses
- `Order`, `LineItem` — Orders with line items and money values
- `Fulfillment`, `TrackingInfo` — Shipping fulfillments
- `Metafield` — Custom metadata
- `Money`, `MoneyBag`, `Image`, `MailingAddress` — Supporting types

**Important:** These are optional conveniences. You can query any Shopify GraphQL type by defining your own struct:

```rust
// This works even though "DiscountNode" isn't in models.rs:
#[derive(Deserialize)]
struct DiscountResponse {
    discount_nodes: Connection<DiscountNode>,
}

#[derive(Deserialize)]
struct DiscountNode {
    id: String,
    discount: serde_json::Value,  // flexible for complex nested types
}
```

### 5.5 `error.rs` — Error Types

```
ShopifyError (enum)
├── Request(reqwest::Error)              ← Network/transport failure
├── Json(serde_json::Error)              ← JSON parse failure
├── UrlParse(url::ParseError)            ← Bad URL construction
├── GraphQLError(Vec<GraphQLErrorDetail>) ← GraphQL execution errors
├── NoData                                ← Response had no data or errors
├── ValidationError(String)              ← Input validation failure
├── RateLimited { retry_after }          ← Rate limit after all retries exhausted
└── ApiError { status, message }         ← HTTP-level errors (401, 403, 500, etc.)
```

Each `GraphQLErrorDetail` contains:
- `message` — human-readable error text
- `locations` — line/column in your query where the error occurred
- `path` — the response field path where the error happened
- `extensions` — optional extra data (error codes, etc.)

---

## 6. Request Lifecycle

Here's the complete path of a GraphQL query:

```
1. You call: client.graphql::<MyResponse, _>(query, Some(&variables)).await?

2. Client builds the request body:
   { "query": "...", "variables": {...} }

3. Client builds the URL from session:
   https://my-store.myshopify.com/admin/api/2026-01/graphql.json

4. Client attaches headers:
   ├── X-Shopify-Access-Token: shpat_xxx
   └── Content-Type: application/json

5. reqwest sends HTTP POST

6. Response handling (dual rate limit check):

   ┌─ HTTP Status ─────────────────────────────────────────┐
   │                                                        │
   │  429? ── Yes ──▶ Read Retry-After header               │
   │   │              Sleep for that duration                │
   │   │              Retry (up to 3×)                      │
   │   │              If exhausted → Err(RateLimited)       │
   │   │                                                    │
   │  200? ── Yes ──▶ Parse JSON into GraphQLResponse<T>    │
   │   │              │                                     │
   │   │              ├── Check for "Throttled" in errors   │
   │   │              │   Yes ──▶ Read extensions.cost      │
   │   │              │           .throttleStatus.restoreRate│
   │   │              │           Compute: (1/rate) + 0.5s  │
   │   │              │           Sleep, retry (up to 3×)   │
   │   │              │                                     │
   │   │              └── No throttle ──▶ Return Ok(response)│
   │   │                                                    │
   │  Other ── Map to ShopifyError::ApiError ───▶ Return Err│
   └────────────────────────────────────────────────────────┘

7. If using graphql_data() instead:
   ├── data present     → return Ok(data)
   ├── errors only      → return Err(GraphQLError)
   └── neither          → return Err(NoData)
```

---

## 7. Rate Limiting — The Standout Feature

This is the most sophisticated part of the entire project. Shopify has **two layers** of rate limiting for GraphQL, and this client handles both automatically.

### Layer 1: HTTP 429 (Too Many Requests)

Standard HTTP rate limiting. Shopify returns a `429` status code with a `Retry-After` header:

```
HTTP/1.1 429 Too Many Requests
Retry-After: 2.0
```

The client reads the header, sleeps for that duration, and retries.

### Layer 2: GraphQL Cost Throttling

Even when the HTTP status is `200 OK`, Shopify can return a **GraphQL-level throttle** inside the response body:

```json
{
    "errors": [{"message": "Throttled"}],
    "extensions": {
        "cost": {
            "requestedQueryCost": 502,
            "actualQueryCost": null,
            "throttleStatus": {
                "maximumAvailable": 1000.0,
                "currentlyAvailable": 0.0,
                "restoreRate": 50.0
            }
        }
    }
}
```

The client:
1. Detects `"throttled"` in any error message
2. Reads `restoreRate` from the cost extensions (points restored per second)
3. Computes optimal wait time: `(1 / restoreRate) + 0.5 seconds`
4. Sleeps for that duration
5. Retries the query

**Why this matters:** Most basic GraphQL clients crash or return errors on throttling. This client transparently handles it — your application code never sees the throttle, it just waits a bit longer.

```
                    Shopify Cost Budget
     ┌──────────────────────────────────────┐
     │ Maximum: 1000 points                 │
     │ Restore rate: 50 points/second       │
     │                                      │
     │ Your query costs 200 points          │
     │ Available: 0 points (throttled!)     │
     │                                      │
     │ Client computes:                     │
     │   wait = (1/50) + 0.5 = 0.52 seconds │
     │   → sleeps 520ms, then retries       │
     └──────────────────────────────────────┘
```

---

## 8. Using the Client

### Basic Query

```rust
use shopify_graphql_api::{Session, Client};
use serde::Deserialize;

#[derive(Deserialize)]
struct ShopResponse {
    shop: ShopData,
}

#[derive(Deserialize)]
struct ShopData {
    name: String,
}

let session = Session::new("my-store.myshopify.com", "shpat_xxx");
let client = Client::new(session);

let data: ShopResponse = client.graphql_data(
    "query { shop { name } }",
    None::<&()>,
).await?;

println!("Shop: {}", data.shop.name);
```

### Query with Variables

```rust
use serde::Serialize;

#[derive(Serialize)]
struct Vars {
    first: i32,
}

let query = r#"
    query($first: Int!) {
        products(first: $first) {
            edges { node { id title } }
            pageInfo { hasNextPage }
        }
    }
"#;

let data = client.graphql_data::<ProductsResponse, _>(
    query,
    Some(&Vars { first: 5 }),
).await?;
```

### Handling Partial Data + Errors

```rust
// Use graphql() (not graphql_data) to see both data AND errors:
let response = client.graphql::<MyResponse, _>(query, None::<&()>).await?;

if let Some(data) = response.data {
    // Process the data that succeeded
}

if let Some(errors) = response.errors {
    for err in errors {
        println!("Error: {} at {:?}", err.message, err.path);
    }
}

// Inspect rate limit budget:
if let Some(ext) = response.extensions {
    if let Some(cost) = ext.cost {
        println!("Query cost: {}", cost.requested_query_cost);
        println!("Budget remaining: {}", cost.throttle_status.currently_available);
    }
}
```

---

## 9. Error Handling

```rust
match client.graphql_data::<MyResponse, _>(query, None::<&()>).await {
    Ok(data) => {
        // Success — use data
    }
    Err(ShopifyError::RateLimited { retry_after }) => {
        // All 3 retries exhausted — Shopify is very busy
        println!("Try again in {}s", retry_after);
    }
    Err(ShopifyError::GraphQLError(errors)) => {
        // Query had errors and no data
        for e in errors {
            println!("GraphQL error: {}", e.message);
        }
    }
    Err(ShopifyError::NoData) => {
        // Response was completely empty (unusual)
    }
    Err(e) => {
        // Network error, JSON parse error, etc.
        println!("Error: {}", e);
    }
}
```

---

## 10. Design Decisions & Trade-offs

| Decision | Rationale | Trade-off |
|---|---|---|
| No `Config` object (just `Session`) | GraphQL client doesn't need OAuth, webhooks, or session storage — keep it simple | Can't share config with REST port |
| Raw query strings | Maximum flexibility — any valid GraphQL works | No compile-time query validation (planned via `graphql_client`) |
| Two client methods (`graphql` vs `graphql_data`) | `graphql` for advanced use (partial data + errors); `graphql_data` for simple use | Two methods to learn instead of one |
| Cost-based backoff formula `(1/rate) + 0.5s` | Matches the time for 1 point to restore, plus a safety buffer | Slightly conservative (could wait less) |
| Generic `T` for response type | Works with any user-defined struct | User must define structs that match their query |
| 12 pre-built models in `models.rs` | Covers the most common Shopify types | Doesn't cover all ~200 Shopify GraphQL types |
| `MAX_RETRIES = 3` | Balances reliability with not hanging forever | May not be enough for sustained heavy load |

---

*This document covers the GraphQL port implementation as of API version 2026-01.*
