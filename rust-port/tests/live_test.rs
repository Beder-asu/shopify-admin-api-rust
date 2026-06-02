use shopify_admin_api::{ConfigParams, MemorySessionStore, Session, ShopifyApp};
use serde_json::Value;

#[tokio::test]
async fn execute_live_api_test() -> Result<(), Box<dyn std::error::Error>> {
    // Load .env from the port/ root directory
    // rust-port/tests/live_test.rs
let manifest_dir = env!("CARGO_MANIFEST_DIR");  // = .../port/rust-port
let env_path = std::path::Path::new(manifest_dir).join("..").join(".env");
let _ = dotenv::from_path(env_path);

    // 1. Fetch credentials from environment variables manually provided for Phase 4 live testing
    let store_url = match std::env::var("STORE_URL") {
        Ok(url) => url,
        Err(_) => {
            println!("Skipping live test: STORE_URL missing");
            return Ok(());
        }
    };
    
    let access_token = match std::env::var("ACCESS_TOKEN") {
        Ok(token) => token,
        Err(_) => {
            println!("Skipping live test: ACCESS_TOKEN missing");
            return Ok(());
        }
    };

    let host_domain = store_url.replace("https://", "").replace("http://", "");

    // 2. Initialize the SDK
    let app = ShopifyApp::new(
        ConfigParams {
            api_key: "test_key".into(),
            api_secret_key: "test_secret".into(),
            host_name: host_domain.clone(),
            api_version: "2026-01".into(),
            scopes: Some("write_products,read_orders".into()),
            is_custom_store_app: true, // Crucial for offline tokens
            admin_api_access_token: Some(access_token.clone()), // Fixes InvalidConfig error
            ..Default::default()
        },
        MemorySessionStore::new(),
    )?;

    // 3. Inject our offline token directly into a new Session to bypass OAuth
    let mut session = Session::new_offline(&host_domain, "live_test_state");
    session.access_token = Some(access_token);

    // 4. Initialize our Rest client with this live session
    let client = app.rest_client(&session);

    println!("Attempting live connection to Shopify at {}...", host_domain);

    // 5. Let's make a manual request to GET /admin/api/2026-01/shop.json
    let response: shopify_admin_api::ApiResponse<Value> = client.get("shop.json").await?;

    println!("SUCCESS! Successfully hit the REST Admin API.");
    println!("Shop Details: {}", serde_json::to_string_pretty(&response.data)?);

    Ok(())
}
