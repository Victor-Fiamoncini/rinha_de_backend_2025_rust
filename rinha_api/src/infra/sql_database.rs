use deadpool_postgres::{
    Config as PostgresConfig, ManagerConfig, Pool, PoolConfig, RecyclingMethod, Runtime,
};
use tokio_postgres::{types::ToSql, NoTls, Row};
use tracing::{error, info};

use crate::config::Config;

#[derive(Clone)]
pub struct SqlDatabase {
    pool: Pool,
}

impl SqlDatabase {
    pub async fn new(config: Config) -> Self {
        let postgres_config = PostgresConfig {
            host: Some(config.postgres.host),
            port: Some(config.postgres.port),
            user: Some(config.postgres.user),
            password: Some(config.postgres.password),
            dbname: Some(config.postgres.db),
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
                error!("Failed to connect to PostgreSQL database");

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
