use actix_web::web;
use crate::presentation::payment_handlers::create_payment;

pub fn routes(config: &mut web::ServiceConfig) {
    config.service(web::scope("/payments")
        .service(create_payment)
    );
}