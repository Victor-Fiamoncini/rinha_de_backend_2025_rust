#!/bin/sh

export PAYMENT_PROCESSOR_DEFAULT_URL=http://127.0.0.1:8001
export PAYMENT_PROCESSOR_FALLBACK_URL=http://127.0.0.1:8002
export POSTGRES_HOST=localhost
export POSTGRES_PORT=5432
export POSTGRES_DB=postgres
export POSTGRES_USER=postgres
export POSTGRES_PASSWORD=postgres
export REDIS_URL=redis://127.0.0.1:6379

cargo run -p rinha_worker
