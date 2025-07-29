use std::fmt::Error;
use crate::application::domain::payment::PaymentsSummary;
use crate::application::repositories::payment_repository::PaymentRepository;

#[derive(Clone)]
pub struct GetPaymentsSummary<T: PaymentRepository> {
    repository: T
}

impl <T:PaymentRepository> GetPaymentsSummary<T> {
    async fn execute() -> Result<PaymentsSummary, Error>{
        Ok(PaymentsSummary{
            total_payments: 0,
            total_amount: 0.0,
        }) // TODO
    }

}