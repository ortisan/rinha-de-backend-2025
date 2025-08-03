use crate::application::domain::payment::Payment;
use crate::constants::START_PAYMENT_CHANNEL;
use redis::{Client, Commands};
use std::sync::Arc;

pub struct AcceptPaymentUsecase {
    pub redis_client: Arc<Client>,
}

impl AcceptPaymentUsecase {
    pub fn new(redis_client: Arc<Client>) -> Self {
        AcceptPaymentUsecase { redis_client }
    }

    pub async fn execute(&self, payment: Payment) -> Result<Payment, redis::RedisError> {
        let user_json = serde_json::to_string(&payment)?;

        let conn_result = self.redis_client.get_connection();
        match conn_result {
            Ok(mut conn) => {
                let publish_result: Result<usize, redis::RedisError> =
                    conn.publish(START_PAYMENT_CHANNEL, user_json);
                match publish_result {
                    Ok(_) => {
                        return Ok(payment);
                    }
                    Err(error) => {
                        return Err(error);
                    }
                }
            }
            Err(error) => {
                return Err(error);
            }
        }
    }
}
