#!/bin/sh

export API_PORT=3000
export REDIS_URL=redis://127.0.0.1:6379

cargo run -p rinha_api
