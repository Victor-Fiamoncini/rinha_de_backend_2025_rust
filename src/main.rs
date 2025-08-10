mod config;
mod consumers;
mod dtos;
mod handlers;
mod queue;
mod services;

use std::sync::Arc;

use axum::{
    routing::{get, post},
    Router,
};
use dotenv::dotenv;

use crate::{
    config::Config,
    consumers::PaymentConsumer,
    handlers::{create_payment, get_payments_summary},
    queue::Queue,
    services::{CreateExternalPaymentService, Services},
};

#[derive(Clone)]
pub struct AppState {
    services: Arc<Services>,
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let config = Config::new();

    let pending_payments_queue = Queue::new(config.clone(), "@pending_payments_queue").await;
    let completed_payments_queue = Queue::new(config.clone(), "@completed_payments_queue").await;

    let app_state = AppState {
        services: Arc::new(Services::new(
            pending_payments_queue.clone(),
            completed_payments_queue.clone(),
        )),
    };

    let create_external_payment_service = CreateExternalPaymentService::new(config.clone());

    let payment_consumer = PaymentConsumer::new(
        create_external_payment_service,
        config.clone(),
        pending_payments_queue.clone(),
    );

    payment_consumer.consume_payments().await;

    let app = Router::new()
        .route("/payments", post(create_payment))
        .route("/payments-summary", get(get_payments_summary))
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind(("0.0.0.0", config.api_port))
        .await
        .unwrap();

    println!("🦀 Crab server listening on 0.0.0.0:{}", config.api_port);

    axum::serve(listener, app).await.unwrap();
}
