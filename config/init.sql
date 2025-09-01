DO $$
BEGIN
    IF NOT EXISTS (SELECT FROM pg_tables WHERE schemaname = 'public' AND tablename = 'payments') THEN
        CREATE UNLOGGED TABLE payments (
            amount DECIMAL NOT NULL,
            correlation_id UUID PRIMARY KEY,
            payment_processor VARCHAR(8) NOT NULL,
            requested_at TIMESTAMP NOT NULL
        );
    END IF;

    IF NOT EXISTS (
        SELECT FROM pg_indexes WHERE schemaname = 'public' AND indexname = 'payments_requested_at_index'
    ) THEN
        CREATE INDEX payments_requested_at_index ON payments (requested_at);
    END IF;

    IF NOT EXISTS (
        SELECT FROM pg_indexes WHERE schemaname = 'public' AND indexname = 'payments_payment_processor_index'
    ) THEN
        CREATE INDEX payments_payment_processor_index ON payments (payment_processor);
    END IF;
END $$;
