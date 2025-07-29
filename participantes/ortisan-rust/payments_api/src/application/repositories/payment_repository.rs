use std::fmt::Error;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use crate::application::domain::payment::{Payment, PaymentsSummary};

#[async_trait]
pub trait PaymentRepository {
    async fn create(&self, payment: Payment) -> Result<Payment, Error>;
    async fn summary(&self, from: DateTime<Utc>, to: DateTime<Utc>) -> Result<PaymentsSummary, Error>;
}