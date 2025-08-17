use crate::application::domain::payment::Payment;
use crate::application::http::http::{HttpMethod, HttpRequest, HttpRequester, HttpResponse};
use crate::application::repositories::payment_repository::PaymentRepository;
use crate::application::usecases::usecase::UseCase;
use crate::infrastructure;
use crate::infrastructure::Error;
use crate::infrastructure::health_checker::{ActiveServer, HealthCheckResponse, HealthChecker};
use crate::infrastructure::reqwest::HttpReqwest;
use crate::presentation::data::PaymentResponse;
use async_trait::async_trait;
use log::debug;
use std::sync::Arc;
use std::time::Duration;

#[derive(Clone)]
pub struct UseCaseConfig {
    pub payment_processor_default_url: String,
    pub payment_processor_fallback_url: String,
}

pub struct CreatePaymentUseCase {
    config: UseCaseConfig,
    payment_repository: Arc<dyn PaymentRepository>,
    http_requester: Arc<HttpReqwest>,
    health_checker: Arc<HealthChecker>,
}

impl CreatePaymentUseCase {
    pub fn new(
        config: UseCaseConfig,
        payment_repository: Arc<dyn PaymentRepository>,
        http_requester: Arc<HttpReqwest>,
        health_checker: Arc<HealthChecker>,
    ) -> Self {
        CreatePaymentUseCase {
            config,
            payment_repository,
            http_requester,
            health_checker,
        }
    }
}

#[async_trait]
impl UseCase<Payment, Payment> for CreatePaymentUseCase {
    async fn execute(&self, payment: Payment) -> infrastructure::Result<Payment> {
        debug!("Creating payment: {:?}", payment);

        let active_server = self.health_checker.get_active_server().await?;

        let url_payment_processor: String;

        match active_server {
            ActiveServer::Default => {
                debug!("Using default payment processor.");
                url_payment_processor = self.config.payment_processor_default_url.clone();
            }
            ActiveServer::Fallback => {
                debug!("Using fallback payment processor.");
                url_payment_processor = self.config.payment_processor_fallback_url.clone();
            }
            ActiveServer::Unavailable => {
                debug!("Payment server is not available.");
                return Err(Error::from("Payment processor server is not available."));
            }
        }

        let body: Option<PaymentResponse> = Some(PaymentResponse::from(&payment));

        let request: HttpRequest<PaymentResponse> = HttpRequest::new(
            HttpMethod::POST,
            format!("{}/payments", url_payment_processor),
            None,
            None,
            body,
            Some(Duration::from_secs(10)),
        );

        let response: HttpResponse<()> = self.http_requester.make_request(request).await?;

        if response.status.is_success() {
            let payment = self.payment_repository.create(&payment).await?;
            Ok(payment)
        } else {
            Err(Error::from("Payment processor error."))?
        }
        // debug!("Creating payment: {:?}", payment);
        // let active_server: ActiveServer = self.health_checker.get_active_server().await?;
        // debug!("Active server: {:?}", &active_server);
        // let url_payment_processor: &String = match active_server {
        //     ActiveServer::Default => {
        //         debug!("Using default payment processor.");
        //         Ok(&self.config.payment_processor_default_url)
        //     }
        //     ActiveServer::Fallback => {
        //         debug!("Using fallback payment processor.");
        //         Ok(&self.config.payment_processor_fallback_url)
        //     }
        //     ActiveServer::None => {
        //         debug!("Payment server is not available.");
        //         Err(Error::from("Payment server is not available."))
        //     }
        // }?;
        // let payment_request = PaymentResponse::from(payment);
        // let response: Response = self
        //     .http_client
        //     .post(format!("{}/payments", url_payment_processor))
        //     .json(&payment_request)
        //     .send()
        //     .await?;
        // debug!("Payment processor response: {:?}", response);
        //
        // if response.status() == 200 {
        //     let payment = self.payment_repository.create(&payment).await?;
        //     Ok(payment)
        // } else {
        //     Err(Error::from("Payment processor error."))?
        // }
    }
}
