use std::env;

use crate::middleware::DEFAULT_ERROR_LOG;

#[derive(Clone)]
pub struct Config {
    pub mongo_uri: String,
    pub mongo_db: String,
    pub bind_addr: String,
    pub error_log: String,
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
        }
    }
}
