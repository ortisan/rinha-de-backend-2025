use crate::application::domain::payment::Payment;
use crate::infrastructure;
use chrono::Timelike;
use redis::{Client, Commands};
use reqwest::Response;
use std::sync::Arc;

#[derive(Clone)]
pub struct UsecaseConfig {
    pub payment_processor_default_url: String,
    pub payment_processor_fallback_url: String,
}

#[derive(Clone)]
pub struct CreatePaymentUsecase {
    config: UsecaseConfig,
    redis_client: Arc<redis::Client>,
    http_client: Arc<reqwest::Client>,
}

impl CreatePaymentUsecase {
    pub fn new(
        config: UsecaseConfig,
        redis_client: Arc<Client>,
        http_client: Arc<reqwest::Client>,
    ) -> Self {
        CreatePaymentUsecase {
            config,
            redis_client,
            http_client,
        }
    }

    pub async fn execute(&mut self, payment: Payment) -> infrastructure::Result<Payment> {
        let mut response: Response;
        response = self
            .http_client
            .post(format!(
                "{}/payments",
                &self.config.payment_processor_default_url
            ))
            .json(&payment)
            .send()
            .await?;

        if response.status() != 200 {
            response = self
                .http_client
                .post(format!(
                    "{}/payments",
                    &self.config.payment_processor_fallback_url
                ))
                .json(&payment)
                .send()
                .await?;
        }

        if response.status() == 200 {
            let payment_json = serde_json::to_string(&payment)?;
            let mut conn = self.redis_client.get_connection()?;
            let result_zadd: bool = conn.zadd(
                "payments_processed",
                payment_json,
                payment.requested_at.nanosecond(),
            )?;
        }

        Ok(payment)
    }
}
