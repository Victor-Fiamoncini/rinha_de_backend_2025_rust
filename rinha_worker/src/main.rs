mod config;
mod consumer;
mod dto;
mod infra;
mod service;

use std::time::Duration;

use tracing::info;
use tracing_subscriber;

use crate::{
    config::Config,
    consumer::PaymentConsumer,
    infra::{redis_queue::RedisQueue, sql_database::SqlDatabase},
    service::{CreateExternalPaymentService, CreateInternalPaymentService},
};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let config = Config::new();

    let pending_payments_queue = RedisQueue::new(config.clone(), "@pending_payments_queue").await;
    let completed_payments_database = SqlDatabase::new(config.clone()).await;

    let create_external_payment = CreateExternalPaymentService::new(config);
    let create_internal_payment = CreateInternalPaymentService::new(completed_payments_database);

    let payment_consumer = PaymentConsumer::new(
        create_external_payment,
        create_internal_payment,
        pending_payments_queue,
    );

    payment_consumer.consume_payments().await;

    info!("🦀 rinha_worker -> main thread started");

    loop {
        tokio::time::sleep(Duration::from_secs(3600)).await;
    }
}
