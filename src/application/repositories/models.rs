use crate::application::domain::payment::Payment;
use bigdecimal::{BigDecimal, FromPrimitive, ToPrimitive};
use chrono::NaiveDateTime;
use diesel::{Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Insertable, Serialize, Deserialize)]
#[diesel(table_name = crate::application::repositories::schema::payments)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct PaymentModel {
    #[serde(rename = "correlationId")]
    pub correlation_id: String,
    pub amount: BigDecimal,
    pub requested_at: NaiveDateTime,
}

impl From<Payment> for PaymentModel {
    fn from(payment: Payment) -> Self {
        PaymentModel {
            correlation_id: payment.correlation_id,
            amount: BigDecimal::from_f64(payment.amount).unwrap(),
            requested_at: payment.requested_at.naive_utc(),
        }
    }
}

impl From<PaymentModel> for Payment {
    fn from(value: PaymentModel) -> Self {
        Payment {
            correlation_id: value.correlation_id,
            amount: value.amount.to_f64().unwrap_or_default(),
            requested_at: value.requested_at.and_utc(),
        }
    }
}
