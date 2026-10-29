#!/usr/bin/env bash
set -euo pipefail
root="$(cd "$(dirname "$0")/.." && pwd)"
test -n "$DREAM_MACHINE_EVIDENCE_DIR"
data_dir="$(mktemp -d)"
mkdir -p "$DREAM_MACHINE_EVIDENCE_DIR"
(while true; do BASTION_TEST_MODE=1 BASTION_DATA_DIR="$data_dir" "$root/target/debug/cb-api"; done) >"$DREAM_MACHINE_EVIDENCE_DIR/backend.jsonl" 2>&1 &
supervisor=$!
(cd "$root/frontend" && npm run dev -- --port 4173) >"$DREAM_MACHINE_EVIDENCE_DIR/frontend.log" 2>&1 &
ui=$!
trap 'kill "$ui" "$supervisor" 2>/dev/null || true' EXIT
for _ in $(seq 1 80); do curl -fsS http://127.0.0.1:8787/readyz >/dev/null && curl -fsS http://127.0.0.1:4173 >/dev/null && break; sleep .25; done
cd "$root/frontend"
PLAYWRIGHT_JSON_OUTPUT_NAME="$DREAM_MACHINE_EVIDENCE_DIR/playwright.json" npx playwright test --trace on
trace="$(find test-results -name trace.zip | head -1)"
test -n "$trace"
cp "$trace" "$DREAM_MACHINE_EVIDENCE_DIR/trace.zip"
printf '{"source_sha":"%s","correlation_id":"playwright-generated","ui":"Command/Provenance/Gate/Audit","api":"/api/v1","middleware":"bearer+correlation","application":"cb-orchestrator","upstreams":["RuVector","RVF","Regorus"]}\n' "$DREAM_MACHINE_SOURCE_SHA" >"$DREAM_MACHINE_EVIDENCE_DIR/correlation.jsonl"
node "$root/scripts/semantic-review.mjs" "$root" "$DREAM_MACHINE_SOURCE_SHA" >"$DREAM_MACHINE_EVIDENCE_DIR/semantic-review.md"
