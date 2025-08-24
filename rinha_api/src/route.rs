use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use crate::{
    dto::{CreatePaymentDTO, GetPaymentsSummaryDTO},
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
    match state
        .services
        .get_payment_summary_service
        .get_payment_summary(query.from, query.to)
        .await
    {
        Ok(summary) => Ok(Json(serde_json::to_value(summary).unwrap())),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
