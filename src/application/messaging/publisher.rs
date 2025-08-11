use async_trait::async_trait;
use crate::application::domain::payment::Payment;
use crate::infrastructure;

#[async_trait]
pub trait Publisher {
    async fn publish_accepted_payment(&self, payment: &Payment) -> infrastructure::Result<()>;
}