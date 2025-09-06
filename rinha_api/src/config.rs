use std::env;

#[derive(Clone, Debug)]
pub struct PostgresConfig {
    pub host: String,
    pub port: u16,
    pub db: String,
    pub user: String,
    pub password: String,
}

#[derive(Clone, Debug)]
pub struct RedisConfig {
    pub url: String,
}

#[derive(Clone, Debug)]
pub struct Config {
    pub api_port: u16,
    pub postgres: PostgresConfig,
    pub redis: RedisConfig,
}

impl Config {
    pub fn new() -> Self {
        let api_port_str = env::var("API_PORT").expect("API_PORT env must be set");
        let api_port: u16 = api_port_str
            .parse()
            .expect("API_PORT env must be a valid u16");

        let postgres_host = env::var("POSTGRES_HOST").expect("POSTGRES_HOST env must be set");
        let postgres_port_str = env::var("POSTGRES_PORT").expect("POSTGRES_PORT env must be set");
        let postgres_port: u16 = postgres_port_str
            .parse()
            .expect("POSTGRES_PORT env must be a valid u16");
        let postgres_db = env::var("POSTGRES_DB").expect("POSTGRES_DB env must be set");
        let postgres_user = env::var("POSTGRES_USER").expect("POSTGRES_USER env must be set");
        let postgres_password =
            env::var("POSTGRES_PASSWORD").expect("POSTGRES_PASSWORD env must be set");

        let redis_url = env::var("REDIS_URL").expect("REDIS_URL env must be set");

        Self {
            api_port,
            postgres: PostgresConfig {
                host: postgres_host,
                port: postgres_port,
                db: postgres_db,
                user: postgres_user,
                password: postgres_password,
            },
            redis: RedisConfig { url: redis_url },
        }
    }
}
