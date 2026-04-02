//! Session management for Shopify API authentication

use serde::{Deserialize, Serialize};

/// Represents an authenticated session with a Shopify store
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    /// The shop's domain (e.g., "my-shop.myshopify.com")
    pub shop: String,
    /// The access token for API authentication
    pub access_token: String,
    /// Optional session state
    pub state: Option<String>,
    /// Whether this is an online (user) or offline (app) session
    pub is_online: bool,
    /// Expiration timestamp (for online sessions)
    pub expires: Option<chrono::DateTime<chrono::Utc>>,
    /// Associated user information (for online sessions)
    pub associated_user: Option<AssociatedUser>,
    /// OAuth scopes granted
    pub scope: Option<String>,
}

/// User information for online sessions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssociatedUser {
    pub id: i64,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
    pub email_verified: bool,
    pub account_owner: bool,
    pub locale: Option<String>,
    pub collaborator: bool,
}

impl Session {
    /// Create a new session with shop domain and access token
    pub fn new(shop: impl Into<String>, access_token: impl Into<String>) -> Self {
        Self {
            shop: shop.into(),
            access_token: access_token.into(),
            state: None,
            is_online: false,
            expires: None,
            associated_user: None,
            scope: None,
        }
    }

    /// Create an online session with user information
    pub fn online(
        shop: impl Into<String>,
        access_token: impl Into<String>,
        user: AssociatedUser,
        expires: chrono::DateTime<chrono::Utc>,
    ) -> Self {
        Self {
            shop: shop.into(),
            access_token: access_token.into(),
            state: None,
            is_online: true,
            expires: Some(expires),
            associated_user: Some(user),
            scope: None,
        }
    }

    /// Check if the session has expired
    pub fn is_expired(&self) -> bool {
        if let Some(expires) = self.expires {
            expires < chrono::Utc::now()
        } else {
            false
        }
    }

    /// Check if the session is valid
    pub fn is_valid(&self) -> bool {
        !self.access_token.is_empty() && !self.shop.is_empty() && !self.is_expired()
    }

    /// Get the base URL for API requests
    pub fn api_url(&self) -> String {
        format!("https://{}/admin/api/{}", self.shop, crate::API_VERSION)
    }
}
