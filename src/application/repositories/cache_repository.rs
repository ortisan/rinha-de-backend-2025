use crate::infrastructure;
use async_trait::async_trait;
use serde::Serialize;
use serde::de::DeserializeOwned;
use std::time::Duration;

#[async_trait]
pub trait CacheRepository: Send + Sync {
    async fn set_with_expiration<T>(
        &self,
        key: String,
        value: T,
        ttl: Duration,
    ) -> infrastructure::Result<()>
    where
        T: Serialize + Send + Sync + Sized + 'static;

    async fn get<T>(&self, key: String) -> infrastructure::Result<Option<T>>
    where
        T: DeserializeOwned + Send + Sync + Sized + 'static;
}
