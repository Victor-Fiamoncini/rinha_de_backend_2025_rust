use std::time::Duration;

use reqwest::{Client, ClientBuilder};

use crate::{config::Config, dtos::CreateExternalPaymentDTO};

pub enum PaymentProcessors {
    Default,
    Fallback,
}

#[derive(Clone)]
pub struct CreateExternalPaymentService {
    config: Config,
    http_client: Client,
}

impl CreateExternalPaymentService {
    pub fn new(config: Config) -> Self {
        let http_client = ClientBuilder::new()
            .pool_max_idle_per_host(100)
            .pool_idle_timeout(Duration::from_secs(30))
            .timeout(Duration::from_millis(10000))
            .connect_timeout(Duration::from_millis(1000))
            .tcp_keepalive(Duration::from_secs(30))
            .tcp_nodelay(true)
            .build()
            .expect("Failed to create HTTP Client");

        CreateExternalPaymentService {
            config,
            http_client,
        }
    }

    pub async fn create_external_payment(
        &self,
        payment_processor: PaymentProcessors,
        payment: CreateExternalPaymentDTO,
    ) -> Result<(), &'static str> {
        let url = match payment_processor {
            PaymentProcessors::Default => {
                format!("{}/payments", self.config.payment_processor_default_url)
            }
            PaymentProcessors::Fallback => {
                format!("{}/payments", self.config.payment_processor_fallback_url)
            }
        };

        println!("Sending payment to {}: {:?}", url, payment);

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
                    return Ok(());
                }

                Err("Failed to create external payment")
            }
            Err(_) => Err("Failed to send external payment request"),
        }
    }
}
