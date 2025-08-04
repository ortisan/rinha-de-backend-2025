use crate::application::domain::payment::{GetPaymentsFilter, PaymentsSummary};
use crate::application::repositories::payment_repository::PaymentRepository;

use crate::infrastructure;

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
    ) -> infrastructure::Result<PaymentsSummary> {
        let summary = self
            .repository
            .get_summary(get_payments_filter.from, get_payments_filter.to)
            .await?;

        Ok(summary)
    }
}
