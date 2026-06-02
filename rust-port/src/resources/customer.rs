//! Customer resource
//!
//! Represents a customer in the Shopify store.
//! Includes `CustomerAddress` sub-resource CRUD.

use crate::{
    base::{CountResponse, FieldsParam, FindAllResponse},
    client::Client,
    error::Result,
};
// Problem 18 fix: removed unused `use async_trait::async_trait`
use serde::{Deserialize, Serialize};

use super::metafield::Metafield;
use super::order::Order;

/// Customer resource
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Customer {
    /// Unique identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// Email address
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// Whether the customer accepts marketing
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accepts_marketing: Option<bool>,
    /// When marketing acceptance was updated
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accepts_marketing_updated_at: Option<String>,
    /// Customer's addresses
    #[serde(skip_serializing_if = "Option::is_none")]
    pub addresses: Option<Vec<CustomerAddress>>,
    /// Currency used by customer
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    /// When customer was created
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// Default address
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_address: Option<CustomerAddress>,
    /// First name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    /// Last name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    /// Last order ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_order_id: Option<i64>,
    /// Last order name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_order_name: Option<String>,
    /// Associated metafield
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metafield: Option<Metafield>,
    /// Marketing opt-in level
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marketing_opt_in_level: Option<String>,
    /// Multipass identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multipass_identifier: Option<String>,
    /// Notes about the customer
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    /// Total number of orders
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orders_count: Option<i64>,
    /// Phone number
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    /// SMS marketing consent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sms_marketing_consent: Option<SmsMarketingConsent>,
    /// Account state
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// Tags associated with customer
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<String>,
    /// Whether customer is tax exempt
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_exempt: Option<bool>,
    /// Tax exemptions
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_exemptions: Option<Vec<String>>,
    /// Total amount spent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_spent: Option<String>,
    /// When customer was last updated
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    /// Whether email is verified
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verified_email: Option<bool>,
}

/// Customer address (also a sub-resource with full CRUD)
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CustomerAddress {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address1: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address2: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub province: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zip: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub province_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<bool>,
}

/// SMS marketing consent
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SmsMarketingConsent {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opt_in_level: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consent_updated_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consent_collected_from: Option<String>,
}

/// Parameters for listing customers
#[derive(Debug, Clone, Default, Serialize)]
#[derive(derive_builder::Builder)]
#[builder(setter(into, strip_option), default)]
pub struct CustomerListParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ids: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub since_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at_min: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at_max: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at_min: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at_max: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<String>,
}

/// Parameters for counting customers
#[derive(Debug, Clone, Default, Serialize)]
#[derive(derive_builder::Builder)]
#[builder(setter(into, strip_option), default)]
pub struct CustomerCountParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at_min: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at_max: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at_min: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at_max: Option<String>,
}

/// Parameters for searching customers
#[derive(Debug, Clone, Default, Serialize)]
#[derive(derive_builder::Builder)]
#[builder(setter(into, strip_option), default)]
pub struct CustomerSearchParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<String>,
}

/// Parameters for getting customer orders
#[derive(Debug, Clone, Default, Serialize)]
#[derive(derive_builder::Builder)]
#[builder(setter(into, strip_option), default)]
pub struct CustomerOrdersParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

// ── Internal wrappers ─────────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
struct CustomerWrapper {
    customer: Customer,
}

#[derive(Debug, Deserialize)]
struct CustomersWrapper {
    customers: Vec<Customer>,
}

#[derive(Debug, Serialize)]
struct CustomerRequest {
    customer: Customer,
}

#[derive(Debug, Deserialize)]
struct AccountActivationUrlResponse {
    account_activation_url: String,
}

#[derive(Debug, Deserialize)]
struct CustomerInviteResponse {
    customer_invite: serde_json::Value,
}

#[derive(Debug, Deserialize)]
struct OrdersWrapper {
    orders: Vec<Order>,
}

