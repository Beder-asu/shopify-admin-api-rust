//! OAuth flow implementation.
//!
//! Mirrors `lib/auth/oauth/oauth.ts` from `@shopify/shopify-api`.

use crate::auth::nonce::generate_nonce;
use crate::config::Config;
use crate::error::{Result, ShopifyError};
use crate::session::{AssociatedUser, OnlineAccessInfo, Session};
use hex;
use hmac::{Hmac, Mac};
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use subtle::ConstantTimeEq;
use url::Url;

type HmacSha256 = Hmac<Sha256>;

/// Cookie name for the OAuth state nonce.
///
/// Store the `OAuthState::state_nonce` in a signed cookie with this name.
pub const STATE_COOKIE_NAME: &str = "shopify_oauth_state";

/// Parameters for starting the OAuth authorization flow.
pub struct BeginParams {
    /// The merchant's shop domain (e.g. `"my-store.myplatform.com"`).
    pub shop: String,
    /// Path on your app server that handles the callback (e.g. `"/auth/callback"`).
    pub callback_path: String,
    /// `true` = per-user (online) token; `false` = app-wide (offline) token.
    pub is_online: bool,
}

/// Result of [`begin`] — use `redirect_url` to redirect the merchant's browser.
pub struct OAuthState {
    /// Redirect the merchant's browser to this URL to begin authorization.
    pub redirect_url: String,
    /// Store this in a signed cookie named [`STATE_COOKIE_NAME`].
    ///
    /// It is validated during [`callback`] to prevent CSRF.
    pub state_nonce: String,
}

/// Parameters arriving at your OAuth callback endpoint.
pub struct CallbackParams {
    /// Shop domain from the `shop` query parameter.
    pub shop: String,
    /// Authorization code from the `code` query parameter.
    pub code: String,
    /// State nonce from the `state` query parameter.
    pub state: String,
    /// HMAC from the `hmac` query parameter — validated before token exchange.
    pub hmac: String,
    /// Optional timestamp from the `timestamp` query parameter.
    pub timestamp: Option<String>,
    /// Whether to request an online (per-user) token.
    pub is_online: bool,
}

/// Successful OAuth callback result.
pub struct CallbackResponse {
    /// The fully built, ready-to-store session.
    pub session: Session,
}

/// Raw access token response from the authorization server.
#[derive(Debug, Deserialize)]
struct AccessTokenResponse {
    access_token: String,
    scope: String,
    expires_in: Option<i64>,
    associated_user_scope: Option<String>,
    associated_user: Option<AssociatedUserResponse>,
}

#[derive(Debug, Deserialize)]
struct AssociatedUserResponse {
    id: i64,
    first_name: Option<String>,
    last_name: Option<String>,
    email: Option<String>,
    email_verified: Option<bool>,
    account_owner: Option<bool>,
    locale: Option<String>,
    collaborator: Option<bool>,
}

/// Build the OAuth authorization redirect URL and state nonce.
///
/// The caller is responsible for setting a signed, short-lived cookie
/// (`key = STATE_COOKIE_NAME, value = OAuthState::state_nonce`).
///
/// Mirrors `shopify.auth.begin()` from `@shopify/shopify-api`.
pub fn begin(config: &Config, params: BeginParams) -> Result<OAuthState> {
    if config.is_custom_store_app {
        return Err(ShopifyError::PrivateAppError(
            "Cannot perform OAuth for custom store apps".to_string(),
        ));
    }

    let state_nonce = generate_nonce();

    let scopes = config
        .scopes
        .as_ref()
        .map(|s| s.to_string())
        .unwrap_or_default();

    let redirect_uri = format!(
        "{}://{}{}",
        config.host_scheme, config.host_name, params.callback_path
    );

    let grant_options = if params.is_online { "per-user" } else { "" };

    let mut url = Url::parse(&format!(
        "https://{}/admin/oauth/authorize",
        params.shop
    ))
    .map_err(ShopifyError::UrlParse)?;

    url.query_pairs_mut()
        .append_pair("client_id", &config.api_key)
        .append_pair("scope", &scopes)
        .append_pair("redirect_uri", &redirect_uri)
        .append_pair("state", &state_nonce)
        .append_pair("grant_options[]", grant_options);

    Ok(OAuthState {
        redirect_url: url.to_string(),
        state_nonce,
    })
}

