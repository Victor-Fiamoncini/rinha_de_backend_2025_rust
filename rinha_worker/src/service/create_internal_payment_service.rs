use crate::{dto::PaymentDTO, queue::Queue};

#[derive(Clone)]
pub struct CreateInternalPaymentService {
    queue: Queue,
}

impl CreateInternalPaymentService {
    pub fn new(queue: Queue) -> Self {
        CreateInternalPaymentService { queue }
    }

    pub async fn create_payment(&self, payment: PaymentDTO) -> Result<(), &'static str> {
        let json_parsed_payment = match serde_json::to_string(&payment) {
            Ok(value) => value,
            Err(_) => return Err("Failed to serialize payment JSON to string"),
        };

        self.queue.enqueue(json_parsed_payment).await
    }
}
