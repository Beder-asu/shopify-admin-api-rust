use axum::{
    extract::{Query, State},
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Redirect},
    routing::{get, post},
    Json, Router,
};
use dotenv::dotenv;
use serde::Deserialize;
use serde_json::json;
use std::env;
use std::sync::Arc;
use tokio::net::TcpListener;

// Import from our SDK crates
use shopify_core::session::Session;
use shopify_graphql_api::Client as GraphQlClient;
use shopify_graphql_api::generated::{product_create, ProductCreate};

#[derive(Clone)]
struct AppState {
    store_url: String,
    access_token: String,
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    
    // Read from our existing .env file
    let store_url = env::var("STORE_URL").expect("STORE_URL must be set");
    let access_token = env::var("ACCESS_TOKEN").expect("ACCESS_TOKEN must be set");

    let state = AppState {
        store_url,
        access_token,
    };

    let app = Router::new()
        .route("/", get(index))
        .route("/auth", get(start_oauth))
        .route("/auth/callback", get(oauth_callback))
        .route("/api/products/create", post(create_dummy_product))
        .route("/webhooks/orders/create", post(receive_webhook))
        .with_state(Arc::new(state));

    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("🚀 Server running on http://127.0.0.1:3000");
    println!("👉 Try creating a product: curl -X POST http://127.0.0.1:3000/api/products/create");
    
    axum::serve(listener, app).await.unwrap();
}

/// Simple index page
async fn index() -> &'static str {
    "Welcome to the Shopify Rust SDK Example App!\n\n\
    Available Routes:\n\
    GET  /auth\n\
    GET  /auth/callback\n\
    POST /api/products/create\n\
    POST /webhooks/orders/create\n"
}

/// 1. Start the OAuth Flow
async fn start_oauth() -> Redirect {
    // In a real app, you would use `ShopifyApp::auth_url()` from `rust-port`
    // redirecting them to the Shopify permission screen.
    println!("Redirecting merchant to Shopify...");
    Redirect::to("https://admin.shopify.com/oauth/authorize")
}

#[derive(Deserialize)]
struct AuthCallback {
    shop: String,
    code: String,
    hmac: String,
}

/// 2. Handle the OAuth Callback
async fn oauth_callback(Query(params): Query<AuthCallback>) -> impl IntoResponse {
    // In a real app, you would:
    // 1. Verify `params.hmac` using `shopify_core::webhooks::validate_hmac`
    // 2. Exchange `params.code` for an access token
    // 3. Save the `Session` to the database
    println!("Received OAuth callback for shop: {}", params.shop);
    (StatusCode::OK, "App successfully installed!")
}

/// 3. GraphQL Mutation: Create a Dummy Product
async fn create_dummy_product(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    // Reconstruct the Session from the database/state
    let shop_domain = state.store_url.replace("https://", "").replace("http://", "");
    let mut session = Session::new_offline(&shop_domain, "state");
    session.access_token = Some(state.access_token.clone());
    
    // Initialize our GraphQL client
    let client = GraphQlClient::new(session, "2026-01");

    // Construct the strongly-typed mutation variables using our generated SDK
    let variables = product_create::Variables {
        input: product_create::ProductInput {
            title: "Rust SDK Dummy Product".to_string(),
            vendor: Some("Rustaceans".to_string()),
        },
    };

    // Execute the GraphQL mutation
    match client.send_query::<ProductCreate>(variables).await {
        Ok(response) => {
            if let Some(data) = response.data {
                let msg = format!("Success! Created Product: {:?}", data.product_create.product.map(|p| p.title));
                (StatusCode::OK, msg)
            } else {
                let msg = format!("GraphQL returned errors: {:?}", response.errors);
                (StatusCode::INTERNAL_SERVER_ERROR, msg)
            }
        }
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, format!("Error: {}", e)),
    }
}

/// 4. Secure Webhook Receiver
async fn receive_webhook(headers: HeaderMap, body: String) -> impl IntoResponse {
    // Extract the HMAC header Shopify sends
    let hmac_header = headers
        .get("X-Shopify-Hmac-Sha256")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");

    // Validate using our core crypto library
    let api_secret = env::var("API_SECRET").unwrap_or_else(|_| "secret".into());
    let is_valid = shopify_core::webhooks::validate_hmac(&api_secret, body.as_bytes(), hmac_header);

    if is_valid {
        println!("✅ Valid Webhook received: {}", body);
        StatusCode::OK
    } else {
        println!("❌ Invalid Webhook Signature!");
        StatusCode::UNAUTHORIZED
    }
}
