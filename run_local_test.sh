#!/bin/sh

RINHA_TEST_DIR="./rinha-test"
PARTIAL_RESULTS_FILE="./partial-results.json"

echo "🦀 Starting local rinha tests..."

rm -rf $PARTIAL_RESULTS_FILE

echo "Waiting for backend to be ready..."

attempt=1
max_attempts=15
success=1

while [ $success -ne 0 ] && [ $max_attempts -ge $attempt ]; do
    curl -f -s http://localhost:9999/payments-summary > /dev/null

    success=$?

    echo "Health check attempt $attempt of $max_attempts..."

    sleep 2

    ((attempt++))
done

if [ $success -eq 0 ]; then
    echo "Backend is ready! Starting k6 tests..."

    k6 run $RINHA_TEST_DIR/rinha.js

    if [ -f $PARTIAL_RESULTS_FILE ]; then
        echo ""
        echo "=== RESULTADOS DA RINHA ==="
        echo ""

        cat $PARTIAL_RESULTS_FILE | jq -r '
            "P99: " + .p99.valor,
            "Bônus por desempenho: " + (.p99.bonus * 100 | tostring) + "%",
            "",
            "Multa:",
            "  - Porcentagem: " + (.multa.porcentagem * 100 | tostring) + "%",
            "  - Total: $" + (.multa.total | tostring),
            "  - Inconsistências: " + (.multa.composicao.total_inconsistencias | tostring),
            "",
            "Pagamentos:",
            "  - Default: " + (.pagamentos_realizados_default.num_pagamentos | tostring) + " ($" + (.pagamentos_realizados_default.total_bruto | tostring) + ")",
            "  - Fallback: " + (.pagamentos_realizados_fallback.num_pagamentos | tostring) + " ($" + (.pagamentos_realizados_fallback.total_bruto | tostring) + ")",
            "",
            "Total Bruto: $" + (.total_bruto | tostring),
            "Total Taxas: $" + (.total_taxas | tostring),
            "",
            "=============================",
            "LUCRO FINAL: $" + (.total_liquido | tostring),
            "============================="
        '

        echo ""
        echo "Test completed! Full results saved to $PARTIAL_RESULTS_FILE"
    else
        echo "Error: no results file generated"
    fi
else
    echo "Backend failed to respond after $max_attempts attempts"
fi
