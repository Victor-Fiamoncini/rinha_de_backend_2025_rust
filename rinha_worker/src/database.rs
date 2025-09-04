use deadpool_postgres::{
    Config as PostgresConfig, ManagerConfig, Pool, PoolConfig, RecyclingMethod, Runtime,
};
use tokio_postgres::{types::ToSql, NoTls};
use tracing::{error, info};

use crate::config::Config;

#[derive(Clone)]
pub struct Database {
    pool: Pool,
}

impl Database {
    pub async fn new(config: Config) -> Self {
        let postgres_config = PostgresConfig {
            host: Some(config.postgres_host),
            port: Some(config.postgres_port),
            user: Some(config.postgres_user),
            password: Some(config.postgres_password),
            dbname: Some(config.postgres_db),
            manager: Some(ManagerConfig {
                recycling_method: RecyclingMethod::Fast,
            }),
            pool: Some(PoolConfig::new(20)),
            ..Default::default()
        };

        let pool = postgres_config
            .create_pool(Some(Runtime::Tokio1), NoTls)
            .unwrap();

        match pool.get().await {
            Ok(_) => {
                info!("Successfully connected to PostgreSQL database");
            }
            Err(_) => {
                panic!("Failed to connect to PostgreSQL database");
            }
        }

        Self { pool }
    }

    pub async fn insert<'a>(
        &self,
        table: &str,
        fields: &[&str],
        values: &[&'a (dyn ToSql + Sync)],
    ) -> Result<(), &'static str> {
        if fields.is_empty() {
            return Err("No fields provided");
        }

        if fields.len() != values.len() {
            return Err("Field and value counts don't match");
        }

        let client = match self.pool.get().await {
            Ok(client) => client,
            Err(_) => return Err("Failed to get database connection"),
        };

        let placeholders: Vec<String> = (1..=fields.len()).map(|i| format!("${}", i)).collect();

        let query = format!(
            "INSERT INTO {} ({}) VALUES ({})",
            table,
            fields.join(", "),
            placeholders.join(", ")
        );

        match client.execute(query.as_str(), values).await {
            Ok(_) => Ok(()),
            Err(_) => {
                error!("Failed to insert into {} table", table);

                Err("Failed to insert")
            }
        }
    }
}
