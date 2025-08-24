use redis::{aio::ConnectionManager, AsyncCommands, Client};

use crate::config::Config;

#[derive(Clone)]
pub struct Queue {
    connection: ConnectionManager,
    name: &'static str,
}

impl Queue {
    pub async fn new(config: Config, name: &'static str) -> Self {
        let client = match Client::open(config.redis_url) {
            Ok(client) => client,
            Err(_) => panic!("Failed to create Redis client"),
        };

        let connection = ConnectionManager::new(client)
            .await
            .expect("Failed to create Redis ConnectionManager");

        Queue { connection, name }
    }

    pub async fn enqueue(&self, message: String) -> Result<(), &'static str> {
        let mut connection = self.connection.clone();

        match connection.rpush::<_, _, ()>(&self.name, message).await {
            Ok(_) => Ok(()),
            Err(_) => Err("Failed to enqueue message on Redis queue"),
        }
    }

    pub async fn dequeue(&self) -> Result<Option<String>, &'static str> {
        let mut connection = self.connection.clone();

        match connection.lpop::<_, Option<String>>(&self.name, None).await {
            Ok(Some(value)) => Ok(Some(value)),
            Ok(None) => Ok(None),
            Err(_) => Err("Failed to dequeue message from Redis queue"),
        }
    }
}
