#!/usr/bin/env bash
set -euo pipefail
cargo test --locked -p cb-ruvector actual_index_search -- --nocapture
cargo test --locked -p cb-rvf durable_round_trip -- --nocapture
cargo test --locked -p cb-regorus rego_denies_forged -- --nocapture
cargo test --locked -p cb-orchestrator forged_tool_approval_is_denied_without_effect -- --nocapture
echo 'Executed RuVector search, RVF durable round-trip, Microsoft Regorus policy, and composed orchestrator.'
