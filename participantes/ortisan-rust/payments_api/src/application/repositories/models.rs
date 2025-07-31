use diesel::{Insertable, Queryable, Selectable};
use bigdecimal::{BigDecimal, FromPrimitive, ToPrimitive};
use chrono::NaiveDateTime;
use crate::application::domain::payment::Payment;


#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::application::repositories::schema::payments)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct PaymentModel {
    correlation_id: String,
    amount: BigDecimal,
    requested_at: NaiveDateTime,
}

impl From<Payment> for PaymentModel {
    fn from(payment: Payment) -> Self {
        PaymentModel {
            correlation_id: payment.correlation_id,
            amount: BigDecimal::from_f64(payment.amount).unwrap(),
            requested_at: NaiveDateTime::parse_from_str(&payment.requested_at, "%Y-%m-%d %H:%M:%S").unwrap()
        }
    }
}

impl From<PaymentModel> for Payment {
    fn from(value: PaymentModel) -> Self {
        Payment {
            correlation_id: value.correlation_id,
            amount: value.amount.to_f64().unwrap_or_default(),
            requested_at: value.requested_at.format("%Y-%m-%d %H:%M:%S").to_string()
        }
    }
}