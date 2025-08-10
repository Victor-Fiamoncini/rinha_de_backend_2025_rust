use chrono::{DateTime, Utc};

use crate::queue::Queue;

#[derive(Clone)]
pub struct GetPaymentSummaryService {
    queue: Queue,
}

impl GetPaymentSummaryService {
    pub fn new(queue: Queue) -> Self {
        GetPaymentSummaryService { queue }
    }

    pub async fn get_payment_summary(
        &self,
        from: DateTime<Utc>,
        to: DateTime<Utc>,
    ) -> Result<(), &'static str> {
        println!("Fetching payment summary from {} to {}", from, to);

        Ok(())
    }
}
