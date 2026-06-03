# Shopify Admin API Rust SDK

Welcome to the Shopify Admin API SDK for Rust. This repository contains the tools you need to build powerful Shopify Apps using Rust. 

The architecture is split into three main crates (libraries):

1. **`shopify-core`**: Contains all the shared logic, including `Session` management, Authentication scopes, Error Types, and Webhook verification.
2. **`shopify-admin-api`** (rust-port): A strongly-typed wrapper around the legacy REST API.
3. **`shopify-graphql-api`** (graphql-port): A strongly-typed, auto-generating client for the Admin GraphQL API.

---

## 🦀 Rust Crash Course (for JS/TS Backend Devs)

If you're coming from a Node.js/TypeScript background, Rust shares a lot of similar concepts, but uses different terminology and enforces stricter rules. Here is a quick mapping of concepts to help you navigate this SDK.

### 1. `Result<T, E>` vs `try / catch`
In JavaScript, functions that might fail throw exceptions which you handle using `try/catch`. 
In Rust, functions that can fail return a `Result<T, E>` enum. A `Result` is either:
- `Ok(T)` — It worked! Here is the data of type `T`.
- `Err(E)` — It failed! Here is the error of type `E`.

**JS/TS Example:**
```typescript
try {
  const response = await client.get("/products.json");
  return response.data;
} catch (error) {
  console.error("Failed:", error);
}
```

**Rust Equivalent (`?` Operator):**
```rust
// The `?` operator acts like an automatic try/catch. 
// If `client.get` returns an `Err`, the function immediately returns that error.
// If it succeeds, it unwraps the `Ok` value.
let response = client.get::<ProductList>("/products.json").await?;
println!("Got products: {:?}", response.data);
```

### 2. `Option<T>` vs `null` / `undefined`
Rust completely avoids "billion-dollar mistakes" by not having `null` or `undefined`. Instead, it uses the `Option<T>` enum.
An `Option` is either:
- `Some(T)` — The value is present.
- `None` — The value is missing.

**JS/TS Example:**
```typescript
if (session.accessToken !== undefined) {
  console.log(session.accessToken);
}
```

**Rust Equivalent:**
```rust
if let Some(token) = &session.access_token {
    println!("{}", token);
}
```

### 3. Generics (`<T>`) and Traits (`DeserializeOwned`)
You will frequently see `<T: DeserializeOwned>` on functions in this SDK. 
- A **Trait** in Rust is like an `interface` in TypeScript.
- `DeserializeOwned` is a Trait from the `serde` library that means "This type can be built from JSON".

When you call `client.get::<ProductList>("/products.json")`, you are telling the compiler: "Expect the JSON to match the `ProductList` struct." The Rust compiler will enforce this at compile time.

### 4. Structs and `impl` vs `class`
Rust does not have classes. It separates data from behavior.
- `struct`: Defines the data (like a TS interface).
- `impl`: Defines the methods that operate on that data.

### 5. `&` (References and Borrowing)
In JavaScript, objects are automatically passed by reference. In Rust, passing a variable gives away "ownership" of it unless you use a reference (`&`).
When you see `client.get(&url)`, the `&` means we are letting the `get` method temporarily "borrow" the string `url` to read it, but the caller still owns the memory.

---

## Getting Started

### GraphQL (Recommended)

Shopify strongly recommends using the GraphQL Admin API for all new development. 

1. Ensure your `schema.graphql` is up to date (download it using the Shopify CLI).
2. Write your `.graphql` queries and mutations inside `graphql-port/queries.graphql`.
3. Rust will **automatically** generate strongly-typed Request and Response structs at compile time!

```rust
use shopify_graphql_api::{Client, Session};
use shopify_graphql_api::generated::{ShopDetails, shop_details};

let session = Session::new_offline("my-store.myshopify.com", "state");
let client = Client::new(session, "2026-01");

// `shop_details::Variables` is automatically generated and type-checked!
let response = client.send_query::<ShopDetails>(shop_details::Variables).await?;

println!("Shop Name: {}", response.data.unwrap().shop.name);
```

### REST (Legacy)

The REST API is largely deprecated for public apps but remains supported.
```rust
use shopify_admin_api::{Client, Session};

let session = Session::new_offline("my-store.myshopify.com", "state");
let client = Client::new(config, session.access_token.unwrap());

// You provide the generic type <T> you want the JSON parsed into
let result = client.get::<MyProductResponse>("products.json").await?;
```
