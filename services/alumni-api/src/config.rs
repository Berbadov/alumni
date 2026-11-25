use std::env;

use crate::middleware::DEFAULT_ERROR_LOG;

fn env_flag(key: &str, default: bool) -> bool {
    match env::var(key) {
        Ok(value) => matches!(
            value.trim().to_ascii_lowercase().as_str(),
            "1" | "true" | "yes" | "on"
        ),
        Err(_) => default,
    }
}

#[derive(Clone)]
pub struct Config {
    pub mongo_uri: String,
    pub mongo_db: String,
    pub bind_addr: String,
    pub error_log: String,
    pub require_email_verification: bool,
    pub require_sms_verification: bool,
    pub bootstrap_admin_email: Option<String>,
    pub bootstrap_admin_password: Option<String>,
}

fn optional_non_empty(key: &str) -> Option<String> {
    env::var(key)
        .ok()
        .map(|value| value.trim().to_owned())
        .filter(|value| !value.is_empty())
}

impl Config {
    pub fn from_env() -> Self {
        Self {
            mongo_uri: env::var("MONGO_URI")
                .unwrap_or_else(|_| "mongodb://mongo:27017".to_string()),
            mongo_db: env::var("MONGO_DB").unwrap_or_else(|_| "alumni".to_string()),
            bind_addr: env::var("BIND_ADDR")
                .unwrap_or_else(|_| "0.0.0.0:8080".to_string()),
            error_log: env::var("ERROR_LOG")
                .unwrap_or_else(|_| DEFAULT_ERROR_LOG.to_string()),
            require_email_verification: env_flag("REQUIRE_EMAIL_VERIFICATION", false),
            require_sms_verification: env_flag("REQUIRE_SMS_VERIFICATION", false),
            bootstrap_admin_email: optional_non_empty("BOOTSTRAP_ADMIN_EMAIL"),
            bootstrap_admin_password: optional_non_empty("BOOTSTRAP_ADMIN_PASSWORD"),
        }
    }
}
