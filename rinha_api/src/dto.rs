use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct CreatePaymentDTO {
    #[serde(rename = "correlationId")]
    pub correlation_id: String,
    pub amount: Decimal,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GetPaymentsSummaryDTO {
    pub from: Option<String>,
    pub to: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PaymentMetricDTO {
    #[serde(rename = "totalRequests")]
    pub total_requests: u64,
    #[serde(rename = "totalAmount")]
    #[serde(serialize_with = "crate::serializers::serialize")]
    pub total_amount: Decimal,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PaymentSummaryDTO {
    pub default: PaymentMetricDTO,
    pub fallback: PaymentMetricDTO,
}
