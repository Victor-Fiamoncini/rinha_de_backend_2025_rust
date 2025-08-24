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
