use crate::application::usecases::create_payment::CreatePaymentUsecase;
use crate::application::usecases::get_payments_summary::GetPaymentsSummaryUsecase;
use crate::infrastructure::postgres::{DbConfig, PostgresPaymentRepository};
use crate::presentation::payment_routes;
use actix_web::middleware::Logger;
use actix_web::{App, HttpServer, web};
use std::sync::Arc;
use env_logger;

mod application;
mod infrastructure;
mod presentation;

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let database_result = dotenv::var("DATABASE_URL");
    match database_result {
        Ok(_) => {}
        Err(_) => panic!("DATABASE_URL not found"),
    };

    let db_config = DbConfig {
        database_url: database_result.unwrap(),
    };

    let payment_repository = Arc::new(PostgresPaymentRepository::new(db_config));
    let create_payment_usecase = CreatePaymentUsecase::new(payment_repository.clone());
    let app_data_create_payment_usecase = web::Data::new(create_payment_usecase);

    let get_summary_usecase = GetPaymentsSummaryUsecase::new(payment_repository);
    let app_data_get_summary_usecase = web::Data::new(get_summary_usecase);

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("debug"));

    let result = HttpServer::new(move || {
        App::new()
            .app_data(app_data_create_payment_usecase.clone())
            .app_data(app_data_get_summary_usecase.clone())
            .wrap(Logger::default())
            .configure(payment_routes::routes)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await?;

    Ok(result)
}
