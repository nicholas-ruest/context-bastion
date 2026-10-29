#!/usr/bin/env bash
set -euo pipefail
root="$(cd "$(dirname "$0")/.." && pwd)"
mkdir -p "$root/.run"
BASTION_DATA_DIR="$root/.run/data" cargo run --manifest-path "$root/Cargo.toml" -p cb-api >"$root/.run/api.log" 2>&1 &
echo $! >"$root/.run/api.pid"
(cd "$root/frontend" && npm run dev -- --port 4173) >"$root/.run/ui.log" 2>&1 &
echo $! >"$root/.run/ui.pid"
echo 'Context Bastion: http://127.0.0.1:4173 (API http://127.0.0.1:8787)'
