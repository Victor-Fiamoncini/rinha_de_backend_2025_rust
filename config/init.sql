CREATE UNLOGGED TABLE payments (
    amount DECIMAL NOT NULL,
    correlation_id UUID PRIMARY KEY,
    payment_processor VARCHAR(8) NOT NULL,
    requested_at TIMESTAMP NOT NULL
);

CREATE INDEX payments_requested_at_index ON payments (requested_at);
CREATE INDEX payments_payment_processor_index ON payments (payment_processor);
