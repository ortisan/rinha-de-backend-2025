use crate::application::domain::payment::{GetPaymentsFilter, Payment, PaymentsSummary};
use chrono::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentRequest {
    #[serde(rename = "correlationId")]
    correlation_id: String,
    amount: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentResponse {
    #[serde(rename = "correlationId")]
    correlation_id: String,
    amount: f64,
    requested_at: DateTime<Utc>,
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

impl From<&Payment> for PaymentResponse {
    fn from(value: &Payment) -> Self {
        PaymentResponse {
            correlation_id: value.correlation_id.clone(),
            amount: value.amount,
            requested_at: value.requested_at,
        }
    }
}

#[derive(Deserialize)]
pub struct GetPaymentsSummaryFilter {
    pub from: String,
    pub to: String,
}

impl From<GetPaymentsSummaryFilter> for GetPaymentsFilter {
    fn from(value: GetPaymentsSummaryFilter) -> Self {
        let from = chrono::DateTime::parse_from_rfc3339(&value.from)
            .unwrap()
            .to_utc();
        let to = chrono::DateTime::parse_from_rfc3339(&value.to)
            .unwrap()
            .to_utc();
        GetPaymentsFilter { from, to }
    }
}

#[derive(Serialize, Deserialize)]
pub struct PaymentsSummaryResponse {
    #[serde(rename = "totalPayments")]
    pub total_payments: u64,
    #[serde(rename = "totalAmount")]
    pub total_amount: f64,
}

impl From<PaymentsSummary> for PaymentsSummaryResponse {
    fn from(value: PaymentsSummary) -> Self {
        PaymentsSummaryResponse {
            total_payments: value.total_payments,
            total_amount: value.total_amount,
        }
    }
}
