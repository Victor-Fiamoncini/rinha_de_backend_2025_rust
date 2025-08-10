use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct CreatePaymentDTO {
    #[serde(rename = "correlationId")]
    pub correlation_id: String,
    pub amount: f64,
}

#[derive(Deserialize)]
pub struct GetPaymentsSummaryDTO {
    pub from: DateTime<Utc>,
    pub to: DateTime<Utc>,
}
