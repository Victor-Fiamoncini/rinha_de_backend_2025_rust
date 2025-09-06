use std::time::Duration;

use chrono::Utc;
use tracing::info;

use crate::{
    dto::{create_external_payment::PaymentProcessor, create_internal_payment::PaymentDTO},
    infra::redis_queue::RedisQueue,
    service::{CreateExternalPaymentService, CreateInternalPaymentService},
};

const MAX_DEQUEUE_RETRIES: usize = 3;
const NUMBER_OF_WORKERS: usize = 5;

pub struct PaymentConsumer {
    create_external_payment: CreateExternalPaymentService,
    create_internal_payment: CreateInternalPaymentService,
    pending_payments_queue: RedisQueue,
}

impl PaymentConsumer {
    pub fn new(
        create_external_payment: CreateExternalPaymentService,
        create_internal_payment: CreateInternalPaymentService,
        pending_payments_queue: RedisQueue,
    ) -> Self {
        Self {
            create_external_payment,
            create_internal_payment,
            pending_payments_queue,
        }
    }

    pub async fn consume_payments(&self) -> () {
        for worker_id in 0..NUMBER_OF_WORKERS {
            let create_external_payment = self.create_external_payment.clone();
            let create_internal_payment = self.create_internal_payment.clone();
            let pending_payments_queue = self.pending_payments_queue.clone();

            tokio::spawn(async move {
                info!("🦀 rinha_worker -> worker {} started", worker_id + 1);

                let mut empty_count: usize = 0;

                loop {
                    let result = if empty_count < MAX_DEQUEUE_RETRIES {
                        pending_payments_queue.dequeue_left().await
                    } else {
                        pending_payments_queue.dequeue_left_blocking(0.1).await
                    };

                    match result {
                        Ok(Some(message)) => {
                            empty_count = 0;

                            let mut payment = match serde_json::from_str::<PaymentDTO>(&message) {
                                Ok(payment) => payment,
                                Err(_) => continue,
                            };

                            payment.requested_at = Utc::now().to_rfc3339();

                            match create_external_payment
                                .execute(PaymentProcessor::Default, payment.clone())
                                .await
                            {
                                Ok(_) => {
                                    payment.payment_processor = "default".to_string();

                                    let _ = create_internal_payment.execute(payment).await;
                                }
                                Err(_) => {
                                    let _ = pending_payments_queue.enqueue_right(message).await;
                                }
                            }
                        }
                        Ok(None) => {
                            empty_count += 1;

                            if empty_count <= MAX_DEQUEUE_RETRIES {
                                tokio::time::sleep(Duration::from_millis(1)).await;
                            }
                        }
                        Err(_) => {
                            tokio::time::sleep(Duration::from_secs(1)).await;
                        }
                    }
                }
            });
        }
    }
}
