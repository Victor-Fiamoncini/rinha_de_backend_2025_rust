use std::time::Duration;

use reqwest::{Client, ClientBuilder};
use tracing::{error, info};

use crate::{
    config::Config,
    dto::{PaymentDTO, PaymentProcessor},
};

#[derive(Clone)]
pub struct CreateExternalPaymentService {
    config: Config,
    http_client: Client,
}

impl CreateExternalPaymentService {
    pub fn new(config: Config) -> Self {
        let http_client = ClientBuilder::new()
            .connect_timeout(Duration::from_millis(300))
            .timeout(Duration::from_millis(300))
            .build()
            .expect("Failed to create HTTP Client");

        CreateExternalPaymentService {
            config,
            http_client,
        }
    }

    pub async fn execute(
        &self,
        payment_processor: PaymentProcessor,
        payment: PaymentDTO,
    ) -> Result<(), &'static str> {
        let url = match payment_processor {
            PaymentProcessor::Default => {
                format!("{}/payments", self.config.payment_processor_default_url)
            }
            PaymentProcessor::Fallback => {
                format!("{}/payments", self.config.payment_processor_fallback_url)
            }
        };

        let response = self
            .http_client
            .post(&url)
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .json(&payment)
            .send()
            .await;

        match response {
            Ok(response) => {
                if response.status().is_success() {
                    info!("Successfully created external payment");

                    return Ok(());
                }

                error!("Failed to create external payment");

                Err("Failed to create external payment")
            }
            Err(_) => {
                error!("Failed to send external payment request");

                Err("Failed to send external payment request")
            }
        }
    }
}
