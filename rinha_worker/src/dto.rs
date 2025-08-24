use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PaymentDTO {
    #[serde(rename = "requestedAt")]
    #[serde(default)]
    pub requested_at: String,
    #[serde(rename = "correlationId")]
    pub correlation_id: String,
    pub amount: f64,
}
