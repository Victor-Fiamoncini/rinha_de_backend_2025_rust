use chrono::{DateTime, NaiveDateTime};
use tokio_postgres::{types::ToSql, Row};
use tracing::error;

use crate::{
    dto::get_payments_summary::{PaymentMetricDTO, PaymentSummaryDTO},
    infra::sql_database::SqlDatabase,
};

#[derive(Clone)]
pub struct GetPaymentsSummaryService {
    sql_database: SqlDatabase,
}

impl GetPaymentsSummaryService {
    pub fn new(sql_database: SqlDatabase) -> Self {
        Self { sql_database }
    }

    fn create_summary_query(
        &self,
        from: &Option<String>,
        to: &Option<String>,
    ) -> (String, Vec<NaiveDateTime>) {
        let mut query = String::from(
            "SELECT payment_processor, COUNT(1) AS count, SUM(amount) AS total_amount FROM payments",
        );

        let mut params = Vec::new();
        let mut param_index = 1;

        let mut from_parsed = None;

        if let Some(from_date) = from {
            let parsed = DateTime::parse_from_rfc3339(&from_date)
                .map(|dt| dt.naive_utc())
                .or_else(|_| NaiveDateTime::parse_from_str(&from_date, "%Y-%m-%dT%H:%M:%S"));

            match parsed {
                Ok(datetime) => {
                    from_parsed = Some(datetime);
                }
                Err(_) => {
                    error!("Failed to parse 'from' param to datetime");
                }
            }
        }

        let mut to_parsed = None;

        if let Some(to_date) = to {
            let parsed = DateTime::parse_from_rfc3339(&to_date)
                .map(|dt| dt.naive_utc())
                .or_else(|_| NaiveDateTime::parse_from_str(&to_date, "%Y-%m-%dT%H:%M:%S"));

            match parsed {
                Ok(datetime) => {
                    to_parsed = Some(datetime);
                }
                Err(_) => {
                    error!("Failed to parse 'to' param to datetime");
                }
            }
        }

        if from_parsed.is_some() || to_parsed.is_some() {
            query.push_str(" WHERE");

            if let Some(parsed_date) = from_parsed {
                query.push_str(&format!(" requested_at >= ${}", param_index));

                params.push(parsed_date);

                param_index += 1;
            }

            if let Some(parsed_date) = to_parsed {
                if param_index > 1 {
                    query.push_str(" AND");
                }

                query.push_str(&format!(" requested_at <= ${}", param_index));

                params.push(parsed_date);
            }
        }

        query.push_str(" GROUP BY payment_processor");

        (query, params)
    }

    fn build_payment_summary(&self, rows: Vec<Row>) -> PaymentSummaryDTO {
        let mut default_metric = PaymentMetricDTO {
            total_requests: 0,
            total_amount: Default::default(),
        };

        let mut fallback_metric = PaymentMetricDTO {
            total_requests: 0,
            total_amount: Default::default(),
        };

        for row in rows {
            let payment_processor: String = row.get(0);

            if payment_processor == "default" {
                let requests: i64 = row.get(1);

                default_metric.total_requests = requests.try_into().unwrap();
                default_metric.total_amount = row.get(2);
            } else {
                let requests: i64 = row.get(1);

                fallback_metric.total_requests = requests.try_into().unwrap();
                fallback_metric.total_amount = row.get(2);
            }
        }

        PaymentSummaryDTO {
            default: default_metric,
            fallback: fallback_metric,
        }
    }

    pub async fn execute(&self, from: Option<String>, to: Option<String>) -> PaymentSummaryDTO {
        let (query, params) = self.create_summary_query(&from, &to);

        let rows: Vec<Row> = self
            .sql_database
            .query(
                &query,
                &params
                    .iter()
                    .map(|param| param as &(dyn ToSql + Sync))
                    .collect::<Vec<_>>(),
            )
            .await
            .unwrap();

        self.build_payment_summary(rows)
    }
}
