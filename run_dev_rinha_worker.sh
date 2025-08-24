#!/bin/sh

export PAYMENT_PROCESSOR_DEFAULT_URL=http://payment-processor-default:8080
export PAYMENT_PROCESSOR_FALLBACK_URL=http://payment-processor-fallback:8080
export REDIS_URL=redis://127.0.0.1:6379

cargo run -p rinha_worker
