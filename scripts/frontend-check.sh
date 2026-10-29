#!/usr/bin/env bash
set -euo pipefail
cd frontend
npm ci
npm run typecheck
npm run lint
npm test -- --run
npm run build
npm run contract
