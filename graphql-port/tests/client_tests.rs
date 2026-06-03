use shopify_graphql_api::{Client, Session};
use shopify_graphql_api::generated::{ShopDetails, shop_details};
use mockito::Server;

#[tokio::test]
async fn test_graphql_query_success() {
    let mut server = Server::new_async().await;
    let url = server.url();
    let host = url.replace("http://", "");

    // The client builds the URL as http://{shop}/admin/api/...
    let mock = server.mock("POST", "/admin/api/2026-01/graphql.json")
        .match_header("X-Shopify-Access-Token", "test_token")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"
        {
            "data": {
                "shop": {
                    "id": "gid://shopify/Shop/123",
                    "name": "Test Shop",
                    "myshopifyDomain": "test-shop.myshopify.com",
                    "plan": {
                        "displayName": "Partner Test",
                        "partnerDevelopment": true,
                        "shopifyPlus": false
                    }
                }
            }
        }
        "#)
        .create_async().await;

    let mut session = Session::new_offline(&host, "state");
    session.access_token = Some("test_token".to_string());
    let client = Client::new(session, "2026-01");

    let response = client.send_query::<ShopDetails>(shop_details::Variables).await.unwrap();

    assert!(response.errors.is_none());
    let data = response.data.unwrap();
    assert_eq!(data.shop.name, "Test Shop");
    assert_eq!(data.shop.myshopify_domain, "test-shop.myshopify.com");
    assert_eq!(data.shop.plan.display_name, "Partner Test");
    
    mock.assert_async().await;
}
