use serde_json::Value;
use shopify_graphql_api::{Client, Session};

#[tokio::test]
async fn execute_live_graphql_test() -> Result<(), Box<dyn std::error::Error>> {
    // Load .env from the port/ root directory
    // rust-port/tests/live_test.rs
let manifest_dir = env!("CARGO_MANIFEST_DIR");  // = .../port/rust-port
let env_path = std::path::Path::new(manifest_dir).join("..").join(".env");
let _ = dotenv::from_path(env_path);

    // 1. Fetch credentials from environment variables manually provided for Phase 4 live testing
    let store_url = match std::env::var("STORE_URL") {
        Ok(url) => url,
        Err(_) => {
            println!("Skipping live graphql test: STORE_URL missing");
            return Ok(());
        }
    };
    
    let access_token = match std::env::var("ACCESS_TOKEN") {
        Ok(token) => token,
        Err(_) => {
            println!("Skipping live graphql test: ACCESS_TOKEN missing");
            return Ok(());
        }
    };

    let host_domain = store_url.replace("https://", "").replace("http://", "").replace("/", "");

    // 2. Initialize the Session and Client
    let mut session = Session::new_offline(&host_domain, "live_test_state");
    session.access_token = Some(access_token);
    let client = Client::new(session, "2026-01");

    println!("Attempting live GraphQL connection to Shopify at {}...", host_domain);

    // 3. Define a simple GraphQL query
    let query = r#"
        query {
            shop {
                name
                primaryDomain {
                    url
                }
            }
        }
    "#;

    // 4. Send the GraphQL query
    let data = client.graphql::<Value, ()>(query, None).await?;

    println!("SUCCESS! Successfully hit the GraphQL Admin API.");
    println!("GraphQL Shop Details: {}", serde_json::to_string_pretty(&data.data)?);
    
    if let Some(extensions) = data.extensions {
        println!("GraphQL Cost Extensions: {:#?}", extensions);
    }
    
    Ok(())
}
