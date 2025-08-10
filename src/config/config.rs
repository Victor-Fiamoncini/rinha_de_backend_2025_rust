use std::env;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Config {
    pub api_port: u16,
    pub payment_processor_default_url: String,
    pub payment_processor_fallback_url: String,
    pub redis_url: String,
}

impl Config {
    pub fn new() -> Self {
        let api_port = env::var("API_PORT")
            .ok()
            .and_then(|port| port.parse::<u16>().ok())
            .expect("API_PORT env must be set");

        let payment_processor_default_url = env::var("PAYMENT_PROCESSOR_DEFAULT_URL")
            .expect("PAYMENT_PROCESSOR_DEFAULT_URL env must be set");

        let payment_processor_fallback_url = env::var("PAYMENT_PROCESSOR_FALLBACK_URL")
            .expect("PAYMENT_PROCESSOR_FALLBACK_URL env must be set");

        let redis_url = env::var("REDIS_URL").expect("REDIS_URL env must be set");

        Self {
            api_port,
            payment_processor_default_url,
            payment_processor_fallback_url,
            redis_url,
        }
    }
}
