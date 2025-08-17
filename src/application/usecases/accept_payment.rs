use crate::application::domain::payment::Payment;
use crate::application::messaging::publisher::Publisher;
use crate::application::usecases::usecase::UseCase;
use crate::infrastructure;
use std::sync::Arc;

pub struct AcceptPaymentUsecase {
    pub publisher: Arc<dyn Publisher>,
}

impl AcceptPaymentUsecase {
    pub fn new(publisher: Arc<dyn Publisher>) -> Self {
        AcceptPaymentUsecase { publisher }
    }
}

#[async_trait::async_trait]
impl UseCase<Payment, Payment> for AcceptPaymentUsecase {
    async fn execute(&self, payment: Payment) -> infrastructure::Result<Payment> {
        let _ = self.publisher.publish_accepted_payment(&payment).await?;
        Ok(payment)
    }
}
