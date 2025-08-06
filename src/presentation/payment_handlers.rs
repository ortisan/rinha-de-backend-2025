use crate::application::domain::payment::{GetPaymentsFilter, Payment};
use crate::application::usecases::accept_payment::AcceptPaymentUsecase;
use crate::application::usecases::get_payments_summary::GetPaymentsSummaryUsecase;
use crate::presentation::data::{
    GetPaymentsSummaryFilter, PaymentRequest, PaymentResponse, PaymentsSummaryResponse,
};
use actix_web::{get, post, web, HttpResponse};

#[post("/payments")]
pub async fn create_payment(
    accept_payment_usecase: web::Data<AcceptPaymentUsecase>,
    payment_data: web::Json<PaymentRequest>,
) -> HttpResponse {
    let payment = &Payment::from(payment_data.into_inner());

    let acccept_payment_result = accept_payment_usecase.execute(payment).await;

    match acccept_payment_result {
        Ok(payment) => {
            let payment_response = PaymentResponse::from(payment);
            HttpResponse::Accepted().json(payment_response)
        }
        Err(error) => HttpResponse::BadRequest().body(error.to_string()),
    }
}

#[get("/payments-summary")]
pub async fn get_payments_payments_summary(
    get_payments_summary_usecase: web::Data<GetPaymentsSummaryUsecase>,
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
