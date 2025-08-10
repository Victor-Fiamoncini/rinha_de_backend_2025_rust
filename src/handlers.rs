use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde_json::json;

use crate::{
    dtos::{CreatePaymentDTO, GetPaymentsSummaryDTO},
    AppState,
};

pub async fn create_payment(
    State(state): State<AppState>,
    Json(payment): Json<CreatePaymentDTO>,
) -> impl IntoResponse {
    match state
        .services
        .create_payment_service
        .create_payment(payment)
        .await
    {
        Ok(_) => StatusCode::CREATED,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

pub async fn get_payments_summary(
    State(state): State<AppState>,
    query: Query<GetPaymentsSummaryDTO>,
) -> impl IntoResponse {
    let from = query.from;
    let to = query.to;

    match state
        .services
        .get_payment_summary_service
        .get_payment_summary(from, to)
        .await
    {
        Ok(_) => Ok(Json(json!("Payment summary fetched successfully"))),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
