use crate::{queue::Queue, types::CreatePaymentDTO};

#[derive(Clone)]
pub struct CreatePaymentService {
    queue: Queue,
}

impl CreatePaymentService {
    pub fn new(queue: Queue) -> Self {
        CreatePaymentService { queue }
    }

    pub async fn create_payment(&self, payment: CreatePaymentDTO) -> Result<(), &'static str> {
        let json_parsed_payment = match serde_json::to_string(&payment) {
            Ok(value) => value,
            Err(_) => return Err("Failed to serialize payment JSON to string"),
        };

        self.queue.enqueue(json_parsed_payment).await
    }
}
