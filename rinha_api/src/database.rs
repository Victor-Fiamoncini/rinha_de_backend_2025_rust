use deadpool_postgres::{Config as PostgresConfig, Pool, Runtime};
use tokio_postgres::{types::ToSql, NoTls, Row};
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
            Err(err) => {
                error!("Failed to connect to PostgreSQL database: {}", err);

                panic!("Failed to connect to PostgreSQL database");
            }
        }

        Self { pool }
    }

    pub async fn query<'a>(
        &self,
        query: &str,
        params: &[&'a (dyn ToSql + Sync)],
    ) -> Result<Vec<Row>, &'static str> {
        let client = match self.pool.get().await {
            Ok(client) => client,
            Err(_) => return Err("Failed to get database connection"),
        };

        match client.query(query, params).await {
            Ok(rows) => Ok(rows),
            Err(_) => {
                error!("Database query error");

                Err("Query execution failed")
            }
        }
    }
}
