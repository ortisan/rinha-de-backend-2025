// use crate::application::domain::payment::{Payment, PaymentsSummary};
// use crate::application::repositories::models::PaymentModel;
// use crate::application::repositories::payment_repository::PaymentRepository;
// use crate::application::repositories::schema::payments;
// use crate::application::repositories::schema::payments::dsl::*;
// use crate::application::repositories::schema::payments::requested_at;
// use async_trait::async_trait;
// use bigdecimal::ToPrimitive;
// use chrono::{DateTime, Utc};
// use diesel::PgConnection;
// use diesel::dsl::insert_into;
// use diesel::prelude::*;
// use diesel::r2d2::ConnectionManager;
// use std::clone::Clone;
// use std::fmt::Error;
// use std::sync::Arc;
//
// pub struct DbConfig {
//     pub(crate) database_url: String,
// }
//
// pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;
//
// pub fn get_connection(url: &str) -> DbPool {
//     let manager = ConnectionManager::<PgConnection>::new(url);
//     r2d2::Pool::builder()
//         .build(manager)
//         .expect("Failed to create pool.")
// }
//
// pub struct PostgresPaymentRepository {
//     pool: DbPool,
// }
//
// impl PostgresPaymentRepository {
//     pub fn new(db_config: DbConfig) -> Self {
//         let pool = get_connection(&db_config.database_url);
//         PostgresPaymentRepository { pool }
//     }
// }
//
// #[async_trait]
// impl PaymentRepository for PostgresPaymentRepository {
//     async fn create(&self, payment: Payment) -> Result<Payment, Error> {
//         let payment_model = PaymentModel::from(payment.clone());
//
//         let connection_result = &mut self.pool.get();
//         if connection_result.is_err() {
//             return Err(Error);
//         }
//
//         let connection = connection_result.as_mut().unwrap();
//
//         let insert_result = insert_into(payments::table)
//             .values(&payment_model)
//             .execute(connection);
//
//         if insert_result.is_err() {
//             return Err(Error);
//         }
//
//         Ok(payment)
//     }
//
//     async fn get_summary(
//         &self,
//         from: DateTime<Utc>,
//         to: DateTime<Utc>,
//     ) -> Result<PaymentsSummary, Error> {
//         let connection_result = &mut self.pool.get();
//         if connection_result.is_err() {
//             return Err(Error);
//         }
//
//         let query_result = payments
//             .filter(requested_at.between(from.naive_utc(), to.naive_utc()))
//             .load::<PaymentModel>(connection_result.as_mut().unwrap());
//
//         if query_result.is_err() {
//             return Err(Error);
//         }
//
//         let mut payments_query_result: Vec<PaymentModel> = query_result.unwrap();
//         let mut total_payments: u64 = 0;
//         let mut total_amount: f64 = 0.0;
//         for payment in payments_query_result {
//             total_payments += 1;
//             total_amount += payment.amount.to_f64().unwrap();
//         }
//         Ok(PaymentsSummary {
//             total_payments,
//             total_amount,
//         })
//     }
// }
