//! OAuth scope types with implied-scope logic.
//!
//! Mirrors `AuthScopes` from `@shopify/shopify-api`.

use std::collections::BTreeSet;
use std::fmt;

/// A parsed, comparable set of OAuth scopes.
///
/// `write_X` implied scopes: asking `has("read_products")` returns `true` if
/// `write_products` is present, because write implies read.
///
/// Mirrors `AuthScopes` class from `@shopify/shopify-api`.
#[derive(Debug, Clone, Default)]
pub struct AuthScopes {
    scopes: BTreeSet<String>,
}

impl AuthScopes {
    /// Parse a comma-separated (or space-separated) scope string.
    ///
    /// ```
    /// use shopify_core::auth::scopes::AuthScopes;
    /// let s = AuthScopes::new("read_products,write_orders");
    /// ```
    pub fn new(raw: impl AsRef<str>) -> Self {
        let scopes = raw
            .as_ref()
            .split(|c| c == ',' || c == ' ')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();
        Self { scopes }
    }

    /// Build from a vector of scope strings.
    pub fn from_vec(scopes: Vec<impl Into<String>>) -> Self {
        Self {
            scopes: scopes.into_iter().map(|s| s.into()).collect(),
        }
    }

    /// Returns `true` if this scope set contains all scopes in `required`.
    ///
    /// `write_X` implies `read_X` automatically.
    ///
    /// Mirrors `AuthScopes.has()` from `@shopify/shopify-api`.
    pub fn has(&self, required: &AuthScopes) -> bool {
        required.scopes.iter().all(|scope| {
            self.scopes.contains(scope) || self.implied_by(scope)
        })
    }

    fn implied_by(&self, scope: &str) -> bool {
        if let Some(resource) = scope.strip_prefix("read_") {
            self.scopes.contains(&format!("write_{}", resource))
        } else {
            false
        }
    }

    /// Returns `true` if both scope sets represent the same effective permissions.
    pub fn equals(&self, other: &AuthScopes) -> bool {
        self.scopes == other.scopes
    }

    /// Returns `true` if no scopes are present.
    pub fn is_empty(&self) -> bool {
        self.scopes.is_empty()
    }

    /// Iterate over all scopes in alphabetical order.
    pub fn iter(&self) -> impl Iterator<Item = &String> {
        self.scopes.iter()
    }
}

impl fmt::Display for AuthScopes {
    /// Formats as a comma-separated string, alphabetically sorted.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            self.scopes
                .iter()
                .cloned()
                .collect::<Vec<_>>()
                .join(",")
        )
    }
}

impl From<&str> for AuthScopes {
    fn from(s: &str) -> Self {
        Self::new(s)
    }
}

impl From<String> for AuthScopes {
    fn from(s: String) -> Self {
        Self::new(s)
    }
}

impl From<Vec<String>> for AuthScopes {
    fn from(v: Vec<String>) -> Self {
        Self::from_vec(v)
    }
}

impl From<Vec<&str>> for AuthScopes {
    fn from(v: Vec<&str>) -> Self {
        Self::from_vec(v)
    }
}
