use rust_decimal::prelude::ToPrimitive;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct CreatePaymentDTO {
    #[serde(rename = "correlationId")]
    pub correlation_id: String,
    pub amount: f64,
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
    #[serde(
        rename = "totalAmount",
        serialize_with = "PaymentMetricDTO::serialize_amount"
    )]
    pub total_amount: Decimal,
}

impl PaymentMetricDTO {
    fn serialize_amount<S>(decimal: &Decimal, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let float_val = decimal.to_f64().unwrap_or(0.0);

        serializer.serialize_f64(float_val)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PaymentSummaryDTO {
    pub default: PaymentMetricDTO,
    pub fallback: PaymentMetricDTO,
}
