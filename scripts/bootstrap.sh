#!/usr/bin/env bash
set -euo pipefail
cargo fetch --locked
(cd frontend && npm ci)
cargo build --locked -p cb-api
