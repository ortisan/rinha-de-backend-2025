use crate::application::domain::payment::{GetPaymentsFilter, PaymentsSummary};
use crate::application::repositories::payment_repository::PaymentRepository;

use std::fmt::Error;

#[derive(Clone)]
pub struct GetPaymentsSummaryUsecase<T: PaymentRepository> {
    repository: T,
}

impl<T: PaymentRepository> GetPaymentsSummaryUsecase<T> {
    pub fn new(repository: T) -> Self {
        Self { repository }
    }
    pub async fn execute(
        &self,
        get_payments_filter: GetPaymentsFilter,
    ) -> Result<PaymentsSummary, Error> {
        let summary_result = self
            .repository
            .get_summary(
                get_payments_filter.from,
                get_payments_filter.to,
            )
            .await;

        match summary_result {
            Ok(summary) => Ok(summary),
            Err(e) => Err(e),
        }
    }
}
