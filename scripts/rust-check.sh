#!/usr/bin/env bash
set -euo pipefail
cargo fmt --all --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features --locked
cargo deny check
node scripts/check-structure.mjs