/// Validate the OAuth callback and exchange the code for an access token.
///
/// Validates the HMAC signature and state nonce, then POSTs to the
/// authorization server's token endpoint to exchange the code.
///
/// Mirrors `shopify.auth.callback()` from `@shopify/shopify-api`.
pub async fn callback(
    config: &Config,
    http: &reqwest::Client,
    params: CallbackParams,
    state_cookie: &str,
) -> Result<CallbackResponse> {
    if config.is_custom_store_app {
        return Err(ShopifyError::PrivateAppError(
            "Cannot perform OAuth for custom store apps".to_string(),
        ));
    }

    // 1. Validate CSRF state
    if params.state != state_cookie {
        return Err(ShopifyError::InvalidOAuth(
            "OAuth state mismatch — possible CSRF attack".to_string(),
        ));
    }

    // 2. Validate HMAC signature of the callback query parameters
    if !validate_callback_hmac(&config.api_secret_key, &params) {
        return Err(ShopifyError::InvalidOAuth(
            "OAuth callback HMAC validation failed".to_string(),
        ));
    }

    // 3. Exchange the authorization code for an access token
    #[derive(Serialize)]
    struct TokenRequest<'a> {
        client_id: &'a str,
        client_secret: &'a str,
        code: &'a str,
    }

    let token_url = format!("https://{}/admin/oauth/access_token", params.shop);
    let token_body = TokenRequest {
        client_id: &config.api_key,
        client_secret: &config.api_secret_key,
        code: &params.code,
    };

    let response = http
        .post(&token_url)
        .json(&token_body)
        .header("Accept", "application/json")
        .send()
        .await
        .map_err(|e| ShopifyError::InvalidOAuth(format!("Token request failed: {e}")))?;

    if !response.status().is_success() {
        let status = response.status().as_u16();
        let body = response.text().await.unwrap_or_default();
        return Err(ShopifyError::InvalidOAuth(format!(
            "Token endpoint returned HTTP {status}: {body}"
        )));
    }

    let token_data: AccessTokenResponse = response
        .json()
        .await
        .map_err(|e| ShopifyError::InvalidOAuth(format!("Token response parse error: {e}")))?;

    // 4. Build the session
    let mut session = if params.is_online {
        Session::new_online(&params.shop, &params.state)
    } else {
        Session::new_offline(&params.shop, &params.state)
    };

    session.access_token = Some(token_data.access_token);
    session.scope = Some(token_data.scope);

    if let Some(expires_in) = token_data.expires_in {
        session.expires =
            Some(chrono::Utc::now() + chrono::Duration::seconds(expires_in));
    }

    if let Some(user) = token_data.associated_user {
        session.online_access_info = Some(OnlineAccessInfo {
            expires_in: token_data.expires_in,
            associated_user_scope: token_data.associated_user_scope,
            associated_user: AssociatedUser {
                id: Some(user.id),
                first_name: user.first_name,
                last_name: user.last_name,
                email: user.email,
                email_verified: user.email_verified.unwrap_or(false),
                account_owner: user.account_owner.unwrap_or(false),
                locale: user.locale,
                collaborator: user.collaborator.unwrap_or(false),
            },
        });
    }

    Ok(CallbackResponse { session })
}

/// Validate the HMAC signature of the OAuth callback query parameters.
///
/// Mirrors `validateHmac()` + `safeCompare()` from `@shopify/shopify-api`.
fn validate_callback_hmac(secret: &str, params: &CallbackParams) -> bool {
    // Build the message: sorted key=value pairs, excluding "hmac" and "signature"
    let mut parts: Vec<(&str, &str)> = vec![
        ("code", &params.code),
        ("shop", &params.shop),
        ("state", &params.state),
    ];
    if let Some(ts) = &params.timestamp {
        parts.push(("timestamp", ts));
    }
    parts.sort_by_key(|(k, _)| *k);

    let message = parts
        .iter()
        .map(|(k, v)| format!("{}={}", k, v))
        .collect::<Vec<_>>()
        .join("&");

    let computed = compute_hmac_hex(secret, message.as_bytes());
    constant_time_compare(&computed, &params.hmac)
}

/// Compute HMAC-SHA256 and return as a lowercase hex string.
pub fn compute_hmac_hex(secret: &str, data: &[u8]) -> String {
    let mut mac = HmacSha256::new_from_slice(secret.as_bytes())
        .expect("HMAC accepts any key length");
    mac.update(data);
    hex::encode(mac.finalize().into_bytes())
}

/// Constant-time string comparison (prevents timing oracle attacks).
///
/// Mirrors `safeCompare()` from `@shopify/shopify-api`.
pub fn constant_time_compare(a: &str, b: &str) -> bool {
    if a.len() != b.len() {
        return false;
    }
    a.as_bytes().ct_eq(b.as_bytes()).into()
}
