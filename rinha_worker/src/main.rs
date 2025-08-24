mod config;
mod consumers;
mod dtos;
mod queue;
mod services;

use tracing::info;
use tracing_subscriber::fmt;

use crate::{
    config::Config,
    consumers::PaymentConsumer,
    queue::Queue,
    services::{CreateExternalPaymentService, CreateInternalPaymentService},
};

const NUM_WORKERS: usize = 1;

#[tokio::main]
async fn main() {
    fmt()
        .with_target(false)
        .with_line_number(false)
        .with_file(false)
        .compact()
        .init();

    let config = Config::new();

    let pending_payments_queue = Queue::new(config.clone(), "@pending_payments_queue").await;
    let completed_payments_queue = Queue::new(config.clone(), "@completed_payments_queue").await;

    let create_external_payment = CreateExternalPaymentService::new(config.clone());
    let create_internal_payment =
        CreateInternalPaymentService::new(completed_payments_queue.clone());

    let mut handles = Vec::new();

    for worker_id in 0..NUM_WORKERS {
        let payment_consumer = PaymentConsumer::new(
            create_external_payment.clone(),
            create_internal_payment.clone(),
            pending_payments_queue.clone(),
        );

        let handle = tokio::spawn(async move {
            info!("Worker {} started", worker_id);

            payment_consumer.consume_payments().await;
        });

        handles.push(handle);
    }

    info!(
        "🦀 rinha_worker running with {} worker threads",
        NUM_WORKERS
    );

    futures::future::join_all(handles).await;

    info!("🦀 All rinha_worker workers have terminated. Shutting down...");
}
