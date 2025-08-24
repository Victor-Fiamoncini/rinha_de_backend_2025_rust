#!/bin/sh

PAYMENT_PROCESSOR_URL_DEFAULT=http://payment-processor-default:8080 \
PAYMENT_PROCESSOR_URL_FALLBACK=http://payment-processor-fallback:8080 \
REDIS_URL=redis://127.0.0.1:6379 \
cargo run -p rinha_worker
