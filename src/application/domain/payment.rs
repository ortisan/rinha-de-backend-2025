use chrono::serde::ts_nanoseconds;
use chrono::{DateTime, FixedOffset, Utc};
use diesel::Queryable;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Queryable, Clone)]
pub struct Payment {
    pub correlation_id: String,
    pub amount: f64,
    #[serde(with = "ts_nanoseconds")]
    pub requested_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GetPaymentsFilter {
    #[serde(with = "ts_nanoseconds")]
    pub from: DateTime<Utc>,
    #[serde(with = "ts_nanoseconds")]
    pub to: DateTime<Utc>,
}

pub struct PaymentsSummary {
    pub total_payments: u64,
    pub total_amount: f64,
}
