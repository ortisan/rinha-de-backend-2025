use std::fmt::Error;
use std::sync::Arc;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use diesel::PgConnection;
use diesel::r2d2::ConnectionManager;
use crate::application::domain::payment::{Payment, PaymentsSummary};
use crate::application::repositories::payment_repository::PaymentRepository;

pub struct DbConfig {
    pub(crate) database_url: String,
}

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn get_connection(url: &str) -> DbPool {
    let manager = ConnectionManager::<PgConnection>::new(url);
    r2d2::Pool::builder().build(manager).expect("Failed to create pool.")
}

pub struct PostgresPaymentRepository {
    pool: DbPool,
}

impl PostgresPaymentRepository {
    pub fn new(db_config: DbConfig) -> Self {
        let database_url = std::env::var(db_config.database_url).expect("DATABASE_URL must be set");
        let pool = get_connection(&database_url);
        PostgresPaymentRepository { pool }
    }
}

#[async_trait]
impl PaymentRepository for Arc<PostgresPaymentRepository> {
    async fn create(&self, payment: Payment) -> Result<Payment, Error> {
        todo!()
    }

    async fn summary(&self, from: DateTime<Utc>, to: DateTime<Utc>) -> Result<PaymentsSummary, Error> {
        todo!()
    }

}