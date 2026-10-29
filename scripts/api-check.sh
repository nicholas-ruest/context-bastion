#!/usr/bin/env bash
set -euo pipefail
port=18787
data_dir="$(mktemp -d)"
BASTION_BIND="127.0.0.1:$port" BASTION_DATA_DIR="$data_dir" target/debug/cb-api >"$data_dir/api.log" 2>&1 &
pid=$!
trap 'kill "$pid" 2>/dev/null || true' EXIT
for _ in $(seq 1 50); do curl -fsS "http://127.0.0.1:$port/readyz" && break; sleep .1; done
curl -fsS "http://127.0.0.1:$port/healthz"
curl -fsS -X POST "http://127.0.0.1:$port/api/v1/sessions" -H 'authorization: Bearer operator-token' -H 'content-type: application/json' -H 'x-correlation-id: api-check-1' -d '{"name":"API contract check"}' | grep -q 'API contract check'
