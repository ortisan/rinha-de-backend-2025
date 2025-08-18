use chrono::serde::ts_nanoseconds;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
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
