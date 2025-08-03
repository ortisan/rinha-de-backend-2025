use crate::application::domain::payment::Payment;
use crate::application::repositories::payment_repository::PaymentRepository;
use crate::application::usecases::create_payment::CreatePaymentUsecase;
use crate::constants::START_PAYMENT_CHANNEL;
use redis::Client;
use std::sync::Arc;

pub struct Worker<T: PaymentRepository> {
    pub redis_client: Arc<Client>,
    pub create_payment_usecase: CreatePaymentUsecase<T>,
}

impl<T: PaymentRepository> Worker<T> {
    pub fn new(redis_client: Arc<Client>, create_payment_usecase: CreatePaymentUsecase<T>) -> Self {
        Worker {
            redis_client,
            create_payment_usecase,
        }
    }

    pub async fn listen_for_payments(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut con = self.redis_client.get_connection()?;
        let mut pubsub = con.as_pubsub();
        pubsub.subscribe(START_PAYMENT_CHANNEL)?;
        loop {
            let msg = pubsub.get_message()?;
            let payload_str: String = msg.get_payload()?;
            let payment: Payment = serde_json::from_str(&payload_str)?;
            self.create_payment_usecase.execute(payment).await?;
        }
    }
}
