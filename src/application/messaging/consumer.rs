use std::sync::Arc;
use crate::application::domain::payment::Payment;
use crate::infrastructure;
use async_trait::async_trait;

#[async_trait]
pub trait Consumer: Send + Sync {
    async fn listen_for_accepted_payment<F, Fut>(&self, f: F) -> infrastructure::Result<()>
    where
        F: Fn(Arc<Payment>) -> Fut + Send + 'static,
        Fut: Future<Output = infrastructure::Result<()>>+ Send + 'static;
}
