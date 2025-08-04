use crate::application::domain::payment::Payment;
use crate::application::usecases::create_payment::CreatePaymentUsecase;
use crate::constants::START_PAYMENT_CHANNEL;
use crate::infrastructure;
use redis::Client;
use std::sync::Arc;

pub struct Worker<'a, 'b> {
    pub redis_client: &'a Arc<Client>,
    pub create_payment_usecase: CreatePaymentUsecase<'a, 'b>,
}

impl<'a, 'b, 'c> Worker<'a, 'b> {
    pub fn new(redis_client: &'a Arc<redis::Client>, create_payment_usecase: CreatePaymentUsecase<'a, 'b>) -> Self {
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
            self.create_payment_usecase.execute(&payment).await?;
        }
    }
}
