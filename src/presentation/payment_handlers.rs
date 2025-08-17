use crate::application::domain::payment::{GetPaymentsFilter, Payment};
use crate::application::usecases::accept_payment::AcceptPaymentUsecase;
use crate::application::usecases::get_payments_summary::GetPaymentsSummaryUsecase;
use crate::application::usecases::usecase::UseCase;
use crate::presentation::data::{
    GetPaymentsSummaryFilter, PaymentRequest, PaymentResponse, PaymentsSummaryResponse,
};
use actix_web::{HttpResponse, get, post, web};

#[post("/payments{tail:/*}")]
pub async fn create_payment(
    usecase: web::Data<AcceptPaymentUsecase>,
    payment_data: web::Json<PaymentRequest>,
) -> HttpResponse {
    let payment = &Payment::from(payment_data.into_inner());
    let acccept_payment_result = usecase.execute(payment.clone()).await;
    match acccept_payment_result {
        Ok(payment) => {
            let payment_response = PaymentResponse::from(&payment);
            HttpResponse::Accepted().json(payment_response)
        }
        Err(error) => HttpResponse::BadRequest().body(error.to_string()),
    }
}

#[get("/payments-summary{tail:/*}")]
pub async fn get_payments_payments_summary(
    usecase: web::Data<GetPaymentsSummaryUsecase>,
    filter: web::Query<GetPaymentsSummaryFilter>,
) -> HttpResponse {
    let get_payments_filter = GetPaymentsFilter::from(filter.into_inner());
    let payments_summary_result = usecase.execute(get_payments_filter).await;

    match payments_summary_result {
        Ok(summary) => {
            let summary_response = PaymentsSummaryResponse::from(summary);
            HttpResponse::Ok().json(summary_response)
        }
        Err(error) => HttpResponse::BadRequest().body(error.to_string()),
    }
}
