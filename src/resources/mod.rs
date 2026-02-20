//! Shopify REST API Resources
//! 
//! This module contains all the resource types for the Shopify Admin REST API.

// Orders & Checkout
pub mod order;
pub mod draft_order;
pub mod checkout;
pub mod abandoned_checkout;
pub mod transaction;
pub mod refund;

// Products & Inventory
pub mod product;
pub mod variant;
pub mod image;
pub mod collection;
pub mod custom_collection;
pub mod smart_collection;
pub mod collect;
pub mod inventory_item;
pub mod inventory_level;

// Fulfillment
pub mod fulfillment;
pub mod fulfillment_order;
pub mod fulfillment_request;
pub mod fulfillment_event;
pub mod fulfillment_service;
pub mod assigned_fulfillment_order;
pub mod cancellation_request;
pub mod carrier_service;
pub mod locations_for_move;
pub mod shipping_zone;

// Customers
pub mod customer;
pub mod metafield;

// Billing & Payments
pub mod application_charge;
pub mod recurring_application_charge;
pub mod usage_charge;
pub mod application_credit;
pub mod payment_gateway;
pub mod payment;
pub mod payment_transaction;
pub mod gift_card;
pub mod gift_card_adjustment;

// Store & Configuration
pub mod shop;
pub mod policy;
pub mod country;
pub mod province;
pub mod currency;
pub mod location;
pub mod theme;
pub mod asset;

// Content
pub mod blog;
pub mod article;
pub mod comment;
pub mod page;
pub mod redirect;
pub mod script_tag;

// Marketing & Sales
pub mod price_rule;
pub mod discount_code;
pub mod marketing_event;
pub mod product_listing;
pub mod collection_listing;
pub mod product_resource_feedback;
pub mod resource_feedback;

// Financial
pub mod balance;
pub mod payout;
pub mod dispute;
pub mod dispute_evidence;
pub mod dispute_file_upload;
pub mod tender_transaction;

// Risk & Order Management
pub mod order_risk;

// Integrations
pub mod webhook;
pub mod event;
pub mod storefront_access_token;
pub mod access_scope;
pub mod mobile_platform_application;
pub mod apple_pay_certificate;
pub mod deprecated_api_call;

// Users
pub mod user;

// Re-exports for convenience
pub use abandoned_checkout::AbandonedCheckout;
pub use access_scope::AccessScope;
pub use apple_pay_certificate::ApplePayCertificate;
pub use application_charge::ApplicationCharge;
pub use application_credit::ApplicationCredit;
pub use article::Article;
pub use asset::Asset;
pub use assigned_fulfillment_order::AssignedFulfillmentOrder;
pub use balance::Balance;
pub use blog::Blog;
pub use cancellation_request::CancellationRequest;
pub use carrier_service::CarrierService;
pub use checkout::Checkout;
pub use collect::Collect;
pub use collection::Collection;
pub use collection_listing::CollectionListing;
pub use comment::Comment;
pub use country::Country;
pub use currency::Currency;
pub use custom_collection::CustomCollection;
pub use customer::Customer;
pub use deprecated_api_call::DeprecatedApiCall;
pub use discount_code::DiscountCode;
pub use dispute::Dispute;
pub use dispute_evidence::DisputeEvidence;
pub use dispute_file_upload::DisputeFileUpload;
pub use draft_order::DraftOrder;
pub use event::Event;
pub use fulfillment::Fulfillment;
pub use fulfillment_event::FulfillmentEvent;
pub use fulfillment_order::FulfillmentOrder;
pub use fulfillment_request::FulfillmentRequest;
pub use fulfillment_service::FulfillmentService;
pub use gift_card::GiftCard;
pub use gift_card_adjustment::GiftCardAdjustment;
pub use image::Image;
pub use inventory_item::InventoryItem;
pub use inventory_level::InventoryLevel;
pub use location::Location;
pub use locations_for_move::LocationsForMove;
pub use marketing_event::MarketingEvent;
pub use metafield::Metafield;
pub use mobile_platform_application::MobilePlatformApplication;
pub use order::Order;
pub use order_risk::OrderRisk;
pub use page::Page;
pub use payment::Payment;
pub use payment_gateway::PaymentGateway;
pub use payment_transaction::PaymentTransaction;
pub use payout::Payout;
pub use policy::Policy;
pub use price_rule::PriceRule;
pub use product::Product;
pub use product_listing::ProductListing;
pub use product_resource_feedback::ProductResourceFeedback;
pub use province::Province;
pub use recurring_application_charge::RecurringApplicationCharge;
pub use redirect::Redirect;
pub use refund::Refund;
pub use resource_feedback::ResourceFeedback;
pub use script_tag::ScriptTag;
pub use shipping_zone::ShippingZone;
pub use shop::Shop;
pub use smart_collection::SmartCollection;
pub use storefront_access_token::StorefrontAccessToken;
pub use tender_transaction::TenderTransaction;
pub use theme::Theme;
pub use transaction::Transaction;
pub use usage_charge::UsageCharge;
pub use user::User;
pub use variant::Variant;
pub use webhook::Webhook;
