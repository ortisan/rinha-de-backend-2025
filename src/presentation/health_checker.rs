use crate::infrastructure;
use log::debug;
use redis::{Commands, RedisError};
use serde::Deserialize;
use std::sync::Arc;
use std::time::Duration;

#[derive(Clone, Debug, Deserialize)]
pub struct HealthCheckConfig {
    pub url: String,
    pub cache_key_name: String,
    pub cache_ttl_seconds: u64,
}

#[derive(Clone, Debug)]
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

    pub async fn get_active_server(&self) -> infrastructure::Result<ActiveServer> {
        let mut conn = self.redis_client.get_connection()?;
        let health_default: Result<i32, RedisError> =
            Commands::get(&mut conn, &self.health_check_config_default.cache_key_name);
        if health_default.is_ok() {
            if health_default.unwrap() == 1 {
                return Ok(ActiveServer::Default);
            }
        }

        let health_fallback: Result<i32, RedisError> =
            Commands::get(&mut conn, &self.health_check_config_fallback.cache_key_name);
        if health_fallback.is_ok() {
            if health_fallback.unwrap() == 1 {
                return Ok(ActiveServer::Fallback);
            }
        }

        Ok(ActiveServer::None)
    }

    pub async fn check_health(&self) -> infrastructure::Result<()> {
        self.check_payment_processor_health(&self.health_check_config_default)
            .await?;
        self.check_payment_processor_health(&self.health_check_config_fallback)
            .await?;

        Ok(())
    }

    async fn check_payment_processor_health(
        &self,
        config: &HealthCheckConfig,
    ) -> infrastructure::Result<()> {
        let response = self
            .http_client
            .get(&config.url)
            .timeout(Duration::from_millis(5000))
            .send()
            .await?;
        debug!("Payment processor health check response: {:?}", response);
        let cache_value = if response.status() == 200 { 1 } else { 0 };

        let mut conn = self.redis_client.get_connection()?;
        let _: () = conn.set_ex(
            &config.cache_key_name,
            cache_value,
            config.cache_ttl_seconds,
        )?;

        Ok(())
    }
}
