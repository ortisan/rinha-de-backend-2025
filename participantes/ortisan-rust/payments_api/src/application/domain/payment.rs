use chrono::{DateTime, Utc};
use diesel::Queryable;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Queryable, Clone)]
pub struct Payment {
    pub correlation_id: String,
    pub amount: f64,
    pub requested_at: DateTime<Utc>
}

pub struct PaymentsSummary {
    pub total_payments: u64,
    pub total_amount: f64,
}