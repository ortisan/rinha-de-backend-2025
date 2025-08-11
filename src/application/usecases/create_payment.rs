use crate::application::domain::payment::Payment;
use crate::application::repositories::payment_repository::PaymentRepository;
use crate::application::usecases::usecase::Usecase;
use crate::infrastructure;
use crate::infrastructure::Error;
use crate::presentation::data::PaymentResponse;
use crate::presentation::health_checker::{ActiveServer, HealthChecker};
use log::debug;
use reqwest::Response;
use std::sync::Arc;

#[derive(Clone)]
pub struct UsecaseConfig {
    pub payment_processor_default_url: String,
    pub payment_processor_fallback_url: String,
}

pub struct CreatePaymentUsecase {
    config: UsecaseConfig,
    payment_repository: Box<dyn PaymentRepository>,
    http_client: Arc<reqwest::Client>,
    health_checker: Arc<HealthChecker>,
}

impl CreatePaymentUsecase {
    pub fn new(
        config: UsecaseConfig,
        payment_repository: Box<dyn PaymentRepository>,
        http_client: Arc<reqwest::Client>,
        health_checker: Arc<HealthChecker>,
    ) -> Self {
        CreatePaymentUsecase {
            config,
            payment_repository,
            http_client,
            health_checker,
        }
    }
}

impl Usecase<&Payment, Payment> for CreatePaymentUsecase {
    async fn execute(&self, payment: &Payment) -> infrastructure::Result<Payment> {
        debug!("Creating payment: {:?}", payment);
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
            let payment = self.payment_repository.create(payment).await?;
            Ok(payment)
        } else {
            Err(Error::from("Payment processor error."))?
        }
    }
}
