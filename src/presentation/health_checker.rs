use crate::constants::{DEFAULT_PAYMENT_PROCESSOR_HEALTH, FALLBACK_PAYMENT_PROCESSOR_HEALTH};
use crate::infrastructure;
use redis::{Commands, TypedCommands};
use std::sync::Arc;

pub struct HealthCheckConfig {
    url: String,
    cache_key_name: String,
    cache_ttl_seconds: u64,
}

pub enum ActiveServer {
    Default,
    Fallback,
    None,
}

pub struct HealthChecker {
    redis_client: Arc<redis::Client>,
    http_client: Arc<reqwest::Client>,
    health_check_config_default: HealthCheckConfig,
    health_check_config_fallback: HealthCheckConfig,
}

impl HealthChecker {
    pub fn new(
        redis_client: Arc<redis::Client>,
        http_client: Arc<reqwest::Client>,
        health_check_config_default: HealthCheckConfig,
        health_check_config_fallback: HealthCheckConfig,
    ) -> Self {
        HealthChecker {
            redis_client,
            http_client,
            health_check_config_default,
            health_check_config_fallback,
        }
    }

    pub async fn get_active_server(&mut self) -> infrastructure::Result<ActiveServer> {
        let mut conn = self.redis_client.get_connection()?;
        let health_default = conn.get(DEFAULT_PAYMENT_PROCESSOR_HEALTH);
        if health_default.is_ok() {
            if health_default.unwrap() == 1 {
                ActiveServer::Default
            }
        }

        let health_fallback = conn.get(FALLBACK_PAYMENT_PROCESSOR_HEALTH);

        if health_fallback.is_ok() {
            if health_fallback.unwrap() == 1 {
                ActiveServer::Fallback
            }
        }

        ActiveServer::None
    }

    pub async fn check_health(&mut self) -> infrastructure::Result<()> {
        self.check_payment_processor_health(&self.health_check_config_default)
            .await;
        self.check_payment_processor_health(&self.health_check_config_fallback)
            .await;

        Ok(())
    }

    async fn check_payment_processor_health(
        &mut self,
        config: &HealthCheckConfig,
    ) -> infrastructure::Result<()> {
        let response = self.http_client.get(&config.url).send().await?;
        let cache_value = if response.status() == 200 { 1 } else { 0 };
        let conn = self.redis_client.get_connection()?;

        conn.set_ex(
            DEFAULT_PAYMENT_PROCESSOR_HEALTH,
            cache_value,
            config.cache_ttl_seconds,
        )
        .await?;

        Ok(())
    }
}
