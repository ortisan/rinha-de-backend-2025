use crate::application::domain::payment::{GetPaymentsFilter, Payment};
use crate::application::usecases::create_payment::CreatePaymentUsecase;
use crate::application::usecases::get_payments_summary::GetPaymentsSummaryUsecase;
use crate::infrastructure::postgres::PostgresPaymentRepository;
use crate::presentation::data::{GetPaymentsSummaryFilter, PaymentRequest, PaymentsSummaryResponse};
use actix_web::{get, post, web, HttpResponse};
use std::sync::Arc;

#[post("/payments")]
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

#[get("/payments-summary")]
pub async fn get_payments_payments_summary(
    get_payments_summary_usecase: web::Data<
        GetPaymentsSummaryUsecase<Arc<PostgresPaymentRepository>>,
    >,
    filter: web::Query<GetPaymentsSummaryFilter>,
) -> HttpResponse {
    let get_payments_filter = GetPaymentsFilter::from(filter.into_inner());
    let payments_summary_result = get_payments_summary_usecase
        .execute(get_payments_filter)
        .await;

    match payments_summary_result {
        Ok(summary) => {
            let summary_response = PaymentsSummaryResponse::from(summary);
            HttpResponse::Ok().json(summary_response)
        }
        Err(error) => HttpResponse::BadRequest().body(error.to_string()),
    }

}
