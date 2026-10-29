#!/usr/bin/env bash
set -euo pipefail
docker compose build
docker compose up -d
trap 'docker compose down -v' EXIT
for _ in $(seq 1 80); do curl -fsS http://127.0.0.1:8787/readyz >/dev/null && curl -fsS http://127.0.0.1:4173 >/dev/null && exit 0; sleep 1; done
docker compose logs
exit 1
