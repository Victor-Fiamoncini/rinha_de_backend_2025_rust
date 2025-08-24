mod config;
mod consumers;
mod dtos;
mod queue;
mod services;

use crate::{
    config::Config, consumers::PaymentConsumer, queue::Queue,
    services::CreateExternalPaymentService,
};

#[tokio::main]
async fn main() {
    let config = Config::new();

    let pending_payments_queue = Queue::new(config.clone(), "@pending_payments_queue").await;
    let completed_payments_queue = Queue::new(config.clone(), "@completed_payments_queue").await;

    let create_external_payment_service = CreateExternalPaymentService::new(config.clone());

    let payment_consumer = PaymentConsumer::new(
        create_external_payment_service,
        config.clone(),
        pending_payments_queue.clone(),
    );

    tokio::spawn(async move { payment_consumer.consume_payments().await });

    println!("🦀 rinha_worker running");
}
