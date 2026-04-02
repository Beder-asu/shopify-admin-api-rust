//! Core Data Models for Shopify
use serde::{Deserialize, Serialize};

/// Represents a Shopify Product
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Product {
    pub id: String,
    pub title: String,
    #[serde(rename = "handle")]
    pub handle: Option<String>,
    #[serde(rename = "vendor")]
    pub vendor: Option<String>,
}

/// Represents a Shopify Customer
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Customer {
    pub id: String,
    #[serde(rename = "firstName")]
    pub first_name: Option<String>,
    #[serde(rename = "lastName")]
    pub last_name: Option<String>,
    pub email: Option<String>,
}

/// Represents a Shopify Order
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Order {
    pub id: String,
    pub name: String,
    #[serde(rename = "totalPriceSet")]
    pub total_price_set: Option<MoneyBag>,
}

/// A money representation common in Shopify
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MoneyBag {
    #[serde(rename = "shopMoney")]
    pub shop_money: Money,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Money {
    pub amount: String,
    #[serde(rename = "currencyCode")]
    pub currency_code: String,
}
