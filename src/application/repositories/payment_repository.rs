use crate::application::domain::payment::{Payment, PaymentsSummary};
use async_trait::async_trait;
use chrono::{DateTime, FixedOffset, Utc};
use std::fmt::Error;

#[async_trait]
pub trait PaymentRepository {
    async fn create(&self, payment: Payment) -> Result<Payment, Error>;
    async fn get_summary(
        &self,
        from: DateTime<Utc>,
        to: DateTime<Utc>,
    ) -> Result<PaymentsSummary, Error>;
}
