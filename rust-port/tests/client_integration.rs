use mockito::Server;
use serde_json::{json, Value};
use shopify_admin_api::{ConfigParams, MemorySessionStore, Session, ShopifyApp};

#[tokio::test]
async fn test_client_handles_200_ok() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Spin up a local background HTTP mock server
    let mut server = Server::new_async().await;
    
    // 2. Mock a Shopify REST endpoint route
    let mock = server.mock("GET", "/admin/api/2026-01/shop.json")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(json!({
            "shop": {
                "name": "Mocked Shop",
                "domain": "mock.myshopify.com"
            }
        }).to_string())
        .create_async().await;

    // 3. Initialize SDK using the mock server URL
    let host = server.url().replace("http://", "");
    let app = ShopifyApp::new(
        ConfigParams {
            api_key: "key".into(),
            api_secret_key: "secret".into(),
            host_name: host.clone(),
            host_scheme: Some(shopify_admin_api::config::Scheme::Http),
            api_version: "2026-01".into(),
            is_custom_store_app: true,
            admin_api_access_token: Some("fake_token".into()),
            ..Default::default()
        },
        MemorySessionStore::new(),
    )?;

    let mut session = Session::new_offline(&host, "state");
    session.access_token = Some("fake_token".into());
    let client = app.rest_client(&session);

    // 4. Hit the endpoint
    let response: shopify_admin_api::ApiResponse<Value> = client.get("shop.json").await?;

    // 5. Assert successful parsing
    assert_eq!(response.data["shop"]["name"], "Mocked Shop");
    mock.assert_async().await;

    Ok(())
}

#[tokio::test]
async fn test_client_handles_429_retries() -> Result<(), Box<dyn std::error::Error>> {
    let mut server = Server::new_async().await;
    
    // Create a mock that returns 429 Too Many Requests exactly once
    let mock_429 = server.mock("GET", "/admin/api/2026-01/products.json")
        .with_status(429)
        .with_header("Retry-After", "0.1") // Speed up tests by waiting 0.1s instead of default 2.0s
        .expect(1)
        .create_async().await;

    // Create a fallback mock for the 200 OK after retry
    let mock_200 = server.mock("GET", "/admin/api/2026-01/products.json")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(json!({"products": []}).to_string())
        .expect(1)
        .create_async().await;

    let host = server.url().replace("http://", "");
    let app = ShopifyApp::new(
        ConfigParams {
            api_key: "key".into(),
            api_secret_key: "secret".into(),
            host_name: host.clone(),
            host_scheme: Some(shopify_admin_api::config::Scheme::Http),
            api_version: "2026-01".into(),
            is_custom_store_app: true,
            admin_api_access_token: Some("fake_token".into()),
            ..Default::default()
        },
        MemorySessionStore::new(),
    )?;

    let mut session = Session::new_offline(&host, "state");
    session.access_token = Some("fake_token".into());
    let client = app.rest_client(&session);

    // The client should abstractly hit the 429, pause, retry, and hit the 200 Ok
    let _: shopify_admin_api::ApiResponse<Value> = client.get("products.json").await?;

    mock_429.assert_async().await;
    mock_200.assert_async().await;

    Ok(())
}
