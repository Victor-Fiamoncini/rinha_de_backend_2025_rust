use deadpool_postgres::{Config as PostgresConfig, Pool, Runtime};
use tokio_postgres::{types::ToSql, NoTls};
use tracing::{error, info};

use crate::config::Config;

#[derive(Clone)]
pub struct Database {
    pool: Pool,
}

impl Database {
    pub async fn new(config: Config) -> Self {
        let mut postgres_config = PostgresConfig::new();

        postgres_config.host = Some(config.postgres_host);
        postgres_config.port = Some(config.postgres_port);
        postgres_config.user = Some(config.postgres_user);
        postgres_config.password = Some(config.postgres_password);
        postgres_config.dbname = Some(config.postgres_db);
        postgres_config.manager = Some(deadpool_postgres::ManagerConfig {
            recycling_method: deadpool_postgres::RecyclingMethod::Fast,
        });
        postgres_config.pool = Some(deadpool_postgres::PoolConfig::new(20));

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
            Err(err) => {
                error!("Failed to insert into {}: {}", table, err);

                Err("Failed to insert")
            }
        }
    }
}
