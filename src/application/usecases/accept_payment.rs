use crate::application::domain::payment::Payment;
use crate::constants::START_PAYMENT_CHANNEL;
use crate::infrastructure;
use redis::{Client, Commands};
use std::sync::Arc;

pub struct AcceptPaymentUsecase {
    pub redis_client: Arc<Client>,
}

impl AcceptPaymentUsecase {
    pub fn new(redis_client: Arc<Client>) -> Self {
        AcceptPaymentUsecase { redis_client }
    }

    pub async fn execute(&self, payment: Payment) -> infrastructure::Result<Payment> {
        let user_json = serde_json::to_string(&payment)?;

        let mut conn = self.redis_client.get_connection()?;
        let _: usize = conn.publish(START_PAYMENT_CHANNEL, user_json)?;

        Ok(payment)
    }
}
