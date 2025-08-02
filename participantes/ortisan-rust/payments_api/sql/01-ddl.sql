CREATE UNLOGGED TABLE payments
(
    correlation_id VARCHAR(50) PRIMARY KEY,
    amount        DECIMAL   NOT NULL,
    requested_at  TIMESTAMP NOT NULL
);

CREATE INDEX payments_requested_at ON payments (requested_at);