use crate::application::http::http::{HttpMethod, HttpRequest, HttpRequester, HttpResponse};
use crate::application::repositories::cache_repository::CacheRepository;
use crate::infrastructure;
use crate::infrastructure::redis::RedisRepository;
use crate::infrastructure::reqwest::HttpReqwest;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::Duration;

const TTL_MILLI: u64 = 5000;

#[derive(Clone, Debug, Deserialize)]
pub struct HealthCheckConfig {
    pub url: String,
    pub method: HttpMethod,
    pub cache_key_name: String,
    pub cache_ttl_seconds: u64,
}
#[derive(Clone, Debug)]
pub enum ActiveServer {
    Default,
    Fallback,
    Unavailable,
}
//
pub struct HealthChecker {
    cache_repository: Arc<RedisRepository>,
    http_requester: Arc<HttpReqwest>,
    health_check_config_default: HealthCheckConfig,
    health_check_config_fallback: HealthCheckConfig,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct HealthCheckResponse {
    pub failing: bool,
    #[serde(rename = "minResponseTime")]
    pub min_response_time: u64,
}

impl HealthChecker {
    pub fn new(
        cache_repository: Arc<RedisRepository>,
        http_requester: Arc<HttpReqwest>,
        health_check_config_default: HealthCheckConfig,
        health_check_config_fallback: HealthCheckConfig,
    ) -> Self {
        HealthChecker {
            cache_repository,
            http_requester,
            health_check_config_default,
            health_check_config_fallback,
        }
    }

    pub async fn get_active_server(&self) -> infrastructure::Result<ActiveServer> {
        let default_health_check: Option<HealthCheckResponse> = self
            .cache_repository
            .get(self.health_check_config_default.cache_key_name.clone())
            .await?;
        let fallback_health_check: Option<HealthCheckResponse> = self
            .cache_repository
            .get(self.health_check_config_fallback.cache_key_name.clone())
            .await?;

        if default_health_check.is_some() && fallback_health_check.is_some() {
            let default_health_check = default_health_check.unwrap();
            let fallback_health_check = fallback_health_check.unwrap();
            if (default_health_check.failing && fallback_health_check.failing) {
                return Ok(ActiveServer::Unavailable);
            } else if (default_health_check.failing) {
                return Ok(ActiveServer::Fallback);
            } else if (fallback_health_check.failing) {
                return Ok(ActiveServer::Default);
            } else {
                return Ok(ActiveServer::Default);
            }
        }
        Ok(ActiveServer::Unavailable)
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
        // TODO change to fluent builder
        let request: HttpRequest<()> = HttpRequest::new(
            HttpMethod::GET,
            config.url.clone(),
            None,
            None,
            None,
            Some(Duration::from_secs(10)),
        );
        let response: HttpResponse<HealthCheckResponse> =
            self.http_requester.make_request(request).await?;

        self.cache_repository
            .set_with_expiration(
                config.cache_key_name.clone(),
                response.body,
                Duration::from_secs(config.cache_ttl_seconds),
            )
            .await?;
        Ok(())
    }
}
