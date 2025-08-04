use crate::application::usecases::accept_payment::AcceptPaymentUsecase;
use crate::application::usecases::create_payment::{CreatePaymentUsecase, UsecaseConfig};
use crate::application::usecases::get_payments_summary::GetPaymentsSummaryUsecase;
use crate::infrastructure::postgres::{DbConfig, PostgresPaymentRepository};
use crate::infrastructure::redis::RedisConfig;
use crate::presentation::payment_routes;
use crate::presentation::worker::Worker;
use actix_web::middleware::Logger;
use actix_web::{App, HttpServer, web};
use env_logger;
use redis::Client;
use reqwest;
use std::sync::Arc;
use tokio;

mod application;
mod constants;
mod infrastructure;
mod presentation;

#[actix_web::main]
async fn main() -> infrastructure::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("debug"));
    let database_uri = dotenv::var("DATABASE_URI").expect("DATABASE_URI not found");
    let db_config = DbConfig {
        database_url: database_uri,
    };
    let redis_uri = dotenv::var("REDIS_URI").expect("REDIS_URI not found");
    let redis_config = RedisConfig { uri: redis_uri };
    let redis_client = Arc::new(Client::open(redis_config.uri)?);
    let accept_payment_usecase = AcceptPaymentUsecase::new(redis_client.clone());
    let app_data_accept_payment_usecase = web::Data::new(accept_payment_usecase);

    let payment_processor_default_url = dotenv::var("PAYMENT_PROCESSOR_DEFAULT_URL")
        .expect("PAYMENT_PROCESSOR_DEFAULT_URL not found");
    let payment_processor_fallback_url = dotenv::var("PAYMENT_PROCESSOR_FALLBACK_URL")
        .expect("PAYMENT_PROCESSOR_FALLBACK_URL not found");
    let create_payment_config = UsecaseConfig {
        payment_processor_default_url,
        payment_processor_fallback_url,
    };

    let payment_repository = Arc::new(PostgresPaymentRepository::new(db_config));
    let get_summary_usecase = GetPaymentsSummaryUsecase::new(payment_repository.clone());
    let app_data_get_summary_usecase = web::Data::new(get_summary_usecase);

    let http_client = Arc::new(reqwest::Client::new());
    let create_payment_usecase =
        CreatePaymentUsecase::new(create_payment_config, redis_client.clone(), http_client);

    tokio::spawn(async move {
        let mut worker = Worker::new(redis_client, create_payment_usecase);
        worker
            .listen_for_payments()
            .await
            .expect("Error to listen for payments");
    });

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
