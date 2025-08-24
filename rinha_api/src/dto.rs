use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct CreatePaymentDTO {
    #[serde(rename = "correlationId")]
    pub correlation_id: String,
    pub amount: f64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GetPaymentsSummaryDTO {
    pub from: Option<DateTime<Utc>>,
    pub to: Option<DateTime<Utc>>,
}

#[derive(Deserialize)]
pub struct CompletedPaymentDTO {
    pub amount: f64,
    pub processor_name: String,
    pub created_at: String,
}

#[derive(Serialize)]
pub struct ProcessorSummaryDTO {
    #[serde(rename = "totalRequests")]
    pub total_requests: u64,
    #[serde(rename = "totalAmount")]
    pub total_amount: f64,
}

#[derive(Serialize)]
pub struct PaymentSummaryDTO {
    pub default: ProcessorSummaryDTO,
    pub fallback: ProcessorSummaryDTO,
}
