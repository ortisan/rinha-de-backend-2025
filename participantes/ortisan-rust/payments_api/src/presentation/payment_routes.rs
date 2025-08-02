use crate::presentation::payment_handlers::{create_payment, get_payments_payments_summary};
use actix_web::web;

pub fn routes(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("")
            .service(create_payment)
            .service(get_payments_payments_summary),
    );
}
