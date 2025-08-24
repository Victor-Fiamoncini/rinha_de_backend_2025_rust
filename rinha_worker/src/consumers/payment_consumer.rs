use std::time::Duration;

use chrono::Utc;

use crate::{
    config::Config,
    dtos::CreateExternalPaymentDTO,
    queue::Queue,
    services::{CreateExternalPaymentService, PaymentProcessors},
};

pub struct PaymentConsumer {
    create_external_payment_service: CreateExternalPaymentService,
    config: Config,
    pending_payments_queue: Queue,
}

impl PaymentConsumer {
    pub fn new(
        create_external_payment_service: CreateExternalPaymentService,
        config: Config,
        pending_payments_queue: Queue,
    ) -> Self {
        PaymentConsumer {
            create_external_payment_service,
            config,
            pending_payments_queue,
        }
    }

    pub async fn consume_payments(&self) {
        loop {
            let result = self.pending_payments_queue.dequeue().await;

            let payment = match result {
                Ok(Some(message)) => {
                    match serde_json::from_str::<CreateExternalPaymentDTO>(&message) {
                        Ok(mut payment) => {
                            payment.requested_at = Utc::now().to_rfc3339();

                            Some(payment)
                        }
                        Err(e) => {
                            println!("Failed to deserialize payment message: {}", e);

                            None
                        }
                    }
                }
                Ok(None) => {
                    println!("No payment messages found, waiting...");

                    tokio::time::sleep(Duration::from_secs(3)).await;

                    None
                }
                Err(_) => {
                    println!("Error while dequeuing payment message, retrying...");

                    tokio::time::sleep(Duration::from_secs(3)).await;

                    None
                }
            };

            if let Some(payment) = payment {
                let result = self
                    .create_external_payment_service
                    .create_external_payment(PaymentProcessors::Default, payment)
                    .await;

                println!("External payment result: {:?}", result);
            }
        }
    }
}
