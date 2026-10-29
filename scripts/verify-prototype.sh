#!/usr/bin/env bash
set -euo pipefail
root="$(cd "$(dirname "$0")/.." && pwd)"
evidence="$1"
sha="$(git -C "$root" rev-parse HEAD)"
test -z "$(git -C "$root" status --porcelain --untracked-files=all)"
mkdir -p "$evidence"
node scripts/dream-machine-product-gate.mjs record "$root" "$evidence" rust -- bash scripts/rust-check.sh
node scripts/dream-machine-product-gate.mjs record "$root" "$evidence" frontend -- bash scripts/frontend-check.sh
cargo build --locked -p cb-api
node scripts/dream-machine-product-gate.mjs record "$root" "$evidence" api -- bash scripts/api-check.sh
node scripts/dream-machine-product-gate.mjs record "$root" "$evidence" middleware -- bash scripts/middleware-check.sh
node scripts/dream-machine-product-gate.mjs record "$root" "$evidence" integrations -- bash scripts/integrations-check.sh
node scripts/dream-machine-product-gate.mjs record "$root" "$evidence" benchmark -- bash scripts/benchmark.sh
node scripts/dream-machine-product-gate.mjs record "$root" "$evidence" ablations -- bash scripts/ablations.sh
node scripts/dream-machine-product-gate.mjs record "$root" "$evidence" containers -- bash scripts/containers-check.sh
node scripts/dream-machine-product-gate.mjs record "$root" "$evidence" browser -- bash scripts/browser-check.sh
node scripts/make-manifest.mjs "$evidence" "$sha"
node scripts/dream-machine-product-gate.mjs verify "$root" "$evidence" "$sha"
