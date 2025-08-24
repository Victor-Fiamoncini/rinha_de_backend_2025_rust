use std::time::Duration;

use chrono::Utc;
use tracing::{error, info};

use crate::{
    dto::{CompletedPaymentDTO, PaymentProcessor, PendingPaymentDTO},
    queue::Queue,
    service::{CreateExternalPaymentService, CreateInternalPaymentService},
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
        let dequeue_result = self.pending_payments_queue.dequeue().await;

        let pending_payment: Option<PendingPaymentDTO> = match dequeue_result {
            Ok(Some(message)) => match serde_json::from_str::<PendingPaymentDTO>(&message) {
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

                tokio::time::sleep(Duration::from_millis(1)).await;

                None
            }
            Err(_) => {
                error!("Error while dequeuing payment message, retrying...");

                tokio::time::sleep(Duration::from_millis(1)).await;

                None
            }
        };

        if let Some(pending_payment) = pending_payment {
            let default_result = self
                .create_external_payment
                .create_external_payment(PaymentProcessor::Default, pending_payment.clone())
                .await;

            match default_result {
                Ok(_) => {
                    let completed_payment = CompletedPaymentDTO {
                        amount: pending_payment.amount,
                        processor_name: PaymentProcessor::Default,
                        created_at: pending_payment.requested_at,
                    };

                    if let Err(e) = self
                        .create_internal_payment
                        .create_payment(completed_payment)
                        .await
                    {
                        error!("Failed to create internal payment (default): {:?}", e);
                    }
                }
                Err(_) => {
                    let fallback_result = self
                        .create_external_payment
                        .create_external_payment(
                            PaymentProcessor::Fallback,
                            pending_payment.clone(),
                        )
                        .await;

                    match fallback_result {
                        Ok(_) => {
                            let completed_payment = CompletedPaymentDTO {
                                amount: pending_payment.amount,
                                processor_name: PaymentProcessor::Fallback,
                                created_at: pending_payment.requested_at,
                            };

                            if let Err(e) = self
                                .create_internal_payment
                                .create_payment(completed_payment)
                                .await
                            {
                                error!("Failed to create internal payment (fallback): {:?}", e);
                            }
                        }
                        Err(_) => match serde_json::to_string(&pending_payment) {
                            Ok(json) => {
                                if let Err(e) = self.pending_payments_queue.enqueue(json).await {
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
