use diesel::{Insertable, Queryable};
use serde::{Deserialize, Serialize};
use crate::application::domain::payment::Payment;

#[derive(Debug, Serialize, Deserialize, Insertable, Queryable, Clone)]
#[diesel(table_name = payments)]  // Removed the quotes around "payments"
pub struct PaymentModel {
    correlation_id: String,
    amount: f64,
    requested_at: String,
}

impl From<Payment> for PaymentModel {
    fn from(payment: Payment) -> Self {
        PaymentModel {
            correlation_id: payment.correlation_id,
            amount: payment.amount,
            requested_at: payment.requested_at
        }
    }
}

impl From<PaymentModel> for Payment {
    fn from(value: PaymentModel) -> Self {
        Payment {
            correlation_id: value.correlation_id,
            amount: value.amount,
            requested_at: value.requested_at
        }
    }
}