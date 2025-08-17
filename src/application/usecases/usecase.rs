use async_trait::async_trait;
use crate::infrastructure;

#[async_trait]
pub trait UseCase<I, O>: Sync + Send + Sized + 'static {
    async fn execute(&self, i: I) -> infrastructure::Result<O>;
}
