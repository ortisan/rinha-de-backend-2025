use crate::application::http::http::HttpMethod;
use crate::application::usecases::accept_payment::AcceptPaymentUsecase;
use crate::application::usecases::create_payment::{CreatePaymentUseCase, UseCaseConfig};
use crate::application::usecases::get_payments_summary::GetPaymentsSummaryUsecase;
use crate::constants::{DEFAULT_PAYMENT_PROCESSOR_HEALTH, FALLBACK_PAYMENT_PROCESSOR_HEALTH};
use crate::infrastructure::health_checker::{HealthCheckConfig, HealthChecker};
use crate::infrastructure::redis::{RedisConfig, RedisRepository};
use crate::infrastructure::reqwest::HttpReqwest;
use crate::presentation::payment_routes;
use crate::presentation::worker::Worker;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use env_logger;
use log::{debug, error};
use redis::Client;
use reqwest;
use std::sync::Arc;
use std::time::Duration;
use tokio;

mod application;
mod constants;
mod infrastructure;
mod presentation;

#[tokio::main]
async fn main() -> infrastructure::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("debug"));

    let redis_uri = dotenv::var("REDIS_URI").expect("REDIS_URI not found");
    let redis_config = RedisConfig { uri: redis_uri };
    let redis_repository = Arc::new(RedisRepository::new(redis_config));

    let http_client = Arc::new(reqwest::Client::new());
    let http_requestor = Arc::new(HttpReqwest::new(http_client.clone()));

    let accept_payment_usecase = AcceptPaymentUsecase::new(redis_repository.clone());
    let app_data_accept_payment_usecase = web::Data::new(accept_payment_usecase);

    let payment_processor_default_url = dotenv::var("PAYMENT_PROCESSOR_DEFAULT_URL")
        .expect("PAYMENT_PROCESSOR_DEFAULT_URL not found");
    let payment_processor_fallback_url = dotenv::var("PAYMENT_PROCESSOR_FALLBACK_URL")
        .expect("PAYMENT_PROCESSOR_FALLBACK_URL not found");
    let create_payment_config = UseCaseConfig {
        payment_processor_default_url,
        payment_processor_fallback_url,
    };

    let get_summary_usecase = GetPaymentsSummaryUsecase::new(redis_repository.clone());
    let app_data_get_summary_usecase = web::Data::new(get_summary_usecase);

    let health_config_default_url = dotenv::var("HEALTH_CHECKER_DEFAULT_URL")?;
    let health_config_fallback_url = dotenv::var("HEALTH_CHECKER_FALBACK_URL")?;
    let health_check_default_config = HealthCheckConfig {
        url: health_config_default_url,
        method: HttpMethod::GET,
        cache_key_name: String::from(DEFAULT_PAYMENT_PROCESSOR_HEALTH),
        cache_ttl_seconds: 5,
    };
    let health_check_fallback_config = HealthCheckConfig {
        url: health_config_fallback_url,
        method: HttpMethod::GET,
        cache_key_name: String::from(FALLBACK_PAYMENT_PROCESSOR_HEALTH),
        cache_ttl_seconds: 5,
    };

    let health_checker = Arc::new(HealthChecker::new(
        redis_repository.clone(),
        http_requestor.clone(),
        health_check_default_config.clone(),
        health_check_fallback_config.clone(),
    ));

    {
        let health_checker_clone = health_checker.clone();
        tokio::spawn(async move {
            loop {
                debug!("Waiting for the next tick of the interval");
                tokio::time::sleep(Duration::from_millis(4000)).await;
                debug!("Health check started");
                let health_result = &health_checker_clone.check_health().await;
                match health_result {
                    Ok(_) => debug!("Health check finished"),
                    Err(e) => error!("Health check error: {:?}", e),
                }
            }
        });
    }

    {
        let create_payment_usecase = Arc::new(CreatePaymentUseCase::new(
            create_payment_config,
            redis_repository.clone(),
            http_requestor.clone(),
            health_checker.clone(),
        ));

        tokio::spawn(async move {
            let worker = Worker::new(redis_repository.clone(), create_payment_usecase);
            match worker.listen_for_payments().await {
                Ok(_) => debug!("Worker finished"),
                Err(e) => error!("Worker error: {:?}", e),
            }
        });
    }

    HttpServer::new(move || {
        App::new()
            .app_data(app_data_accept_payment_usecase.clone())
            .app_data(app_data_get_summary_usecase.clone())
            .wrap(Logger::default())
            .configure(payment_routes::routes)
    })
    .bind("0.0.0.0:8000")?
    .run()
    .await?;

    Ok(())
}
