# REST Port Implementation Guide

> **Audience:** Developers familiar with general programming concepts but not necessarily Rust.
> This document explains the full architecture and implementation of the Shopify Admin REST API SDK, ported from the official TypeScript `@shopify/shopify-api` package into Rust.

---

## Table of Contents

1. [What This Port Does](#1-what-this-port-does)
2. [High-Level Architecture](#2-high-level-architecture)
3. [Project Structure](#3-project-structure)
4. [Rust Concepts You'll Need](#4-rust-concepts-youll-need)
5. [Module Walkthrough](#5-module-walkthrough)
6. [Request Lifecycle](#6-request-lifecycle)
7. [Authentication & Security](#7-authentication--security)
8. [Session Management](#8-session-management)
9. [Resource Pattern](#9-resource-pattern-73-endpoints)
10. [Error Handling](#10-error-handling)
11. [Design Decisions & Trade-offs](#11-design-decisions--trade-offs)

---

## 1. What This Port Does

This crate (`shopify-admin-api`) is a **Rust SDK** that wraps Shopify's Admin REST API. Instead of manually constructing HTTP requests, parsing JSON, and handling rate limits, developers use typed Rust functions:

```
// Without the SDK (manual approach):
// 1. Build URL: https://my-shop.myshopify.com/admin/api/2026-01/products.json
// 2. Add header: X-Shopify-Access-Token: shpat_xxx
// 3. Send GET request
// 4. Parse JSON string into usable data
// 5. Handle 429 rate limit errors manually
// 6. Handle pagination via Link headers

// With the SDK:
let products = Product::all(&client, Default::default()).await?;
```

The SDK handles networking, authentication, serialization, rate limiting, and pagination automatically.

---

## 2. High-Level Architecture

```
┌─────────────────────────────────────────────────────┐
│                    ShopifyApp                        │
│  (Top-level entry point — initialized once at startup)│
│                                                      │
│  ┌──────────┐  ┌──────────────┐  ┌───────────────┐  │
│  │   Auth   │  │   Webhooks   │  │    Session     │  │
│  │  Module  │  │    Module    │  │    Manager     │  │
│  │          │  │              │  │                │  │
│  │ • begin  │  │ • validate   │  │ • store/load   │  │
│  │ • callback│ │   hmac      │  │ • get_active   │  │
│  └──────────┘  └──────────────┘  └───────┬───────┘  │
│                                          │          │
│  ┌───────────────────────────────────────┘          │
│  │  SessionStore (trait)                             │
│  │  └── MemorySessionStore (built-in)               │
│  │  └── YourCustomStore (you implement)              │
│  └──────────────────────────────────────────────────│
│                                                      │
│  ┌──────────────────────────────────────────────┐   │
│  │              REST Client                      │   │
│  │  • get, post, put, delete                     │   │
│  │  • Automatic 429 retry with Retry-After       │   │
│  │  • Link header pagination parsing             │   │
│  └──────────────┬───────────────────────────────┘   │
│                 │                                    │
└─────────────────┼────────────────────────────────────┘
                  │
    ┌─────────────┴─────────────┐
    │     73 Resource Modules    │
    │  Product, Order, Customer  │
    │  Fulfillment, Webhook...   │
    │                            │
    │  Each has:                 │
    │  • Typed struct (fields)   │
    │  • find() / all() / count()│
    │  • create() / update()     │
    │  • delete() / save()       │
    └────────────────────────────┘
```

**Data flows top-down:** `ShopifyApp` creates a `Client` from a `Session`, then resource methods (like `Product::find`) use that `Client` to make HTTP requests.

---

## 3. Project Structure

```
rust-port/
├── Cargo.toml              # Dependencies and package metadata
├── src/
│   ├── lib.rs              # Crate root — declares modules, public API
│   ├── main.rs             # Stub (unused)
│   │
│   ├── shopify.rs          # ShopifyApp — the top-level entry point
│   ├── config.rs           # Configuration validation (ConfigParams → Config)
│   ├── client.rs           # HTTP client with retry logic
│   ├── error.rs            # Error types (ShopifyError enum)
│   ├── base.rs             # Shared traits and types (Findable, Listable, etc.)
│   ├── logger.rs           # Structured logging
│   │
│   ├── session.rs          # Session struct (offline/online tokens)
│   ├── session_store.rs    # SessionStore trait + MemorySessionStore
│   ├── session_manager.rs  # Higher-level session operations
│   │
│   ├── auth/
│   │   ├── mod.rs          # AuthModule (begin + callback)
│   │   ├── oauth.rs        # OAuth flow, HMAC validation
│   │   ├── nonce.rs        # Cryptographic nonce generation
│   │   └── scopes.rs       # OAuth scope parsing (write_X implies read_X)
│   │
│   ├── webhooks/
│   │   └── mod.rs          # Webhook HMAC validation
│   │
│   └── resources/
│       ├── mod.rs           # Re-exports all 73 resources
│       ├── product.rs       # Product CRUD
│       ├── order.rs         # Order CRUD + cancel/close/open
│       ├── customer.rs      # Customer CRUD + search
│       ├── ... (70 more)
│
└── tests/
    ├── config_tests.rs      # Config validation tests
    ├── crypto_tests.rs      # HMAC and constant-time comparison tests
    ├── client_integration.rs# Mock HTTP server tests
    └── live_test.rs         # Real Shopify API test (needs credentials)
```

---

## 4. Rust Concepts You'll Need

### Ownership & References (`&`)

In Rust, data has a single **owner**. When you pass `&client` to a function, you're lending a read-only reference — the function can use it but doesn't take ownership.

```rust
// &client = "borrow the client, don't take it"
let products = Product::all(&client, params).await?;
// client is still usable here because we only lent a reference
```

### `Option<T>` — Nullable Values

Rust has no `null`. Instead, `Option<T>` is either `Some(value)` or `None`:

```rust
pub title: Option<String>  // Could be Some("My Product") or None
```

### `Result<T, E>` and the `?` Operator

Functions that can fail return `Result<T, E>` — either `Ok(value)` or `Err(error)`. The `?` operator propagates errors automatically:

```rust
let response = client.get("shop.json").await?;
//                                          ^ if this fails, return the error immediately
```

### `async/await`

Like JavaScript/Python async. Functions marked `async` return a future that must be `.await`ed:

```rust
async fn find(client: &Client, id: i64) -> Result<Option<Self>> {
    let response = client.get(&path).await?;  // waits for HTTP response
    Ok(Some(response.data.product))
}
```

### Traits (Interfaces)

Traits are Rust's version of interfaces. They define a contract:

```rust
// Any type implementing SessionStore must provide these methods
trait SessionStore {
    async fn store_session(&self, session: Session) -> Result<bool>;
    async fn load_session(&self, id: &str) -> Result<Option<Session>>;
    async fn delete_session(&self, id: &str) -> Result<bool>;
}
```

### `Arc<T>` — Shared Ownership

`Arc` (Atomic Reference Count) lets multiple parts of the code share the same data safely across threads:

```rust
let config = Arc::new(validated_config);
// Now auth, webhooks, and session_manager can all share this config
// without copying it
```

### Derive Macros

`#[derive(...)]` auto-generates code:

```rust
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Product { ... }
// Debug     → enables println!("{:?}", product)
// Clone     → enables product.clone()
// Serialize → enables converting to JSON
// Deserialize → enables parsing from JSON
// Default   → enables Product::default() (all fields None/empty)
```

---

## 5. Module Walkthrough

### 5.1 `config.rs` — Configuration

**Purpose:** Validate and normalize SDK settings before anything else runs.

**Flow:** Raw `ConfigParams` (user input) → `validate_config()` → Immutable `Config`

```
ConfigParams (user provides)          Config (validated, immutable)
┌──────────────────────────┐         ┌──────────────────────────┐
│ api_key: ""              │──FAIL──▶│ Error: "missing api_key" │
│ api_secret_key: "secret" │         └──────────────────────────┘
│ host_name: "app.com/"    │
│ api_version: "2026-01"   │         ┌──────────────────────────┐
│ ...                      │──OK───▶ │ host_name: "app.com"     │ ← slash trimmed
└──────────────────────────┘         │ host_scheme: Https       │ ← default applied
                                     └──────────────────────────┘
```

Key validation rules:
- `api_secret_key`, `host_name`, `api_version` — always required
- `api_key` — required unless `is_custom_store_app` is true
- `is_custom_store_app` — requires `admin_api_access_token`
- Trailing slashes on `host_name` are automatically stripped

### 5.2 `client.rs` — HTTP Client

**Purpose:** Send authenticated REST API requests with automatic retry on rate limiting.

The client provides 6 HTTP methods: `get`, `get_with_params`, `post`, `put`, `delete`, `delete_with_params`. All follow the same pattern:

```
Request Flow:
                                           ┌─────────┐
    Build URL ──▶ Attach Headers ──▶ Send ──┤ Status? │
                                           └────┬────┘
                                    ┌───────────┼───────────┐
                                    ▼           ▼           ▼
                                200-299       429      400/401/5xx
                                    │           │           │
                              Parse JSON    Sleep for    Map to
                              + pagination  Retry-After  ShopifyError
                              info from     seconds,       │
                              Link header   then retry     ▼
                                    │       (max 3×)    Return Err
                                    ▼           │
                              Return Ok     ────┘
```

**Rate Limit Retry Logic (lines 118-153 in `client.rs`):**

```rust
// Simplified pseudo-code of the retry loop:
loop {
    response = send_request()

    if response.status == 429 {
        if retries >= 3 {
            return Error::RateLimited
        }
        wait_seconds = parse_header("Retry-After") or default 2.0
        sleep(wait_seconds)
        retries += 1
        continue  // try again
    }

    if response.status is 2xx {
        return Ok(parsed_json)
    } else {
        return Err(map_status_to_error(status, body))
    }
}
```

**Pagination:** The client parses `Link` headers from Shopify's response to extract `next` and `previous` page cursors:

```
Link: <https://...?page_info=abc123>; rel="next",
      <https://...?page_info=xyz789>; rel="previous"
```

### 5.3 `session.rs` — Session

**Purpose:** Represent an authenticated shop installation with its access token.

Two session types:

| Type | ID Format | Lifespan | Use Case |
|---|---|---|---|
| **Offline** | `"offline_{shop_domain}"` | Permanent | Background jobs, webhooks |
| **Online** | UUID v4 | Short-lived (expires) | User-facing requests |

Key methods:
- `is_active(scopes, buffer_ms)` — checks token is present, not expired, and scopes match
- `is_expired(buffer_ms)` — returns true if token expires within `buffer_ms` milliseconds
- `is_scope_changed(required)` — returns true if the session's scopes don't cover what's required
- `to_property_array()` / `from_property_array()` — serialize/deserialize for database storage

### 5.4 `session_store.rs` — Storage Backend

**Purpose:** Define a pluggable interface for persisting sessions.

```
                   SessionStore (trait/interface)
                  ┌──────────────────────────────┐
                  │  store_session(session)       │
                  │  load_session(id) → Option    │
                  │  delete_session(id)           │
                  │  delete_sessions(ids[])       │
                  │  find_sessions_by_shop(shop)  │
                  └──────────┬───────────────────┘
                             │
              ┌──────────────┼──────────────┐
              ▼              ▼              ▼
      MemorySessionStore  PostgresStore  RedisStore
      (built-in, dev     (you build)    (you build)
       only, not
       persistent)
```

`MemorySessionStore` uses a thread-safe `HashMap` wrapped in `Arc<RwLock<...>>`, meaning multiple async tasks can read sessions simultaneously, but writes are exclusive.

### 5.5 `auth/oauth.rs` — OAuth Flow

**Purpose:** Implement the full Shopify OAuth authorization code flow.

```
Step 1: begin()
  App ──▶ Build redirect URL with client_id, scopes, state nonce
      ──▶ Return URL + nonce (caller sets cookie)

Step 2: Merchant authorizes in browser
  Shopify ──▶ Redirects to your callback_path with ?code=...&hmac=...&state=...

Step 3: callback()
  App ──▶ Validate state nonce matches cookie (anti-CSRF)
      ──▶ Validate HMAC signature of query params (anti-tampering)
      ──▶ POST to Shopify's /admin/oauth/access_token with the code
      ──▶ Receive access_token, scope, optional user info
      ──▶ Build and return a Session object
```

HMAC validation uses `HMAC-SHA256` with constant-time comparison (`subtle` crate) to prevent timing attacks.

### 5.6 `auth/scopes.rs` — OAuth Scopes

**Purpose:** Parse and compare OAuth permission scopes with implied-scope logic.

Key behavior — **`write_X` implies `read_X`:**

```
App has scopes:     ["write_products", "read_orders"]
Required scopes:    ["read_products", "read_orders"]

Result: ✅ Satisfied
  → "read_products" is covered because "write_products" implies it
  → "read_orders" is explicitly present
```

### 5.7 `webhooks/mod.rs` — Webhook Validation

**Purpose:** Verify that incoming webhook requests genuinely came from Shopify.

```
Incoming webhook HTTP request:
  Body: {"order": {"id": 123, ...}}
  Headers:
    X-Shopify-Hmac-Sha256: <base64-encoded HMAC>
    X-Shopify-Topic: orders/paid
    X-Shopify-Shop-Domain: my-store.myshopify.com
    X-Shopify-API-Version: 2026-01
    X-Shopify-Webhook-Id: unique-id

Validation:
  1. Extract X-Shopify-Hmac-Sha256 header
  2. Compute HMAC-SHA256(api_secret_key, raw_body_bytes)
  3. Base64-encode the result
  4. Constant-time compare with the header value
  5. Extract required headers (topic, domain, api_version, webhook_id)
  6. Return WebhookValidation { valid: true/false, fields, error }
```

### 5.8 `error.rs` — Error Types

**Purpose:** Provide structured, actionable error types instead of generic strings.

```
ShopifyError (enum — each variant is a different error type)
├── Request(reqwest::Error)       ← Network failure
├── Json(serde_json::Error)       ← JSON parse failure
├── UrlParse(url::ParseError)     ← Bad URL
├── Unauthorized(String)          ← HTTP 401
├── Forbidden(String)             ← HTTP 403
├── NotFound { resource, id }     ← HTTP 404
├── ValidationError(String)       ← HTTP 422
├── RateLimited { retry_after }   ← HTTP 429 (after all retries exhausted)
├── ServerError(String)           ← HTTP 5xx
├── ApiError { status, message }  ← Any other HTTP error
├── InvalidOAuth(String)          ← OAuth HMAC/state failure
├── PrivateAppError(String)       ← OAuth attempted on custom app
├── CookieNotFound(String)        ← Missing OAuth state cookie
├── InvalidWebhookHmac            ← Webhook signature mismatch
├── InvalidSession(String)        ← Session data corrupted
└── InvalidConfig(String)         ← Missing config fields
```

---

## 6. Request Lifecycle

Here's the complete path of a `Product::find(&client, 123)` call:

```
1. Product::find(&client, 123)
   └── Builds path: "products/123.json"

2. client.get::<ProductWrapper>("products/123.json")
   └── Builds full URL: "https://my-shop.com/admin/api/2026-01/products/123.json"
   └── Attaches headers:
       ├── X-Shopify-Access-Token: shpat_xxx
       └── Content-Type: application/json

3. reqwest sends HTTP GET
   └── Shopify responds with JSON:
       {"product": {"id": 123, "title": "Widget", ...}}

4. Response handling:
   ├── Status 200 → Parse JSON into ProductWrapper struct
   ├── Status 429 → Sleep(Retry-After), retry (up to 3×)
   └── Status 4xx/5xx → Map to ShopifyError variant

5. Extract inner product from wrapper:
   ProductWrapper { product: Product { id: 123, title: "Widget" } }
   └── Return Ok(Some(product))
```

---

## 7. Authentication & Security

| Feature | Implementation | Why It Matters |
|---|---|---|
| **Token injection** | `X-Shopify-Access-Token` header on every request | Standard Shopify auth |
| **OAuth HMAC** | HMAC-SHA256 hex digest of callback query params | Prevents callback tampering |
| **Webhook HMAC** | HMAC-SHA256 base64 digest of raw body | Verifies webhook authenticity |
| **Constant-time compare** | `subtle::ConstantTimeEq` | Prevents timing oracle attacks |
| **CSRF protection** | Random nonce in OAuth state cookie | Prevents cross-site request forgery |
| **No unsafe code** | `#![deny(unsafe_code)]` at crate root | Memory safety guaranteed by compiler |

---

## 8. Session Management

```
                    ┌─────────────────┐
                    │  SessionManager  │
                    │  (high-level)    │
                    └────────┬────────┘
                             │ delegates to
                    ┌────────▼────────┐
                    │  SessionStore    │
                    │  (low-level)     │
                    └────────┬────────┘
                             │ implements
                    ┌────────▼────────┐
                    │ MemorySession   │
                    │ Store (HashMap) │
                    └─────────────────┘

SessionManager adds business logic on top of raw storage:
  • get_active_session(id) → loads session, checks is_active() with 500ms buffer
  • delete_shop_sessions(shop) → finds all sessions for a shop, bulk deletes
```

---

## 9. Resource Pattern (73 Endpoints)

Every resource follows the same consistent pattern. Here's `Product` as an example:

```rust
// 1. STRUCT — defines all fields with Option<T> for flexibility
struct Product {
    id: Option<i64>,
    title: Option<String>,
    body_html: Option<String>,
    vendor: Option<String>,
    variants: Option<Vec<Variant>>,
    images: Option<Vec<Image>>,
    // ... more fields
}

// 2. WRAPPERS — match Shopify's JSON envelope format
//    Shopify returns: {"product": {...}}  not just {...}
struct ProductWrapper { product: Product }       // single
struct ProductsWrapper { products: Vec<Product> }// list

// 3. PARAMS — typed query parameters with builder pattern
struct ProductListParams {
    limit: Option<i32>,
    vendor: Option<String>,
    status: Option<String>,
    // ... more filters
}

// 4. METHODS — CRUD operations
impl Product {
    fn find(client, id) → Option<Product>           // GET /products/{id}.json
    fn all(client, params) → FindAllResponse         // GET /products.json
    fn count(client, params) → i64                   // GET /products/count.json
    fn create(client, product) → Product             // POST /products.json
    fn update(client, product) → Product             // PUT /products/{id}.json
    fn save(client, product) → Product               // create or update
    fn delete(client, id) → ()                       // DELETE /products/{id}.json
}
```

Some resources have additional actions:
- `Order`: `cancel()`, `close()`, `open()`
- `Customer`: `search()`
- `Fulfillment`: specialized creation flows

---

## 10. Error Handling

The SDK uses Rust's `Result` type throughout — no exceptions, no panics, no null returns:

```rust
// Every SDK call returns Result<T, ShopifyError>
match Product::find(&client, 123).await {
    Ok(Some(product)) => println!("Found: {}", product.title.unwrap()),
    Ok(None)          => println!("Not found"),
    Err(ShopifyError::RateLimited { retry_after }) => {
        println!("Rate limited, retry in {}s", retry_after);
    }
    Err(ShopifyError::Unauthorized(msg)) => {
        println!("Bad token: {}", msg);
    }
    Err(e) => println!("Other error: {}", e),
}
```

---

## 11. Design Decisions & Trade-offs

| Decision | Rationale | Trade-off |
|---|---|---|
| `Option<T>` for all resource fields | Shopify may omit any field; partial updates send only changed fields | More verbose than required fields |
| `Arc<Config>` shared across modules | Single validated config shared without copying; thread-safe | Slight indirection overhead |
| Retry logic duplicated per HTTP method | Each method is self-contained and readable | ~50 lines repeated 6× (could be DRY'd) |
| `String` timestamps instead of `DateTime` | Simpler, no parse failures on unexpected formats | Loses compile-time date validation |
| `serde_json::Value` for some nested types | Some Shopify response shapes are highly variable | Loses type safety for those fields |
| `MemorySessionStore` as default | Zero-config for development and testing | Not suitable for production (data lost on restart) |
| `derive_builder` for param structs | Fluent construction without manual builder code | Adds a compile-time dependency |

---

*This document covers the REST port implementation as of API version 2026-01.*
