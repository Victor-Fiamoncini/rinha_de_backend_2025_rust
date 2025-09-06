use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use tokio_postgres::types::ToSql;
use tracing::{error, info};
use uuid::Uuid;

use crate::{dto::create_internal_payment::PaymentDTO, infra::sql_database::SqlDatabase};

#[derive(Clone)]
pub struct CreateInternalPaymentService {
    sql_database: SqlDatabase,
}

impl CreateInternalPaymentService {
    pub fn new(sql_database: SqlDatabase) -> Self {
        Self { sql_database }
    }

    pub async fn execute(&self, payment: PaymentDTO) -> Result<(), &'static str> {
        let amount = match Decimal::try_from(payment.amount) {
            Ok(decimal) => decimal,
            Err(_) => {
                error!("Failed to parse amount to decimal");

                return Err("Invalid amount format");
            }
        };

        let correlation_id = match Uuid::parse_str(&payment.correlation_id) {
            Ok(uuid) => uuid,
            Err(_) => {
                error!("Failed to parse UUID '{}'", payment.correlation_id);

                return Err("Invalid correlation_id format");
            }
        };

        let requested_at = match DateTime::parse_from_rfc3339(&payment.requested_at) {
            Ok(datetime) => datetime.with_timezone(&Utc),
            Err(_) => {
                error!("Failed to parse timestamp '{}'", payment.requested_at);

                return Err("Invalid requested_at format");
            }
        };

        let requested_at = requested_at.naive_utc();

        let fields = &[
            "amount",
            "correlation_id",
            "payment_processor",
            "requested_at",
        ];

        let values: &[&(dyn ToSql + Sync)] = &[
            &amount,
            &correlation_id,
            &payment.payment_processor,
            &requested_at,
        ];

        match self.sql_database.insert("payments", fields, values).await {
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
