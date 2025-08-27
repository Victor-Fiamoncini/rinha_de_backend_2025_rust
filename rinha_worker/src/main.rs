mod config;
mod consumer;
mod dto;
mod queue;
mod service;

use tracing::info;
use tracing_subscriber::fmt;

use crate::{
    config::Config,
    consumer::PaymentConsumer,
    queue::Queue,
    service::{CreateExternalPaymentService, CreateInternalPaymentService},
};

const NUM_WORKERS: u8 = 5;

#[tokio::main]
async fn main() {
    fmt().compact().with_file(false).init();

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
            info!("Worker {} started", worker_id + 1);

            loop {
                payment_consumer.consume_payments().await;
            }
        });

        handles.push(handle);
    }

    info!(
        "🦀 rinha_worker running with {} worker threads",
        NUM_WORKERS
    );

    futures::future::join_all(handles).await;

    info!("🦀 rinha_worker workers have terminated. Shutting down...");
}