#[derive(Debug, Deserialize)]
struct CustomerAddressWrapper {
    customer_address: CustomerAddress,
}

#[derive(Debug, Deserialize)]
struct CustomerAddressesWrapper {
    addresses: Vec<CustomerAddress>,
}

#[derive(Debug, Serialize)]
struct CustomerAddressRequest {
    address: CustomerAddress,
}

// ── Customer impl ─────────────────────────────────────────────────────────────

impl Customer {
    /// Find a customer by ID
    pub async fn find(client: &Client, id: i64) -> Result<Option<Self>> {
        let path = format!("customers/{}.json", id);
        let response = client.get::<CustomerWrapper>(&path).await?;
        Ok(Some(response.data.customer))
    }

    /// Find a customer by ID with specific fields.
    ///
    /// Problem 9 fix: uses `get_with_params` for proper URL encoding instead of
    /// embedding `?fields=` directly in the path string.
    pub async fn find_with_fields(client: &Client, id: i64, fields: &str) -> Result<Option<Self>> {
        let path = format!("customers/{}.json", id);
        let response = client
            .get_with_params::<CustomerWrapper, _>(&path, &FieldsParam { fields })
            .await?;
        Ok(Some(response.data.customer))
    }

    /// List all customers
    pub async fn all(client: &Client, params: CustomerListParams) -> Result<FindAllResponse<Self>> {
        let response = client
            .get_with_params::<CustomersWrapper, _>("customers.json", &params)
            .await?;

        Ok(FindAllResponse {
            data: response.data.customers,
            next_page: response.page_info.as_ref().and_then(|p| p.next.clone()),
            previous_page: response.page_info.as_ref().and_then(|p| p.previous.clone()),
        })
    }

    /// Count customers
    pub async fn count(client: &Client, params: CustomerCountParams) -> Result<i64> {
        let response = client
            .get_with_params::<CountResponse, _>("customers/count.json", &params)
            .await?;
        Ok(response.data.count)
    }

    /// Search customers by query
    pub async fn search(client: &Client, params: CustomerSearchParams) -> Result<Vec<Self>> {
        let response = client
            .get_with_params::<CustomersWrapper, _>("customers/search.json", &params)
            .await?;
        Ok(response.data.customers)
    }

    /// Get orders placed by this customer.
    ///
    /// Problem 16 fix: returns concrete `Vec<Order>` instead of `serde_json::Value`.
    pub async fn orders(
        client: &Client,
        id: i64,
        params: CustomerOrdersParams,
    ) -> Result<Vec<Order>> {
        let path = format!("customers/{}/orders.json", id);
        let response = client
            .get_with_params::<OrdersWrapper, _>(&path, &params)
            .await?;
        Ok(response.data.orders)
    }

    /// Create a new customer
    pub async fn create(client: &Client, customer: &Self) -> Result<Self> {
        let request = CustomerRequest {
            customer: customer.clone(),
        };
        let response = client
            .post::<CustomerWrapper, _>("customers.json", &request)
            .await?;
        Ok(response.data.customer)
    }

    /// Update an existing customer
    pub async fn update(client: &Client, customer: &Self) -> Result<Self> {
        let id = customer.id.ok_or_else(|| {
            crate::ShopifyError::ValidationError("Customer ID is required for update".to_string())
        })?;
        let path = format!("customers/{}.json", id);
        let request = CustomerRequest {
            customer: customer.clone(),
        };
        let response = client.put::<CustomerWrapper, _>(&path, &request).await?;
        Ok(response.data.customer)
    }

    /// Save the customer (create if new, update if existing)
    pub async fn save(client: &Client, customer: &Self) -> Result<Self> {
        if customer.id.is_some() {
            Self::update(client, customer).await
        } else {
            Self::create(client, customer).await
        }
    }

