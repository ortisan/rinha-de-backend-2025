use crate::application::domain::payment::Payment;
use crate::application::usecases::create_payment::CreatePaymentUsecase;
use crate::constants::START_PAYMENT_CHANNEL;
use crate::infrastructure;
use crate::presentation::data::PaymentsSummaryResponse;
use actix_web::HttpResponse;
use log::{debug, error};
use redis::Client;
use std::sync::Arc;

pub struct Worker {
    pub redis_client: Arc<Client>,
    pub create_payment_usecase: CreatePaymentUsecase,
}

impl Worker {
    pub fn new(redis_client: Arc<Client>, create_payment_usecase: CreatePaymentUsecase) -> Self {
        Worker {
            redis_client,
            create_payment_usecase,
        }
    }

    pub async fn listen_for_payments(&mut self) -> infrastructure::Result<()> {
        let mut con = self.redis_client.get_connection()?;
        let mut pubsub = con.as_pubsub();
        pubsub.subscribe(START_PAYMENT_CHANNEL)?;
        loop {
            let msg = pubsub.get_message()?;
            let payload_str: String = msg.get_payload()?;
            let payment: Payment = serde_json::from_str(&payload_str)?;
            let create_payment_result = self.create_payment_usecase.execute(&payment).await;
            match create_payment_result {
                Ok(_) => {
                    debug!("Payment created successfully");
                }
                Err(error) => {
                    error!("Error creating payment: {}", error);
                }
            }
        }
    }
}
