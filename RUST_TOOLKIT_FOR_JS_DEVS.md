# 🦀 Rust Toolkit for JS/TS Developers

Welcome! If you are a Javascript/Typescript backend developer diving into this Rust codebase, you will find that the **business logic** looks very familiar, but the **memory management and strict typing** can feel alien.

This toolkit explains the most confusing Rust syntax and concepts you'll encounter in this Shopify App SDK, mapped directly to what you already know from Node.js/TypeScript.

---

## 1. Ownership & Borrowing (The `&` Symbol)

In JS, objects are passed by reference automatically. When you pass `user` to a function, the function can mutate or read `user` freely.
In Rust, variables have **Ownership**. If you pass a variable to a function, you *give* it away. If you try to use it again, the compiler throws an error.

To avoid giving away ownership, you **Borrow** the variable using the `&` symbol (called a reference).

**JS / TS:**

```typescript
function processData(data: object) { /* ... */ }

const payload = { id: 1 };
processData(payload);
console.log(payload); // Perfectly fine
```

**Rust:**

```rust
fn process_data(data: &Payload) { /* ... */ }

let payload = Payload { id: 1 };
// We pass `&payload` to say "you can borrow this to read it, but I keep ownership"
process_data(&payload);
println!("{:?}", payload); // Perfectly fine, because we only let process_data borrow it.
```

**Takeaway:** If you see a function demanding `&something`, just add `&` in front of your variable when you call it.

---

## 2. Strings: `String` vs `&str`

This is the #1 pain point for JS devs. In JS, a string is just a string.
In Rust, there are two main types of strings:

- `String`: An owned, mutable string allocated on the heap (like a dynamic array of characters).
- `&str`: A borrowed "slice" (reference) to string data. It's fast and read-only.

**Common String Conversions you will see in this codebase:**

- `"hello"` ➔ This is an `&str` (a hardcoded string literal).
- `"hello".to_string()` ➔ Converts `&str` into a fully owned `String`.
- `my_string.clone()` ➔ Creates a deep copy of a `String` (used when you need to pass a string somewhere but also keep the original).
- `my_string.as_str()` ➔ Converts a `String` back into an `&str`.
- `my_option_string.as_deref()` ➔ Converts `Option<String>` into `Option<&str>`.

**JS / TS:**

```typescript
const shop = "my-store.myshopify.com";
const id = `offline_${shop}`;
```

**Rust:**

```rust
let shop: &str = "my-store.myshopify.com";
let id: String = format!("offline_{}", shop); // format! creates an owned String
```

---

## 3. Handling Nulls: `Option<T>`

Rust completely eliminates `null` and `undefined`. Instead, it uses the `Option<T>` enum.
A value can either be `Some(data)` or `None`.

**JS / TS:**

```typescript
if (session.accessToken !== undefined && session.accessToken !== null) {
  makeRequest(session.accessToken);
}
```

**Rust:**

```rust
// "If let" is a fast way to unwrap an Option if it exists
if let Some(token) = &session.access_token {
    make_request(token);
}

// Alternatively, you can forcefully unwrap it (Will crash the app if it's None!)
let token = session.access_token.unwrap(); 
```

---

## 4. Error Handling: `Result<T, E>` and the `?` Operator

Rust does not use `try / catch` blocks or exceptions. Functions that can fail return a `Result` enum, which is either `Ok(data)` or `Err(error)`.

The `?` operator is magical syntax sugar. If you put `?` at the end of a function call, it means: *"If this succeeds, unwrap the `Ok` value. If it fails, immediately exit the current function and return the `Err`."*

**JS / TS:**

```typescript
try {
  const response = await client.get("/products.json");
  return response.data;
} catch (error) {
  // Bubbles up implicitly
  throw error; 
}
```

**Rust:**

```rust
// The `?` acts as an automatic try/catch that bubbles the error up.
let response = client.get("/products.json").await?;
Ok(response.data)
```

---

## 5. Structs and Methods (`impl`)

Rust does not have `class`. Data and behavior are strictly separated.

- `struct` defines the fields (like a TS `interface`).
- `impl` defines the methods.

**JS / TS:**

```typescript
class Session {
  id: string;
  constructor(id: string) {
    this.id = id;
  }
  isExpired() { return false; }
}
```

**Rust:**

```rust
// Data
pub struct Session {
    pub id: String,
}

// Behavior
impl Session {
    // A static method (no `self`)
    pub fn new(id: String) -> Self {
        Self { id }
    }
  
    // An instance method (takes `&self`)
    pub fn is_expired(&self) -> bool {
        false
    }
}
```

---

## 6. Generics (`<T>`) and Traits (`DeserializeOwned`)

Rust heavily relies on generic programming. A **Trait** in Rust is similar to an `interface` in TypeScript—it defines functionality that a type must have.

When you see a function signature like:

```rust
pub async fn get<T: DeserializeOwned>(&self, path: &str) -> Result<ApiResponse<T>>
```

It reads as: *"This function works for any type `T`, as long as `T` implements the `DeserializeOwned` trait."* (`DeserializeOwned` is a Trait from the `serde` library that means the JSON can be converted into this struct).

**How to call it (The "Turbofish" `::<T>`):**

```rust
// We tell the compiler we want the JSON parsed into the `ProductResponse` struct.
let result = client.get::<ProductResponse>("products.json").await?;
```

---

## 7. Macros (`#[derive(...)]` and `format!()`)

If you see syntax ending in `!` (like `println!()` or `format!()`), or syntax starting with `#` (like `#[derive(Debug)]`), these are **Macros**.

Macros are code that writes code before the app compiles.

- `#[derive(Serialize, Deserialize)]` tells the compiler: *"Look at this struct, and automatically generate the 100 lines of boilerplate code needed to convert it to and from JSON."*
- `format!("hello {}", name)` tells the compiler to safely interpolate strings (similar to JS template literals `` `hello ${name}` ``).
