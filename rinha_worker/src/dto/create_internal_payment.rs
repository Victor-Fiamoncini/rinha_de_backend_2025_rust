use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PaymentDTO {
    pub amount: f64,
    #[serde(rename = "correlationId")]
    pub correlation_id: String,
    #[serde(default)]
    pub payment_processor: String,
    #[serde(rename = "requestedAt")]
    #[serde(default)]
    pub requested_at: String,
}
