use crate::application::domain::payment::Payment;
use crate::application::usecases::create_payment::CreatePaymentUsecase;
use crate::constants::START_PAYMENT_CHANNEL;
use crate::infrastructure;
use crate::infrastructure::Error;
use crate::presentation::data::PaymentsSummaryResponse;
use actix_web::HttpResponse;
use log::{debug, error};
use redis::Client;
use std::sync::Arc;

pub struct Worker {
    pub redis_client: Arc<Client>,
    pub create_payment_usecase: Arc<CreatePaymentUsecase>,
}

impl Worker {
    pub fn new(
        redis_client: Arc<Client>,
        create_payment_usecase: Arc<CreatePaymentUsecase>,
    ) -> Self {
        Worker {
            redis_client,
            create_payment_usecase,
        }
    }

    pub async fn listen_for_payments(&self) -> infrastructure::Result<()> {
        let mut conn = self.redis_client.get_connection()?;
        let mut pub_sub = conn.as_pubsub();
        pub_sub.subscribe(START_PAYMENT_CHANNEL)?;
        loop {
            let msg = pub_sub.get_message();
            if msg.is_err() {
                error!("Error getting message from channel: {:?}", msg.err().unwrap());
                continue;
            }
            let payload_str: String = msg.unwrap().get_payload()?;
            let payment: Payment = serde_json::from_str(&payload_str)?;
            let create_payment_result = self.create_payment_usecase.execute(&payment).await;
            match create_payment_result {
                Ok(p) => {
                    debug!("Payment created successfully {:?}", p);
                }
                Err(error) => {
                    error!("Error creating payment: {}", error);
                }
            }
        }
    }
}
