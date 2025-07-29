use std::sync::Arc;
use crate::application::domain::payment::Payment;
use crate::application::usecases::create_payment::CreatePaymentUsecase;
use crate::infrastructure::postgres::PostgresPaymentRepository;
use actix_web::{HttpResponse, post, web};

#[post("/")]
pub async fn create_payment(
    create_payment_usecase: web::Data<CreatePaymentUsecase<Arc<PostgresPaymentRepository>>>,
    payment_data: web::Json<Payment>,
) -> HttpResponse {
    HttpResponse::Ok().finish()
}
