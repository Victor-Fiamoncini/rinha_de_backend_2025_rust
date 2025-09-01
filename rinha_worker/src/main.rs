mod config;
mod consumer;
mod database;
mod dto;
mod queue;
mod service;

use std::time::Duration;

use tracing::info;
use tracing_subscriber::fmt;

use crate::{
    config::Config,
    consumer::PaymentConsumer,
    database::Database,
    queue::Queue,
    service::{CreateExternalPaymentService, CreateInternalPaymentService},
};

#[tokio::main]
async fn main() {
    fmt().compact().with_file(false).init();

    let config = Config::new();

    let pending_payments_queue = Queue::new(config.clone(), "@pending_payments_queue").await;
    let completed_payments_database = Database::new(config.clone()).await;

    let create_external_payment = CreateExternalPaymentService::new(config);
    let create_internal_payment = CreateInternalPaymentService::new(completed_payments_database);

    let payment_consumer = PaymentConsumer::new(
        create_external_payment,
        create_internal_payment,
        pending_payments_queue,
    );

    payment_consumer.consume_payments().await;

    loop {
        tokio::time::sleep(Duration::from_secs(60)).await;

        info!("🦀 rinha_worker running...");
    }
}
