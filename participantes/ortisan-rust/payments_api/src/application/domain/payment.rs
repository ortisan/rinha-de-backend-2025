use diesel::Queryable;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct Payment {
    pub id: Option<String>,
    pub correlation_id: String,
    pub amount: f64,
    pub requested_at: String // TODO change to temporal
}

pub struct PaymentsSummary {
    pub total_payments: u64,
    pub total_amount: f64,
}