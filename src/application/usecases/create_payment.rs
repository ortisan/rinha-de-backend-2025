use crate::application::domain::payment::Payment;
use crate::infrastructure;
use crate::presentation::data::PaymentResponse;
use chrono::Timelike;
use redis::{Client, Commands};
use reqwest::Response;
use std::sync::Arc;
use crate::constants::PAYMENTS_KEY;

#[derive(Clone)]
pub struct UsecaseConfig {
    pub payment_processor_default_url: String,
    pub payment_processor_fallback_url: String,
}

#[derive(Clone)]
pub struct CreatePaymentUsecase {
    config: UsecaseConfig,
    redis_client: Arc<Client>,
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

    pub async fn execute<'a>(
        &mut self,
        payment: &'a Payment,
    ) -> infrastructure::Result<&'a Payment> {
        let payment_request = PaymentResponse::from(payment);

        let mut response: Response;
        response = self
            .http_client
            .post(format!(
                "{}/payments",
                &self.config.payment_processor_default_url
            ))
            .json(&payment_request)
            .send()
            .await?;

        if response.status() != 200 {
            response = self
                .http_client
                .post(format!(
                    "{}/payments",
                    &self.config.payment_processor_fallback_url
                ))
                .json(&payment_request)
                .send()
                .await?;
        }

        if response.status() == 200 {
            let timestamp = payment.requested_at.timestamp_nanos_opt().unwrap();
            let payment_json = serde_json::to_string(&payment)?;
            let mut conn = self.redis_client.get_connection()?;
            let result_zadd: bool = conn.zadd(
                PAYMENTS_KEY,
                payment_json,
                timestamp,
            )?;
        }

        Ok(payment)
    }
}
