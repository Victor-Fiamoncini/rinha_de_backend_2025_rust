use chrono::{DateTime, Utc};
use tracing::info;

use crate::{
    dto::{CompletedPaymentDTO, PaymentSummaryDTO, ProcessorSummaryDTO},
    queue::Queue,
};

#[derive(Clone)]
pub struct GetPaymentSummaryService {
    queue: Queue,
}

impl GetPaymentSummaryService {
    pub fn new(queue: Queue) -> Self {
        GetPaymentSummaryService { queue }
    }

    pub async fn get_payment_summary(
        &self,
        from: Option<DateTime<Utc>>,
        to: Option<DateTime<Utc>>,
    ) -> Result<PaymentSummaryDTO, &'static str> {
        let messages = self.queue.get_all().await?;

        let mut default_total_requests = 0;
        let mut default_total_amount = 0.0;
        let mut fallback_total_requests = 0;
        let mut fallback_total_amount = 0.0;

        for message in messages {
            match serde_json::from_str::<CompletedPaymentDTO>(&message) {
                Ok(payment) => {
                    if let Ok(payment_date) = DateTime::parse_from_rfc3339(&payment.created_at) {
                        let payment_date_utc = payment_date.with_timezone(&Utc);

                        if (from.is_none() || payment_date_utc >= from.unwrap())
                            && (to.is_none() || payment_date_utc <= to.unwrap())
                        {
                            match payment.processor_name.as_str() {
                                "default" => {
                                    default_total_requests += 1;
                                    default_total_amount += payment.amount;
                                }
                                "fallback" => {
                                    fallback_total_requests += 1;
                                    fallback_total_amount += payment.amount;
                                }
                                _ => {}
                            }
                        }
                    }
                }
                Err(e) => {
                    info!("Error parsing payment message: {}", e);
                }
            }
        }

        Ok(PaymentSummaryDTO {
            default: ProcessorSummaryDTO {
                total_requests: default_total_requests,
                total_amount: default_total_amount,
            },
            fallback: ProcessorSummaryDTO {
                total_requests: fallback_total_requests,
                total_amount: fallback_total_amount,
            },
        })
    }
}
