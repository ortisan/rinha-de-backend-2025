use crate::application::domain::payment::Payment;
use crate::application::repositories::payment_repository::PaymentRepository;
use std::fmt::Error;

#[derive(Clone)]
pub struct UsecaseConfig {
    pub payment_processor_default_url: String,
    pub payment_processor_fallback_url: String,
}

#[derive(Clone)]
pub struct CreatePaymentUsecase<T: PaymentRepository> {
    config: UsecaseConfig,
    repository: T,
}

impl<T: PaymentRepository> CreatePaymentUsecase<T> {
    pub fn new(config: UsecaseConfig, repository: T) -> Self {
        Self { config, repository }
    }

    pub async fn execute(&self, payment: Payment) -> Result<Payment, Error> {
        let create_result = self.repository.create(payment).await;
        match create_result {
            Ok(payment) => Ok(payment),
            Err(error) => Err(error),
        }
    }
}
