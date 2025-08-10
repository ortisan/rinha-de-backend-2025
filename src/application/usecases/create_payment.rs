use crate::application::domain::payment::Payment;
use crate::constants::PAYMENTS_KEY;
use crate::infrastructure;
use crate::infrastructure::Error;
use crate::presentation::data::PaymentResponse;
use crate::presentation::health_checker::{ActiveServer, HealthChecker};
use log::debug;
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
    redis_client: Arc<Client>,
    http_client: Arc<reqwest::Client>,
    health_checker: Arc<HealthChecker>,
}

impl CreatePaymentUsecase {
    pub fn new(
        config: UsecaseConfig,
        redis_client: Arc<Client>,
        http_client: Arc<reqwest::Client>,
        health_checker: Arc<HealthChecker>,
    ) -> Self {
        CreatePaymentUsecase {
            config,
            redis_client,
            http_client,
            health_checker,
        }
    }

    pub async fn execute<'a>(&self, payment: &'a Payment) -> infrastructure::Result<&'a Payment> {
        let active_server: ActiveServer = self.health_checker.get_active_server().await?;
        debug!("Active server: {:?}", &active_server);
        let url_payment_processor: &String = match active_server {
            ActiveServer::Default => {
                debug!("Using default payment processor.");
                Ok(&self.config.payment_processor_default_url)
            }
            ActiveServer::Fallback => {
                debug!("Using fallback payment processor.");
                Ok(&self.config.payment_processor_fallback_url)
            }
            ActiveServer::None => {
                debug!("Payment server is not available.");
                Err(Error::from("Payment server is not available."))
            }
        }?;
        let payment_request = PaymentResponse::from(payment);
        let response: Response = self
            .http_client
            .post(format!("{}/payments", url_payment_processor))
            .json(&payment_request)
            .send()
            .await?;
        debug!("Payment processor response: {:?}", response);

        if response.status() == 200 {
            let timestamp = payment.requested_at.timestamp_nanos_opt().unwrap();
            let payment_json = serde_json::to_string(&payment)?;
            let mut conn = self.redis_client.get_connection()?;
            let _: bool = conn.zadd(PAYMENTS_KEY, payment_json, timestamp)?;
        } else {
            Err(Error::from("Payment processor error."))?
        }
        Ok(payment)
    }
}
