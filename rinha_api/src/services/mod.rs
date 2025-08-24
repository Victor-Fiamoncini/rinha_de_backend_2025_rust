mod create_payment_service;
mod get_payment_summary_service;

use crate::{
    queue::Queue,
    services::{
        create_payment_service::CreatePaymentService,
        get_payment_summary_service::GetPaymentSummaryService,
    },
};

#[derive(Clone)]
pub struct Services {
    pub create_payment_service: CreatePaymentService,
    pub get_payment_summary_service: GetPaymentSummaryService,
}

impl Services {
    pub fn new(pending_payments_queue: Queue, completed_payments_queue: Queue) -> Self {
        Services {
            create_payment_service: CreatePaymentService::new(pending_payments_queue),
            get_payment_summary_service: GetPaymentSummaryService::new(completed_payments_queue),
        }
    }
}
