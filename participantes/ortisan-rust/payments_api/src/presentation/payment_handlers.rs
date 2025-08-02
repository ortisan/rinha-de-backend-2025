use crate::application::domain::payment::Payment;
use crate::application::usecases::create_payment::CreatePaymentUsecase;
use crate::infrastructure::postgres::PostgresPaymentRepository;
use crate::presentation::data::PaymentRequest;
use actix_web::{HttpResponse, post, web};
use std::sync::Arc;

#[post("/")]
pub async fn create_payment(
    create_payment_usecase: web::Data<CreatePaymentUsecase<Arc<PostgresPaymentRepository>>>,
    payment_data: web::Json<PaymentRequest>,
) -> HttpResponse {
    let create_payment_result = create_payment_usecase
        .execute(Payment::from(payment_data.into_inner()))
        .await;

    match create_payment_result {
        Ok(payment) => {
            let pay_resp = PaymentRequest::from(payment);
            HttpResponse::Created().json(pay_resp)
        }
        Err(error) => HttpResponse::BadRequest().body(error.to_string()),
    }
}
