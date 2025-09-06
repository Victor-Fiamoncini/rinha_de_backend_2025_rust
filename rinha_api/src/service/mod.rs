mod create_payment_service;
mod get_payments_summary_service;

use crate::{
    infra::{redis_queue::RedisQueue, sql_database::SqlDatabase},
    service::{
        create_payment_service::CreatePaymentService,
        get_payments_summary_service::GetPaymentsSummaryService,
    },
};

#[derive(Clone)]
pub struct Services {
    pub create_payment_service: CreatePaymentService,
    pub get_payments_summary_service: GetPaymentsSummaryService,
}

impl Services {
    pub fn new(
        completed_payments_database: SqlDatabase,
        pending_payments_queue: RedisQueue,
    ) -> Self {
        Services {
            create_payment_service: CreatePaymentService::new(pending_payments_queue),
            get_payments_summary_service: GetPaymentsSummaryService::new(
                completed_payments_database,
            ),
        }
    }
}
