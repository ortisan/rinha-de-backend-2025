use diesel::table;

table! {
    payments (correlation_id) {
        correlation_id -> Varchar,
        amount -> Numeric,
        requested_at -> Timestamptz
    }
}