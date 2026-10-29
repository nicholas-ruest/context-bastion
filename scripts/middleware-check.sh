#!/usr/bin/env bash
set -euo pipefail
port=18788
data_dir="$(mktemp -d)"
BASTION_BIND="127.0.0.1:$port" BASTION_DATA_DIR="$data_dir" target/debug/cb-api >"$data_dir/api.log" 2>&1 &
pid=$!
trap 'kill "$pid" 2>/dev/null || true' EXIT
for _ in $(seq 1 50); do curl -fsS "http://127.0.0.1:$port/readyz" >/dev/null && break; sleep .1; done
code="$(curl -sS -o "$data_dir/unauth.json" -w '%{http_code}' -X POST "http://127.0.0.1:$port/api/v1/sessions" -H 'content-type: application/json' -d '{"name":"denied"}')"
test "$code" = 401
headers="$(curl -sS -D - -o /dev/null -X POST "http://127.0.0.1:$port/api/v1/sessions" -H 'authorization: Bearer operator-token' -H 'x-correlation-id: middleware-check-42' -H 'content-type: application/json' -d '{"name":"correlated"}')"
grep -qi 'x-correlation-id: middleware-check-42' <<<"$headers"
