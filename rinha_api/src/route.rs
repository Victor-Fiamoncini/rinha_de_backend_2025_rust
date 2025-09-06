use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde_json::json;

use crate::{
    dto::{create_payment::CreatePaymentDTO, get_payments_summary::GetPaymentsSummaryDTO},
    AppState,
};

pub async fn create_payment(
    State(state): State<AppState>,
    Json(payment): Json<CreatePaymentDTO>,
) -> impl IntoResponse {
    match state.services.create_payment_service.execute(payment).await {
        Ok(_) => StatusCode::CREATED,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

pub async fn get_payments_summary(
    State(state): State<AppState>,
    query: Query<GetPaymentsSummaryDTO>,
) -> impl IntoResponse {
    let summary = state
        .services
        .get_payments_summary_service
        .execute(query.from.clone(), query.to.clone())
        .await;

    Json(json!(summary))
}
