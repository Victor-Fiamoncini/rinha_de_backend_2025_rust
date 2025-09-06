use redis::{aio::ConnectionManager, AsyncCommands, Client};

use crate::config::Config;

#[derive(Clone)]
pub struct RedisQueue {
    connection: ConnectionManager,
    name: &'static str,
}

impl RedisQueue {
    pub async fn new(config: Config, name: &'static str) -> Self {
        let client = match Client::open(config.redis.url) {
            Ok(client) => client,
            Err(_) => panic!("Failed to create Redis Client"),
        };

        let connection = ConnectionManager::new(client)
            .await
            .expect("Failed to create Redis ConnectionManager");

        Self { connection, name }
    }

    pub async fn enqueue_right(&self, message: String) -> Result<(), &'static str> {
        let mut connection = self.connection.clone();

        match connection.rpush::<_, _, ()>(&self.name, message).await {
            Ok(_) => Ok(()),
            Err(_) => Err("Failed to enqueue message on Redis queue"),
        }
    }
}
