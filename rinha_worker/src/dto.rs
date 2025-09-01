use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PaymentDTO {
    pub amount: Decimal,
    #[serde(rename = "correlationId")]
    pub correlation_id: String,
    #[serde(default)]
    pub payment_processor: String,
    #[serde(rename = "requestedAt")]
    #[serde(default)]
    pub requested_at: String,
}

#[derive(Debug)]
pub enum PaymentProcessor {
    Default,
    Fallback,
}
