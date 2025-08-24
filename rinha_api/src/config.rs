use std::env;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Config {
    pub api_port: u16,
    pub redis_url: String,
}

impl Config {
    pub fn new() -> Self {
        let api_port_str = env::var("API_PORT").expect("API_PORT env must be set");
        let api_port: u16 = api_port_str
            .parse()
            .expect("API_PORT env must be a valid u16");

        let redis_url = env::var("REDIS_URL").expect("REDIS_URL env must be set");

        Self {
            api_port,
            redis_url,
        }
    }
}
