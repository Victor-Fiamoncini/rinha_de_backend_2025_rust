mod config;
mod dto;
mod infra;
mod route;
mod service;

use std::sync::Arc;

use axum::{
    routing::{get, post},
    Router,
};
use tracing::info;
use tracing_subscriber;

use crate::{
    config::Config,
    infra::{redis_queue::RedisQueue, sql_database::SqlDatabase},
    route::{create_payment, get_payments_summary},
    service::Services,
};

#[derive(Clone)]
pub struct AppState {
    services: Arc<Services>,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let config = Config::new();

    let completed_payments_database = SqlDatabase::new(config.clone()).await;
    let pending_payments_queue = RedisQueue::new(config.clone(), "@pending_payments_queue").await;

    let app_state = AppState {
        services: Arc::new(Services::new(
            completed_payments_database,
            pending_payments_queue,
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
