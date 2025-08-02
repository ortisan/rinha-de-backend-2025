use crate::application::domain::payment::{Payment, PaymentsSummary};
use crate::application::repositories::models::PaymentModel;
use crate::application::repositories::payment_repository::PaymentRepository;
use crate::application::repositories::schema::payments;
use std::clone::Clone;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use diesel::PgConnection;
use diesel::dsl::insert_into;
use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use std::fmt::Error;
use std::sync::Arc;

pub struct DbConfig {
    pub(crate) database_url: String,
}

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn get_connection(url: &str) -> DbPool {
    let manager = ConnectionManager::<PgConnection>::new(url);
    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.")
}

pub struct PostgresPaymentRepository {
    pool: DbPool,
}

impl PostgresPaymentRepository {
    pub fn new(db_config: DbConfig) -> Self {
        let pool = get_connection(&db_config.database_url);
        PostgresPaymentRepository { pool }
    }
}

#[async_trait]
impl PaymentRepository for Arc<PostgresPaymentRepository> {
    async fn create(&self, payment: Payment) -> Result<Payment, Error> {
        let payment_model = PaymentModel::from(payment.clone());

        let connection_result = &mut self.pool.get();
        if connection_result.is_err() {
            return Err(Error);
        }

        let connection = connection_result.as_mut().unwrap();

        let insert_result = insert_into(payments::table)
            .values(&payment_model)
            .execute(connection);

        if insert_result.is_err() {
            return Err(Error);
        }

        Ok(payment)
    }

    async fn summary(
        &self,
        from: DateTime<Utc>,
        to: DateTime<Utc>,
    ) -> Result<PaymentsSummary, Error> {
        todo!()
    }
}
