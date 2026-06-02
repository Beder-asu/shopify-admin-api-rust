//! SDK logger with configurable log levels.
//!
//! Mirrors `ShopifyLogger` from `@shopify/shopify-api`.

use crate::config::{LogConfig, LogLevel};
use std::sync::Arc;

/// SDK logger — respects the configured log level.
///
/// Obtain one via `ShopifyApp.logger` or construct directly for testing.
///
/// Mirrors `ShopifyLogger` from `@shopify/shopify-api`.
#[derive(Clone)]
pub struct Logger {
    config: Arc<LogConfig>,
}

impl Logger {
    /// Create a new logger with the given configuration.
    pub fn new(config: Arc<LogConfig>) -> Self {
        Self { config }
    }

    /// Log a debug-level message (only emitted when level ≤ Debug).
    pub fn debug(&self, msg: &str) {
        if self.config.level <= LogLevel::Debug {
            self.emit("DEBUG", msg);
        }
    }

    /// Log an info-level message.
    pub fn info(&self, msg: &str) {
        if self.config.level <= LogLevel::Info {
            self.emit("INFO", msg);
        }
    }

    /// Log a warning-level message.
    pub fn warning(&self, msg: &str) {
        if self.config.level <= LogLevel::Warning {
            self.emit("WARN", msg);
        }
    }

    /// Log an error-level message (always emitted).
    pub fn error(&self, msg: &str) {
        self.emit("ERROR", msg);
    }

    fn emit(&self, level: &str, msg: &str) {
        if self.config.timestamps {
            eprintln!(
                "[{}] [{}] {}",
                chrono::Utc::now().format("%Y-%m-%dT%H:%M:%SZ"),
                level,
                msg
            );
        } else {
            eprintln!("[{}] {}", level, msg);
        }
    }
}
