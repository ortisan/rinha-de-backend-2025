use std::sync::Arc;
use redis::Commands;
use crate::constants::DEFAULT_PAYMENT_PROCESSOR_HEALTH;

pub struct HealthChecker {
    redis_client: Arc<redis::Client>,
    http_client: Arc<reqwest::Client>,
    payment_processor_default_healthcheck_url: String,
    payment_processor_fallback_healthcheck_url: String,
}

impl HealthChecker {
    pub fn new(
        redis_client: Arc<redis::Client>,
        http_client: Arc<reqwest::Client>,
        payment_processor_default_healthcheck_url: String,
        payment_processor_fallback_healthcheck_url: String,
    ) -> Self {
        HealthChecker {
            redis_client,
            http_client,
            payment_processor_default_healthcheck_url,
            payment_processor_fallback_healthcheck_url,
        }
    }

    pub async fn check_health(&self) -> Result<(), Box<dyn std::error::Error>> {


    }

    async fn check_payment_processor_health(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let response = self.http_client.get(&self.payment_processor_default_healthcheck_url).send().await?;
        if response.status() == 200 {
            self.redis_client.set(DEFAULT_PAYMENT_PROCESSOR_HEALTH, 1)
            Ok(())
        } else {
            self.redis_client.set(DEFAULT_PAYMENT_PROCESSOR_HEALTH, 0);
            Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "Payment processor is not healthy")))
        }
    }
}
