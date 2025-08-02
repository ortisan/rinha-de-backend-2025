use chrono::{DateTime, FixedOffset, Utc};
use diesel::Queryable;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Queryable, Clone)]
pub struct Payment {
    pub correlation_id: String,
    pub amount: f64,
    pub requested_at: DateTime<FixedOffset>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GetPaymentsFilter {
    pub from: DateTime<FixedOffset>,
    pub to: DateTime<FixedOffset>,
}

pub struct PaymentsSummary {
    pub total_payments: u64,
    pub total_amount: f64,
}