    /// Delete a customer
    pub async fn delete(client: &Client, id: i64) -> Result<()> {
        let path = format!("customers/{}.json", id);
        client.delete(&path).await
    }

    /// Generate an account activation URL for the customer
    pub async fn account_activation_url(client: &Client, id: i64) -> Result<String> {
        let path = format!("customers/{}/account_activation_url.json", id);
        let response = client
            .post::<AccountActivationUrlResponse, _>(&path, &serde_json::json!({}))
            .await?;
        Ok(response.data.account_activation_url)
    }

    /// Send an account invite email to the customer
    pub async fn send_invite(
        client: &Client,
        id: i64,
        invite: Option<serde_json::Value>,
    ) -> Result<serde_json::Value> {
        let path = format!("customers/{}/send_invite.json", id);
        let body = invite.unwrap_or(serde_json::json!({}));
        let response = client
            .post::<CustomerInviteResponse, _>(&path, &body)
            .await?;
        Ok(response.data.customer_invite)
    }
}

// ── CustomerAddress impl ──────────────────────────────────────────────────────
//
// Problem 12 fix: full CRUD for the nested /customers/{id}/addresses sub-resource.

impl CustomerAddress {
    /// List all addresses for a customer.
    ///
    /// `GET /customers/{customer_id}/addresses.json`
    pub async fn all(client: &Client, customer_id: i64) -> Result<Vec<Self>> {
        let path = format!("customers/{}/addresses.json", customer_id);
        let response = client.get::<CustomerAddressesWrapper>(&path).await?;
        Ok(response.data.addresses)
    }

    /// Retrieve a single address.
    ///
    /// `GET /customers/{customer_id}/addresses/{address_id}.json`
    pub async fn find(client: &Client, customer_id: i64, id: i64) -> Result<Option<Self>> {
        let path = format!("customers/{}/addresses/{}.json", customer_id, id);
        let response = client.get::<CustomerAddressWrapper>(&path).await?;
        Ok(Some(response.data.customer_address))
    }

    /// Create a new address for a customer.
    ///
    /// `POST /customers/{customer_id}/addresses.json`
    pub async fn create(client: &Client, customer_id: i64, address: &Self) -> Result<Self> {
        let path = format!("customers/{}/addresses.json", customer_id);
        let request = CustomerAddressRequest {
            address: address.clone(),
        };
        let response = client
            .post::<CustomerAddressWrapper, _>(&path, &request)
            .await?;
        Ok(response.data.customer_address)
    }

    /// Update an existing address.
    ///
    /// `PUT /customers/{customer_id}/addresses/{address_id}.json`
    pub async fn update(client: &Client, customer_id: i64, address: &Self) -> Result<Self> {
        let id = address.id.ok_or_else(|| {
            crate::ShopifyError::ValidationError(
                "Address ID is required for update".to_string(),
            )
        })?;
        let path = format!("customers/{}/addresses/{}.json", customer_id, id);
        let request = CustomerAddressRequest {
            address: address.clone(),
        };
        let response = client
            .put::<CustomerAddressWrapper, _>(&path, &request)
            .await?;
        Ok(response.data.customer_address)
    }

    /// Delete an address.
    ///
    /// `DELETE /customers/{customer_id}/addresses/{address_id}.json`
    pub async fn delete(client: &Client, customer_id: i64, id: i64) -> Result<()> {
        let path = format!("customers/{}/addresses/{}.json", customer_id, id);
        client.delete(&path).await
    }

    /// Set an address as the customer's default.
    ///
    /// `PUT /customers/{customer_id}/addresses/{address_id}/default.json`
    pub async fn set_default(client: &Client, customer_id: i64, id: i64) -> Result<Self> {
        let path = format!(
            "customers/{}/addresses/{}/default.json",
            customer_id, id
        );
        let response = client
            .put::<CustomerAddressWrapper, _>(&path, &serde_json::json!({}))
            .await?;
        Ok(response.data.customer_address)
    }
}
