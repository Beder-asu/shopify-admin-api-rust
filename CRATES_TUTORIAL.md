# 📦 Rust Crates Tutorial (for JS/TS Devs)

If you are coming from JavaScript or TypeScript, you are very familiar with `npm`, `package.json`, and `node_modules`. 

Rust has a completely different ecosystem, but it maps perfectly to what you already know. This tutorial will explain **Crates**, **Cargo**, and how dependencies work in Rust.

---

## 1. What is a "Crate"?

In Rust, a **Crate** is simply a package or library. It is the equivalent of an npm package. 
When people say "I used the `serde` crate", they mean "I installed the `serde` npm package".

There are two main types of crates:
1. **Library Crates (`lib.rs`)**: These are meant to be imported and used by other projects. (Equivalent to `npm install lodash`). Most of our Shopify SDK components (`shopify-core`, `rust-port`, `graphql-port`) are library crates.
2. **Binary Crates (`main.rs`)**: These are executable programs. When you compile them, they produce an `.exe` (on Windows) or a binary file (on Mac/Linux) that you can actually run.

---

## 2. Cargo vs NPM

**Cargo** is Rust's official package manager and build system. It is the equivalent of `npm` (or `yarn` / `pnpm`).

| Action | JavaScript (npm) | Rust (Cargo) |
| --- | --- | --- |
| Create a new project | `npm init` | `cargo init` or `cargo new my_app` |
| Install a dependency | `npm install axios` | `cargo add reqwest` |
| Remove a dependency | `npm uninstall axios`| `cargo remove reqwest` |
| Build the project | `npm run build` | `cargo build` |
| Run the project | `npm start` | `cargo run` |
| Run tests | `npm test` | `cargo test` |

---

## 3. `Cargo.toml` vs `package.json`

In Node.js, your dependencies and scripts are defined in `package.json`.
In Rust, this file is called **`Cargo.toml`**. 

Here is what a basic `Cargo.toml` looks like compared to a `package.json`:

```toml
[package]
name = "my_awesome_app"
version = "0.1.0"
edition = "2021"

[dependencies]
# This is where your external crates go (like node_modules)
tokio = { version = "1.0", features = ["full"] }
reqwest = { version = "0.11", features = ["json"] }
serde = { version = "1.0", features = ["derive"] }
```

### Note on "Features":
Notice the `features = ["json"]` part? Rust is obsessed with keeping binary sizes small. Crates usually turn off most of their code by default. If you want `reqwest` to be able to parse JSON, you have to explicitly enable the `"json"` feature when you install it.

---

## 4. Cargo Workspaces (Monorepos)

In JavaScript, if you want multiple packages in one repository, you use "Yarn Workspaces" or "Lerna".
Rust has this built-in, and it's called **Cargo Workspaces**.

If you look at the root of our `port/` folder, you'll see a `Cargo.toml` that looks like this:
```toml
[workspace]
members = [
    "shopify-core",
    "rust-port",
    "graphql-port"
]
```
This tells Cargo: *"Treat these three folders as independent crates, but compile them together and share their dependencies."*

When `rust-port` needs to use `shopify-core`, it simply adds it to its own `Cargo.toml` like this:
```toml
[dependencies]
shopify-core = { path = "../shopify-core" }
```

---

## 5. The "Big Four" Rust Crates

In Node.js, you use standard libraries like `Express`, `Axios`, and `Jest`.
In Rust, the standard library is very small, so the community relies on heavily tested external crates. Here are the 4 crates you will see in almost every single Rust backend project:

### 1. Tokio (The Event Loop)
JavaScript has an event loop built into the engine. Rust does not. If you want to use `async` / `await`, you must install a runtime. **Tokio** is the standard async runtime for Rust.
*(You'll often see `#[tokio::main]` at the top of a `main.rs` file. This starts the event loop).*

### 2. Serde (JSON Parsing)
Parsing JSON in JS is easy (`JSON.parse()`). In Rust, because it is strongly typed, you must map JSON to Structs. **Serde** (Serialize/Deserialize) is the crate that handles this magically using Macros (`#[derive(Serialize, Deserialize)]`).

### 3. Reqwest (HTTP Client)
This is the Rust equivalent of **Axios** or **Fetch**. It is used to make HTTP requests to other APIs.

### 4. Thiserror (Error Handling)
Because Rust doesn't have `throw new Error()`, creating custom Error enums can be tedious. **Thiserror** is a utility crate that automatically generates the boilerplate required to create beautiful, readable error types.

---

## Summary

- **Crates** = `npm` packages
- **Cargo** = `npm` / `yarn`
- **Cargo.toml** = `package.json`
- **Cargo.lock** = `package-lock.json`
- **Workspace** = Monorepo
- **lib.rs** = `index.js` (exported library)
- **main.rs** = `server.js` (executable binary)
