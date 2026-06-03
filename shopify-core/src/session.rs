//! Session management for the Shopify Admin API.
//!
//! Mirrors the TypeScript `Session` class from `@shopify/shopify-api`.

use crate::auth::scopes::AuthScopes;
use crate::error::{Result, ShopifyError};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// User information for online (per-user) sessions.
///
/// Present only when `is_online == true`.
/// 
/// RUST TIP: `#[derive(...)]` is a macro that automatically generates boilerplate code 
/// for this struct. Here it implements traits like `Debug` (so we can print it), 
/// `Clone` (so we can duplicate it), and `Serialize`/`Deserialize` (to convert it to/from JSON).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct OnlineAccessInfo {
    /// Seconds until the online token expires.
    /// RUST TIP: `Option<T>` is an enum that can be either `Some(T)` or `None`.
    /// Rust doesn't have `null` or `undefined`, so `Option` is how you represent optional values.
    pub expires_in: Option<i64>,
    /// Scopes granted to the associated user.
    pub associated_user_scope: Option<String>,
    /// The staff user associated with this online session.
    pub associated_user: AssociatedUser,
}

/// Staff user record attached to an online session.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AssociatedUser {
    /// Shopify user ID.
    pub id: Option<i64>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
    pub email_verified: bool,
    pub account_owner: bool,
    pub locale: Option<String>,
    pub collaborator: bool,
}

/// A fully authenticated session representing a single shop-app installation.
///
/// Mirrors the TypeScript `Session` class from `@shopify/shopify-api` with
/// full field parity and equivalent helper methods.
///
/// Sessions are either:
/// - **Offline** (`is_online = false`) — app-wide, long-lived, id = `"offline_{shop}"`
/// - **Online** (`is_online = true`) — per-user, short-lived, id = UUID
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    /// Unique session identifier.
    /// Offline: `"offline_{shop}"`. Online: UUID v4.
    pub id: String,
    /// The shop's domain (e.g. `"my-store.myplatform.com"`).
    pub shop: String,
    /// OAuth state nonce — used only during the auth flow to prevent CSRF.
    pub state: String,
    /// `true` for per-user (online) tokens, `false` for app-wide (offline) tokens.
    pub is_online: bool,
    /// Comma-separated granted OAuth scopes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
    /// The access token for API authentication.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_token: Option<String>,
    /// When the access token expires (primarily for online sessions).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires: Option<DateTime<Utc>>,
    /// Refresh token (if your platform supports token refresh).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_token: Option<String>,
    /// When the refresh token expires.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_token_expires: Option<DateTime<Utc>>,
    /// Staff user info — present only for online sessions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub online_access_info: Option<OnlineAccessInfo>,
}

/// RUST TIP: `impl Session` is where we define methods for the `Session` struct.
/// It's the Rust equivalent of a `class` body in TypeScript containing methods.
impl Session {
    /// Create a new offline (app-wide) session shell.
    ///
    /// `access_token` and `scope` must be populated after the OAuth callback.
    /// 
    /// RUST TIP: `impl Into<String>` allows the function to accept anything that 
    /// can be converted into a `String` (like a string literal `&str` or a `String` object).
    /// This makes the function much easier to call.
    pub fn new_offline(shop: impl Into<String>, state: impl Into<String>) -> Self {
        // `.into()` performs the conversion into an owned `String`
        let shop = shop.into();
        let id = Self::offline_id(&shop);
        Self {
            id,
            shop,
            state: state.into(),
            is_online: false,
            scope: None,
            access_token: None,
            expires: None,
            refresh_token: None,
            refresh_token_expires: None,
            online_access_info: None,
        }
    }

