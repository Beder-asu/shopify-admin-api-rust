//! Cryptographic nonce generation for the OAuth state parameter.
//!
//! Mirrors `nonce.ts` from `@shopify/shopify-api`.

use rand::Rng;

/// Generate a cryptographically secure random nonce.
///
/// Returns a 32-character lowercase hex string (16 random bytes).
///
/// Mirrors `nonce()` from `@shopify/shopify-api`.
pub fn generate_nonce() -> String {
    let bytes: [u8; 16] = rand::thread_rng().gen();
    hex::encode(bytes)
}
