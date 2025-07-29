use std::fmt::Error;
use crate::application::domain::payment::Payment;
use crate::application::repositories::payment_repository::PaymentRepository;

#[derive(Clone)]
pub struct CreatePaymentUsecase<T: PaymentRepository> {
    repository: T
}

impl <T: PaymentRepository> CreatePaymentUsecase<T> {
    pub fn new(repository: T) -> Self {
        Self {
            repository
        }
    }

    pub async fn execute(&self, payment: Payment) -> Result<Payment, Error> {
        let create_result = self.repository.create(payment).await;
        match create_result {
            Ok(payment) => Ok(payment),
            Err(error) => Err(error)
        }
    }
}