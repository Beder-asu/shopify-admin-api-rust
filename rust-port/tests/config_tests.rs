use shopify_admin_api::{ConfigParams, config::validate_config};

#[test]
fn test_config_validation_success() {
    let params = ConfigParams {
        api_key: "myapp_key".into(),
        api_secret_key: "secret".into(),
        host_name: "app.test.com/".into(), // Testing slash trimming
        api_version: "2026-01".into(),
        ..Default::default()
    };

    let config = validate_config(params).expect("Valid config should not error");
    assert_eq!(config.host_name, "app.test.com"); // Ensured slash was trimmed
}

#[test]
fn test_config_validation_missing_key() {
    let params = ConfigParams {
        api_secret_key: "secret".into(),
        host_name: "app.test.com".into(),
        api_version: "2026-01".into(),
        ..Default::default()
    };

    let result = validate_config(params);
    assert!(result.is_err());
    if let Err(e) = result {
        assert!(e.to_string().contains("api_key"));
    }
}

#[test]
fn test_custom_app_requires_admin_token() {
    // Custom apps skip the `api_key` check but require `admin_api_access_token`
    let params = ConfigParams {
        api_secret_key: "secret".into(),
        host_name: "app.test.com".into(),
        api_version: "2026-01".into(),
        is_custom_store_app: true,
        ..Default::default()
    };

    let result = validate_config(params);
    assert!(result.is_err());
    
    if let Err(e) = result {
        assert!(e.to_string().contains("admin_api_access_token"));
    }
}
