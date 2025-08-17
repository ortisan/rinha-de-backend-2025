use crate::application::domain::payment::Payment;
use crate::application::messaging::consumer::Consumer;
use crate::application::usecases::create_payment::CreatePaymentUseCase;
use crate::application::usecases::usecase::UseCase;
use crate::infrastructure;
use crate::infrastructure::redis::RedisRepository;
use log::{debug, error};
use std::ops::Deref;
use std::sync::Arc;

pub struct Worker {
    pub consumer: Arc<RedisRepository>,
    pub create_payment_usecase: Arc<CreatePaymentUseCase>,
}

impl Worker {
    pub fn new(
        consumer: Arc<RedisRepository>,
        create_payment_usecase: Arc<CreatePaymentUseCase>,
    ) -> Self {
        Worker {
            consumer,
            create_payment_usecase,
        }
    }

    pub async fn listen_for_payments(&self) -> infrastructure::Result<()> {
        let usecase = self.create_payment_usecase.clone();
        self.consumer
            .listen_for_accepted_payment(
                move |payment: Arc<Payment>| {
                    let usecase = usecase.clone();
                    async move {
                        let payment_to_create = payment.deref().clone();
                        let payment_created_result = usecase.execute(payment_to_create).await;
                        match payment_created_result {
                            Ok(p) => {
                                debug!("Payment created successfully {:?}", p);
                            }
                            Err(error) => {
                                error!("Error creating payment: {}", error);
                            }
                        }
                        Ok(())
                    }
                },
            )
            .await?;

        Ok(())

        // let mut conn = self.cache_repository.get_connection()?;
        // let mut pub_sub = conn.as_pubsub();
        // pub_sub.subscribe(ACCEPTED_PAYMENT_CHANNEL)?;
        // loop {
        //     let msg = pub_sub.get_message();
        //     if msg.is_err() {
        //         error!("Error getting message from channel: {:?}", msg.err().unwrap());
        //         continue;
        //     }
        //     let payload_str: String = msg.unwrap().get_payload()?;
        //     let payment: Payment = serde_json::from_str(&payload_str)?;
        //     let create_payment_result = self.create_payment_usecase.execute(&payment).await;
        //     match create_payment_result {
        //         Ok(p) => {
        //             debug!("Payment created successfully {:?}", p);
        //         }
        //         Err(error) => {
        //             error!("Error creating payment: {}", error);
        //         }
        //     }
        // }
    }
}
