# Core-memory guidance map

CORE_MEMORY_RECEIPT: nicholas-ruest/core-memory@a7bb2223aa47da7bff5926977890bb8b994ac192

| Source | Applied decision | Evidence |
|---|---|---|
| dream-machine-build-standard.md | minimum 20 meaningful Rust crates (23 independently counted; 25 total), 25 ADRs, 12 DDDs, real composition | architecture, semantic review |
| dream-machine-product-gate.md | product-first browser slice and external schema-v2 evidence | Playwright and CI artifact |
| learning-loop.md / ADR-0026 | only verified reviewed lessons may influence work | NONE_AVAILABLE digest receipt |
| AGENTS.md / CLAUDE.md | Rust quality, evidence, branch/PR workflow | validation logs and PR |

The copied product checker is blob 7cfdec81eccbeba9f746be476ff92b15787e7539 from the same source commit. Explicit user constraints remain authoritative.
