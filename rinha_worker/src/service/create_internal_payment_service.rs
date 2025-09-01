use chrono::{DateTime, Utc};
use tokio_postgres::types::ToSql;
use tracing::{error, info};
use uuid::Uuid;

use crate::{database::Database, dto::PaymentDTO};

#[derive(Clone)]
pub struct CreateInternalPaymentService {
    database: Database,
}

impl CreateInternalPaymentService {
    pub fn new(database: Database) -> Self {
        CreateInternalPaymentService { database }
    }

    pub async fn execute(&self, payment: PaymentDTO) -> Result<(), &'static str> {
        let correlation_id = match Uuid::parse_str(&payment.correlation_id) {
            Ok(uuid) => uuid,
            Err(_) => {
                error!("Failed to parse UUID '{}'", payment.correlation_id);

                Uuid::new_v4()
            }
        };

        let requested_at = match DateTime::parse_from_rfc3339(&payment.requested_at) {
            Ok(datetime) => datetime.with_timezone(&Utc),
            Err(_) => {
                error!("Failed to parse timestamp '{}'", payment.requested_at);

                Utc::now()
            }
        };

        let naive_requested_at = requested_at.naive_utc();

        let fields = &[
            "amount",
            "correlation_id",
            "payment_processor",
            "requested_at",
        ];

        let values: &[&(dyn ToSql + Sync)] = &[
            &payment.amount,
            &correlation_id,
            &payment.payment_processor,
            &naive_requested_at,
        ];

        match self.database.insert("payments", fields, values).await {
            Ok(_) => {
                info!("Successfully created internal payment");

                Ok(())
            }
            Err(_) => {
                error!("Failed to create internal payment");

                Err("Failed to create internal payment")
            }
        }
    }
}
