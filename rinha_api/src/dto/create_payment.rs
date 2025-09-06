use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct CreatePaymentDTO {
    #[serde(rename = "correlationId")]
    pub correlation_id: String,
    pub amount: f64,
}