    /// Create a new online (per-user) session shell.
    ///
    /// `access_token`, `scope`, and `expires` must be populated after the OAuth callback.
    pub fn new_online(shop: impl Into<String>, state: impl Into<String>) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            shop: shop.into(),
            state: state.into(),
            is_online: true,
            scope: None,
            access_token: None,
            expires: None,
            refresh_token: None,
            refresh_token_expires: None,
            online_access_info: None,
        }
    }

    /// The canonical offline session ID for a shop.
    ///
    /// Mirrors `Session.offlineId(shop)` from `@shopify/shopify-api`.
    pub fn offline_id(shop: &str) -> String {
        format!("offline_{}", shop)
    }

    /// Returns `true` if the session has a valid, non-expired access token
    /// whose scopes satisfy `required_scopes`.
    ///
    /// - `within_ms` — treat the token as expired this many milliseconds early
    ///   (default `500` in the TS SDK, pass `0` for exact comparison).
    ///
    /// Mirrors `session.isActive(scopes)` from `@shopify/shopify-api`.
    pub fn is_active(&self, required_scopes: &AuthScopes, within_ms: i64) -> bool {
        self.access_token.is_some()
            && !self.is_expired(within_ms)
            && !self.is_scope_changed(required_scopes)
    }

    /// Returns `true` if the access token is expired (or will be within `within_ms` ms).
    ///
    /// Mirrors `session.isExpired(withinMillisecondsOfExpiry)`.
    pub fn is_expired(&self, within_ms: i64) -> bool {
        match self.expires {
            Some(expires) => {
                let buffer = chrono::Duration::milliseconds(within_ms);
                expires - buffer < Utc::now()
            }
            None => false,
        }
    }

    /// Returns `true` if the session's scopes do not satisfy `required`.
    ///
    /// Mirrors `session.isScopeChanged(scopes)`.
    pub fn is_scope_changed(&self, required: &AuthScopes) -> bool {
        if required.is_empty() {
            return false;
        }
        let session_scopes = AuthScopes::new(self.scope.as_deref().unwrap_or(""));
        !session_scopes.has(required)
    }

    /// Returns `true` if the session's scopes are a superset of `required`.
    pub fn is_scope_included(&self, required: &AuthScopes) -> bool {
        let session_scopes = AuthScopes::new(self.scope.as_deref().unwrap_or(""));
        session_scopes.has(required)
    }

    /// Serialize the session to a flat key-value list for database storage.
    ///
    /// Mirrors `session.toPropertyArray(returnUserData)` from `@shopify/shopify-api`.
    pub fn to_property_array(&self, include_user_data: bool) -> Vec<(String, String)> {
        let mut entries = vec![
            ("id".to_string(), self.id.clone()),
            ("shop".to_string(), self.shop.clone()),
            ("state".to_string(), self.state.clone()),
            ("isOnline".to_string(), self.is_online.to_string()),
        ];

        if let Some(scope) = &self.scope {
            entries.push(("scope".to_string(), scope.clone()));
        }
        if let Some(token) = &self.access_token {
            entries.push(("accessToken".to_string(), token.clone()));
        }
        if let Some(exp) = self.expires {
            entries.push(("expires".to_string(), exp.timestamp_millis().to_string()));
        }
        if let Some(rt) = &self.refresh_token {
            entries.push(("refreshToken".to_string(), rt.clone()));
        }
        if let Some(rte) = self.refresh_token_expires {
            entries.push((
                "refreshTokenExpires".to_string(),
                rte.timestamp_millis().to_string(),
            ));
        }

        if let Some(info) = &self.online_access_info {
            if include_user_data {
                if let Some(id) = info.associated_user.id {
                    entries.push(("userId".to_string(), id.to_string()));
                }
                if let Some(n) = &info.associated_user.first_name {
                    entries.push(("firstName".to_string(), n.clone()));
                }
                if let Some(n) = &info.associated_user.last_name {
                    entries.push(("lastName".to_string(), n.clone()));
                }
                if let Some(e) = &info.associated_user.email {
                    entries.push(("email".to_string(), e.clone()));
                }
                if let Some(l) = &info.associated_user.locale {
                    entries.push(("locale".to_string(), l.clone()));
                }
                entries.push((
                    "emailVerified".to_string(),
                    info.associated_user.email_verified.to_string(),
                ));
                entries.push((
                    "accountOwner".to_string(),
                    info.associated_user.account_owner.to_string(),
                ));
                entries.push((
                    "collaborator".to_string(),
                    info.associated_user.collaborator.to_string(),
                ));
            } else if let Some(uid) = info.associated_user.id {
                entries.push(("onlineAccessInfo".to_string(), uid.to_string()));
            }
        }

        entries
    }

    /// Reconstruct a session from stored key-value pairs.
    ///
    /// Mirrors `Session.fromPropertyArray(entries)` from `@shopify/shopify-api`.
    pub fn from_property_array(entries: &[(String, String)]) -> Result<Self> {
        let mut id = String::new();
        let mut shop = String::new();
        let mut state = String::new();
        let mut is_online = false;
        let mut scope: Option<String> = None;
        let mut access_token: Option<String> = None;
        let mut expires: Option<DateTime<Utc>> = None;
        let mut refresh_token: Option<String> = None;
        let mut refresh_token_expires: Option<DateTime<Utc>> = None;
        let mut online_info: Option<OnlineAccessInfo> = None;

        for (key, value) in entries {
            match key.as_str() {
                "id" => id = value.clone(),
                "shop" => shop = value.clone(),
                "state" => state = value.clone(),
                "isOnline" => is_online = value == "true" || value == "1",
                "scope" => scope = Some(value.clone()),
                "accessToken" => access_token = Some(value.clone()),
                "expires" => {
                    if let Ok(ms) = value.parse::<i64>() {
                        expires = DateTime::from_timestamp_millis(ms);
                    }
                }
                "refreshToken" => refresh_token = Some(value.clone()),
                "refreshTokenExpires" => {
                    if let Ok(ms) = value.parse::<i64>() {
                        refresh_token_expires = DateTime::from_timestamp_millis(ms);
                    }
                }
                k @ ("userId" | "onlineAccessInfo" | "firstName" | "lastName"
                | "email" | "locale" | "emailVerified" | "accountOwner"
                | "collaborator") => {
                    let info = online_info.get_or_insert_with(OnlineAccessInfo::default);
                    let user = &mut info.associated_user;
                    match k {
                        "userId" | "onlineAccessInfo" => {
                            user.id = value.parse::<i64>().ok();
                        }
                        "firstName" => user.first_name = Some(value.clone()),
                        "lastName" => user.last_name = Some(value.clone()),
                        "email" => user.email = Some(value.clone()),
                        "locale" => user.locale = Some(value.clone()),
                        "emailVerified" => user.email_verified = value == "true",
                        "accountOwner" => user.account_owner = value == "true",
                        "collaborator" => user.collaborator = value == "true",
                        _ => {}
                    }
                }
                _ => {}
            }
        }

        if id.is_empty() || shop.is_empty() {
            return Err(ShopifyError::InvalidSession(
                "Session requires at least 'id' and 'shop' fields".to_string(),
            ));
        }

        Ok(Self {
            id,
            shop,
            state,
            is_online,
            scope,
            access_token,
            expires,
            refresh_token,
            refresh_token_expires,
            online_access_info: online_info,
        })
    }

    /// Returns `true` if both sessions have the same identity and token.
    ///
    /// Mirrors `session.equals(other)` from `@shopify/shopify-api`.
    pub fn equals(&self, other: &Session) -> bool {
        self.id == other.id
            && self.shop == other.shop
            && self.state == other.state
            && self.is_online == other.is_online
            && self.scope == other.scope
            && self.access_token == other.access_token
            && self.expires == other.expires
    }
}
