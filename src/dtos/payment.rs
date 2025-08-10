use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateExternalPaymentDTO {
    #[serde(rename = "requestedAt")]
    #[serde(default)]
    pub requested_at: String,
    #[serde(rename = "correlationId")]
    pub correlation_id: String,
    pub amount: f64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreatePaymentDTO {
    #[serde(rename = "correlationId")]
    pub correlation_id: String,
    pub amount: f64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GetPaymentsSummaryDTO {
    pub from: DateTime<Utc>,
    pub to: DateTime<Utc>,
}
