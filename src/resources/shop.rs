//! Shop resource
//! 
//! Represents the shop configuration and information.

use crate::{client::Client, error::Result};
use serde::{Deserialize, Serialize};

/// Shop resource
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Shop {
    /// Unique identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// Shop name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Shop email
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// Shop domain
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    /// Province
    #[serde(skip_serializing_if = "Option::is_none")]
    pub province: Option<String>,
    /// Country
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// Address line 1
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address1: Option<String>,
    /// Address line 2
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address2: Option<String>,
    /// Zip/postal code
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zip: Option<String>,
    /// City
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// Source
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    /// Phone number
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    /// Latitude
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latitude: Option<f64>,
    /// Longitude
    #[serde(skip_serializing_if = "Option::is_none")]
    pub longitude: Option<f64>,
    /// Primary locale
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_locale: Option<String>,
    /// Country code
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_code: Option<String>,
    /// Country name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_name: Option<String>,
    /// Currency
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    /// Customer email address
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_email: Option<String>,
    /// Timezone
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
    /// IANA timezone
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iana_timezone: Option<String>,
    /// Shop owner
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shop_owner: Option<String>,
    /// Money format
    #[serde(skip_serializing_if = "Option::is_none")]
    pub money_format: Option<String>,
    /// Money with currency format
    #[serde(skip_serializing_if = "Option::is_none")]
    pub money_with_currency_format: Option<String>,
    /// Weight unit
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight_unit: Option<String>,
    /// Province code
    #[serde(skip_serializing_if = "Option::is_none")]
    pub province_code: Option<String>,
    /// Whether taxes are included
    #[serde(skip_serializing_if = "Option::is_none")]
    pub taxes_included: Option<bool>,
    /// Auto configure tax inclusivity
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_configure_tax_inclusivity: Option<bool>,
    /// Tax shipping
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_shipping: Option<bool>,
    /// County taxes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub county_taxes: Option<bool>,
    /// Plan display name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan_display_name: Option<String>,
    /// Plan name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan_name: Option<String>,
    /// Has discounts
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_discounts: Option<bool>,
    /// Has gift cards
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_gift_cards: Option<bool>,
    /// Has storefront
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_storefront: Option<bool>,
    /// Myshopify domain
    #[serde(skip_serializing_if = "Option::is_none")]
    pub myshopify_domain: Option<String>,
    /// Google Apps domain
    #[serde(skip_serializing_if = "Option::is_none")]
    pub google_apps_domain: Option<String>,
    /// Google Apps login enabled
    #[serde(skip_serializing_if = "Option::is_none")]
    pub google_apps_login_enabled: Option<bool>,
    /// Money in emails format
    #[serde(skip_serializing_if = "Option::is_none")]
    pub money_in_emails_format: Option<String>,
    /// Money with currency in emails format
    #[serde(skip_serializing_if = "Option::is_none")]
    pub money_with_currency_in_emails_format: Option<String>,
    /// Eligible for payments
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eligible_for_payments: Option<bool>,
    /// Requires extra payments agreement
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requires_extra_payments_agreement: Option<bool>,
    /// Password enabled
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_enabled: Option<bool>,
    /// Multi-location enabled
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multi_location_enabled: Option<bool>,
    /// Setup required
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_required: Option<bool>,
    /// Pre-launch enabled
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pre_launch_enabled: Option<bool>,
    /// Enabled presentment currencies
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled_presentment_currencies: Option<Vec<String>>,
    /// Checkout API supported
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checkout_api_supported: Option<bool>,
    /// Transactional SMS disabled
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transactional_sms_disabled: Option<bool>,
    /// Marketing SMS consent enabled at checkout
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marketing_sms_consent_enabled_at_checkout: Option<bool>,
    /// Finances
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finances: Option<bool>,
    /// Force SSL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force_ssl: Option<bool>,
    /// When shop was created
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// When shop was last updated
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

/// Parameters for getting shop
#[derive(Debug, Clone, Default, Serialize)]
pub struct ShopParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<String>,
}

/// Wrapper for API responses
#[derive(Debug, Deserialize)]
struct ShopWrapper {
    shop: Shop,
}

impl Shop {
    /// Get the current shop information
    pub async fn current(client: &Client) -> Result<Self> {
        let response = client.get::<ShopWrapper>("shop.json").await?;
        Ok(response.data.shop)
    }

    /// Get the current shop information with specific fields
    pub async fn current_with_fields(client: &Client, fields: &str) -> Result<Self> {
        let params = ShopParams {
            fields: Some(fields.to_string()),
        };
        let response = client
            .get_with_params::<ShopWrapper, _>("shop.json", &params)
            .await?;
        Ok(response.data.shop)
    }
}
