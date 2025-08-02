use crate::application::domain::payment::Payment;
use chrono::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentRequest {
    #[serde(rename = "correlationId")]
    correlation_id: String,
    amount: f64,
}

impl From<PaymentRequest> for Payment {
    fn from(value: PaymentRequest) -> Self {
        let requested_at = Utc::now();
        Payment {
            correlation_id: value.correlation_id,
            amount: value.amount,
            requested_at,
        }
    }
}

impl From<Payment> for PaymentRequest {
    fn from(value: Payment) -> Self {
        PaymentRequest {
            correlation_id: value.correlation_id,
            amount: value.amount,
        }
    }
}
