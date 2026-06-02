//! Incoming webhook validation.
//!
//! Mirrors `lib/webhooks/validate.ts` from `@shopify/shopify-api`.

use base64::{engine::general_purpose, Engine as _};
use hmac::{Hmac, Mac};
use reqwest::header::HeaderMap;
use sha2::Sha256;
use subtle::ConstantTimeEq;

type HmacSha256 = Hmac<Sha256>;

/// Fields extracted from a validated webhook's HTTP headers.
#[derive(Debug, Clone)]
pub struct WebhookFields {
    /// The webhook topic (e.g. `"orders/paid"`).
    pub topic: String,
    /// The shop domain that sent the webhook.
    pub domain: String,
    /// The API version the webhook was sent from.
    pub api_version: String,
    /// The unique webhook delivery ID.
    pub webhook_id: String,
    /// Optional sub-topic for scoped webhooks.
    pub sub_topic: Option<String>,
}

/// Reason a webhook validation failed.
#[derive(Debug, Clone)]
pub enum WebhookValidationError {
    /// The `X-Shopify-Hmac-Sha256` header was missing from the request.
    MissingHmacHeader,
    /// The HMAC did not match — body may have been tampered with.
    InvalidHmac,
    /// One or more required webhook headers were absent.
    MissingRequiredHeaders(Vec<String>),
}

/// Result of validating an incoming webhook request.
#[derive(Debug)]
pub struct WebhookValidation {
    /// Whether the webhook signature and headers are valid.
    pub valid: bool,
    /// Parsed header fields — present only when `valid == true`.
    pub fields: Option<WebhookFields>,
    /// Reason for failure — present only when `valid == false`.
    pub error: Option<WebhookValidationError>,
}

/// Validate an incoming webhook's HMAC-SHA256 signature.
///
/// Uses **constant-time comparison** to prevent timing oracle attacks.
///
/// # Arguments
/// - `secret` — your app's `api_secret_key` (from [`Config`][crate::config::Config])
/// - `raw_body` — the raw, unparsed POST body bytes
/// - `hmac_header` — value of the `X-Shopify-Hmac-Sha256` header (Base64-encoded)
///
/// Mirrors the HMAC validation in `lib/utils/hmac-validator.ts`.
pub fn validate_hmac(secret: &str, raw_body: &[u8], hmac_header: &str) -> bool {
    let mut mac = match HmacSha256::new_from_slice(secret.as_bytes()) {
        Ok(m) => m,
        Err(_) => return false,
    };
    mac.update(raw_body);
    let computed = mac.finalize().into_bytes();

    let expected = match general_purpose::STANDARD.decode(hmac_header.trim()) {
        Ok(b) => b,
        Err(_) => return false,
    };

    if computed.len() != expected.len() {
        return false;
    }

    // Constant-time comparison (prevents timing attacks)
    computed.as_slice().ct_eq(expected.as_slice()).into()
}

/// Full webhook validation: HMAC signature check + required header extraction.
///
/// Mirrors `shopify.webhooks.validate()` from `@shopify/shopify-api`.
///
/// # Arguments
/// - `secret` — your app's `api_secret_key`
/// - `raw_body` — raw POST body bytes (before JSON parsing)
/// - `headers` — the request's HTTP headers
pub fn validate(
    secret: &str,
    raw_body: &[u8],
    headers: &HeaderMap,
) -> WebhookValidation {
    let hmac_header = match headers
        .get("x-shopify-hmac-sha256")
        .and_then(|v| v.to_str().ok())
    {
        Some(v) => v.to_string(),
        None => {
            return WebhookValidation {
                valid: false,
                fields: None,
                error: Some(WebhookValidationError::MissingHmacHeader),
            };
        }
    };

    if !validate_hmac(secret, raw_body, &hmac_header) {
        return WebhookValidation {
            valid: false,
            fields: None,
            error: Some(WebhookValidationError::InvalidHmac),
        };
    }

    let get = |name: &str| -> Option<String> {
        headers
            .get(name)
            .and_then(|v| v.to_str().ok())
            .map(|s| s.to_string())
    };

    let mut missing = Vec::new();
    let topic = get("x-shopify-topic");
    let domain = get("x-shopify-shop-domain");
    let api_version = get("x-shopify-api-version");
    let webhook_id = get("x-shopify-webhook-id");

    if topic.is_none() {
        missing.push("x-shopify-topic".to_string());
    }
    if domain.is_none() {
        missing.push("x-shopify-shop-domain".to_string());
    }
    if api_version.is_none() {
        missing.push("x-shopify-api-version".to_string());
    }
    if webhook_id.is_none() {
        missing.push("x-shopify-webhook-id".to_string());
    }

    if !missing.is_empty() {
        return WebhookValidation {
            valid: false,
            fields: None,
            error: Some(WebhookValidationError::MissingRequiredHeaders(missing)),
        };
    }

    WebhookValidation {
        valid: true,
        fields: Some(WebhookFields {
            topic: topic.unwrap(),
            domain: domain.unwrap(),
            api_version: api_version.unwrap(),
            webhook_id: webhook_id.unwrap(),
            sub_topic: get("x-shopify-sub-topic"),
        }),
        error: None,
    }
}
