use crate::application::domain::payment::{Payment, PaymentsSummary};
use crate::application::messaging::consumer::Consumer;
use crate::application::messaging::publisher::Publisher;
use crate::application::repositories::payment_repository::PaymentRepository;
use crate::constants::{ACCEPTED_PAYMENT_CHANNEL, PAYMENTS_KEY};
use crate::infrastructure;
use chrono::{DateTime, Utc};
use log::{debug, error};
use redis::TypedCommands;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::future::Future;
use std::sync::Arc;
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct RedisConfig {
    pub uri: String,
}

pub struct RedisRepository {
    redis_client: redis::Client,
}

impl RedisRepository {
    pub fn new(config: RedisConfig) -> Self {
        RedisRepository {
            redis_client: redis::Client::open(config.uri).unwrap(),
        }
    }

    pub fn set_with_expiration<T>(
        &self,
        key: String,
        value: T,
        ttl: Duration,
    ) -> infrastructure::Result<()>
    where
        T: Serialize + DeserializeOwned + Send + Sync,
    {
        let mut conn = self.redis_client.get_connection()?;
        let value = serde_json::to_string(&value)?;
        let _ = conn.set_ex(&key, value, ttl.as_secs());
        Ok(())
    }
}

#[async_trait::async_trait]
impl PaymentRepository for RedisRepository {
    async fn create(&self, payment: &Payment) -> infrastructure::Result<Payment> {
        debug!("Creating payment: {:?}", payment);
        let timestamp = payment.requested_at.timestamp_nanos_opt().unwrap();
        let payment_json = serde_json::to_string(&payment)?;
        let mut conn = self.redis_client.get_connection()?;
        let _: usize = conn.zadd(PAYMENTS_KEY, payment_json, timestamp)?;
        Ok(payment.clone())
    }

    async fn get_summary(
        &self,
        from: DateTime<Utc>,
        to: DateTime<Utc>,
    ) -> infrastructure::Result<PaymentsSummary> {
        let min_key = from.timestamp_nanos_opt().unwrap();
        let max_key = to.timestamp_nanos_opt().unwrap();
        debug!(
            "Getting payments summary. To: {}, From: {}",
            min_key, max_key
        );

        let mut conn = self.redis_client.get_connection()?;
        let payments: Vec<String> = conn.zrangebyscore(PAYMENTS_KEY, min_key, max_key)?;
        let mut total_payments: u64 = 0;
        let mut total_amount: f64 = 0.0;

        payments.iter().for_each(|x| {
            let payment: Payment = serde_json::from_str(x).unwrap();
            total_payments += 1;
            total_amount += payment.amount;
        });

        Ok(PaymentsSummary {
            total_payments,
            total_amount,
        })
    }
}

#[async_trait::async_trait]
impl Publisher for RedisRepository {
    async fn publish_accepted_payment(&self, payment: &Payment) -> infrastructure::Result<()> {
        let user_json = serde_json::to_string(payment)?;
        let mut conn = self.redis_client.get_connection()?;
        let _: usize = conn.publish(ACCEPTED_PAYMENT_CHANNEL, user_json)?;
        Ok(())
    }
}

#[async_trait::async_trait]
impl Consumer for RedisRepository {
    async fn listen_for_accepted_payment<F, Fut>(&self, mut f: F) -> infrastructure::Result<()>
    where
        F: FnMut(Arc<Payment>) -> Fut + Send + 'static,
        Fut: Future<Output = infrastructure::Result<()>> + Send + 'static,
    {
        let mut conn = self.redis_client.get_connection()?;
        let mut pub_sub = conn.as_pubsub();
        pub_sub.subscribe(ACCEPTED_PAYMENT_CHANNEL)?;
        loop {
            let msg = pub_sub.get_message();
            if msg.is_err() {
                error!(
                    "Error getting message from channel: {:?}",
                    msg.err().unwrap()
                );
                continue;
            }
            let payload_str: String = msg.unwrap().get_payload()?;
            let payment: Payment = serde_json::from_str(&payload_str)?;
            let dispatched_result = f(Arc::new(payment)).await;
            if dispatched_result.is_err() {
                error!(
                    "Error dispatching payment: {:?}",
                    dispatched_result.err().unwrap()
                );
            }
        }
    }
}
