use std::time::Duration;

use chrono::Utc;
use tracing::{error, info};

use crate::{
    dto::PaymentDTO,
    queue::Queue,
    service::{CreateExternalPaymentService, CreateInternalPaymentService, PaymentProcessors},
};

pub struct PaymentConsumer {
    create_external_payment: CreateExternalPaymentService,
    create_internal_payment: CreateInternalPaymentService,
    pending_payments_queue: Queue,
}

impl PaymentConsumer {
    pub fn new(
        create_external_payment: CreateExternalPaymentService,
        create_internal_payment: CreateInternalPaymentService,
        pending_payments_queue: Queue,
    ) -> Self {
        PaymentConsumer {
            create_external_payment,
            create_internal_payment,
            pending_payments_queue,
        }
    }

    pub async fn consume_payments(&self) {
        loop {
            let dequeue_result = self.pending_payments_queue.dequeue().await;

            let payment = match dequeue_result {
                Ok(Some(message)) => match serde_json::from_str::<PaymentDTO>(&message) {
                    Ok(mut payment) => {
                        payment.requested_at = Utc::now().to_rfc3339();

                        Some(payment)
                    }
                    Err(e) => {
                        info!("Failed to deserialize payment message: {}", e);

                        None
                    }
                },
                Ok(None) => {
                    info!("No payment messages found, waiting...");

                    tokio::time::sleep(Duration::from_secs(3)).await;

                    None
                }
                Err(_) => {
                    error!("Error while dequeuing payment message, retrying...");

                    tokio::time::sleep(Duration::from_secs(3)).await;

                    None
                }
            };

            if let Some(payment) = payment {
                let default_result = self
                    .create_external_payment
                    .create_external_payment(PaymentProcessors::Default, payment.clone())
                    .await;

                match default_result {
                    Ok(_) => {
                        if let Err(e) = self
                            .create_internal_payment
                            .create_payment(payment.clone())
                            .await
                        {
                            error!("Failed to create internal payment (default): {:?}", e);
                        }
                    }
                    Err(_) => {
                        let fallback_result = self
                            .create_external_payment
                            .create_external_payment(PaymentProcessors::Fallback, payment.clone())
                            .await;

                        match fallback_result {
                            Ok(_) => {
                                if let Err(e) = self
                                    .create_internal_payment
                                    .create_payment(payment.clone())
                                    .await
                                {
                                    error!("Failed to create internal payment (fallback): {:?}", e);
                                }
                            }
                            Err(_) => match serde_json::to_string(&payment) {
                                Ok(json) => {
                                    if let Err(e) = self.pending_payments_queue.enqueue(json).await
                                    {
                                        error!("Failed to enqueue payment for requeue: {:?}", e);
                                    }
                                }
                                Err(_) => {
                                    error!("Failed to serialize payment for requeue");
                                }
                            },
                        }
                    }
                }
            }
        }
    }
}
