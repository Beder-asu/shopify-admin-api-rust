use shopify_admin_api::auth::oauth::compute_hmac_hex;
use shopify_admin_api::webhooks;
use base64::{engine::general_purpose, Engine as _};
use hmac::{Hmac, Mac};
use sha2::Sha256;

#[test]
fn test_oauth_hmac_computation() {
    let secret = "hush";
    let data = "code=0907a61c0c8d55e99db179b68161bc&shop=some-shop.myshopify.com&state=0.6784241404160823&timestamp=1337178173";
    
    // Shopify documents this exact known payload and secret resulting in a specific HMAC-SHA256 hex string.
    let expected_hmac = "4c5c88b5a7fbeec2fd17f4c49d02a663c3ecbc6f431cdab8e1375e9ab0f4b955";
    
    let computed = compute_hmac_hex(secret, data.as_bytes());
    assert_eq!(computed, expected_hmac);
}

#[test]
fn test_webhook_hmac_validation_success() {
    let secret = "my_super_secret_key";
    let body_bytes = br#"{"my": "webhook payload"}"#;
    
    // Manually compute the valid HMAC base64 string
    let mut mac = Hmac::<Sha256>::new_from_slice(secret.as_bytes()).unwrap();
    mac.update(body_bytes);
    let valid_mac_bytes = mac.finalize().into_bytes();
    let valid_base64 = general_purpose::STANDARD.encode(valid_mac_bytes);
    
    let is_valid = webhooks::validate_hmac(secret, body_bytes, &valid_base64);
    assert!(is_valid, "Valid HMAC header should pass validation");
}

#[test]
fn test_webhook_hmac_validation_failure() {
    let secret = "my_super_secret_key";
    let body_bytes = br#"{"my": "webhook payload"}"#;
    
    let invalid_base64 = "aW52YWxpZF9zaWduYXR1cmU="; // "invalid_signature" in b64
    
    let is_valid = webhooks::validate_hmac(secret, body_bytes, invalid_base64);
    assert!(!is_valid, "Invalid HMAC header MUST fail validation");
}

#[test]
fn test_constant_time_compare_lengths() {
    // Ensuring basic functionality of ct compare string wrapper
    use shopify_admin_api::auth::oauth::constant_time_compare;
    
    assert!(constant_time_compare("exact_match", "exact_match"));
    assert!(!constant_time_compare("exact_match", "Exact_match"));
    assert!(!constant_time_compare("short", "longer_string"));
}
