use actix_web::{web, App, HttpServer};
use actix_web::middleware::Logger;
use std::sync::Arc;
use crate::application::usecases::create_payment::CreatePaymentUsecase;
use crate::infrastructure::postgres::{DbConfig, PostgresPaymentRepository};
use crate::presentation::payment_routes;

mod presentation;
mod application;
mod infrastructure;

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db_config = DbConfig {
        database_url: String::from("localhost:5432"),
    };

    let payment_repository = Arc::new(PostgresPaymentRepository::new(db_config));
    let create_payment_usecase = CreatePaymentUsecase::new(payment_repository);
    let app_data_usecase = web::Data::new(create_payment_usecase);

    let result = HttpServer::new(move || {
        App::new()
            .app_data(app_data_usecase.clone())
            .wrap(Logger::default())
            .configure(payment_routes::routes)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await?;

    Ok(result)
}
