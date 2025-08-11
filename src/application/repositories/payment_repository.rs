use crate::application::domain::payment::{Payment, PaymentsSummary};
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use crate::infrastructure;

#[async_trait]
pub trait PaymentRepository {
    async fn create(&self, payment: &Payment) -> infrastructure::Result<Payment>;
    async fn get_summary(
        &self,
        from: DateTime<Utc>,
        to: DateTime<Utc>,
    ) -> infrastructure::Result<PaymentsSummary>;
}
