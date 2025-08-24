mod config;
mod dto;
mod queue;
mod route;
mod service;

use std::sync::Arc;

use axum::{
    routing::{get, post},
    Router,
};
use tracing::info;
use tracing_subscriber::fmt;

use crate::{
    config::Config,
    queue::Queue,
    route::{create_payment, get_payments_summary},
    service::Services,
};

#[derive(Clone)]
pub struct AppState {
    services: Arc<Services>,
}

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

    info!("🦀 rinha_api listening on 0.0.0.0:{}", config.api_port);

    axum::serve(listener, app).await.unwrap();
}
