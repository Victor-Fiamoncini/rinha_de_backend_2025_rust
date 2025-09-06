use crate::{dto::create_payment::CreatePaymentDTO, infra::redis_queue::RedisQueue};

#[derive(Clone)]
pub struct CreatePaymentService {
    redis_queue: RedisQueue,
}

impl CreatePaymentService {
    pub fn new(redis_queue: RedisQueue) -> Self {
        Self { redis_queue }
    }

    pub async fn execute(&self, payment: CreatePaymentDTO) -> Result<(), &'static str> {
        let json_parsed_payment = match serde_json::to_string(&payment) {
            Ok(value) => value,
            Err(_) => return Err("Failed to serialize payment JSON to string"),
        };

        self.redis_queue.enqueue_right(json_parsed_payment).await
    }
}
