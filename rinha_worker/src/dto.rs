use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PendingPaymentDTO {
    #[serde(rename = "requestedAt")]
    #[serde(default)]
    pub requested_at: String,
    #[serde(rename = "correlationId")]
    pub correlation_id: String,
    pub amount: f64,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum PaymentProcessor {
    #[serde(rename = "default")]
    Default,
    #[serde(rename = "fallback")]
    Fallback,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CompletedPaymentDTO {
    pub amount: f64,
    pub processor_name: PaymentProcessor,
    pub created_at: String,
}
