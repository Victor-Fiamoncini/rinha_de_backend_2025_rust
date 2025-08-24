mod config;
mod dtos;
mod handlers;
mod queue;
mod services;

use std::sync::Arc;

use axum::{
    routing::{get, post},
    Router,
};

use crate::{
    config::Config,
    handlers::{create_payment, get_payments_summary},
    queue::Queue,
    services::Services,
};

#[derive(Clone)]
pub struct AppState {
    services: Arc<Services>,
}

#[tokio::main]
async fn main() {
    let config = Config::new();

    let pending_payments_queue = Queue::new(config.clone(), "@pending_payments_queue").await;
    let completed_payments_queue = Queue::new(config.clone(), "@completed_payments_queue").await;

    let app_state = AppState {
        services: Arc::new(Services::new(
            pending_payments_queue,
            completed_payments_queue,
        )),
    };

    let app = Router::new()
        .route("/payments", post(create_payment))
        .route("/payments-summary", get(get_payments_summary))
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind(("0.0.0.0", config.api_port))
        .await
        .unwrap();

    println!("🦀 rinha_api listening on 0.0.0.0:{}", config.api_port);

    axum::serve(listener, app).await.unwrap();
}
