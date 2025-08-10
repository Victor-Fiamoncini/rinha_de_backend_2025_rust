mod create_payment_service;

use crate::{queue::Queue, services::create_payment_service::CreatePaymentService};

#[derive(Clone)]
pub struct Services {
    pub create_payment_service: CreatePaymentService,
}

impl Services {
    pub fn new(payments_queue: Queue) -> Self {
        Services {
            create_payment_service: CreatePaymentService::new(payments_queue),
        }
    }
}
