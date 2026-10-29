# Architecture

## System thesis

Context Bastion is a provenance-safe context and action runtime. It prevents authority laundering when long agent sessions are compacted: display text can change, but typed role, source authentication, claim lineage, and immutable event identity remain available to the policy gate.

## Runtime topology

Browser controls call the typed TypeScript client, Axum middleware authenticates and correlates, cb-orchestrator coordinates the session/provenance/governance contexts, Regorus agrees with the native policy, RuVector recalls related claims, and RVF appends the recovery snapshot. The browser never holds service credentials.

## Crate map

| Crate | Owned behavior | Real caller |
|---|---|---|
| cb-types | canonical events, claims, decisions, receipts and API DTOs | all crates |
| cb-errors | explicit domain and boundary failures | all boundary crates |
| cb-config | validated runtime namespace, bind and persistence configuration | cb-api |
| cb-auth | operator bearer authentication | cb-middleware |
| cb-middleware | auth context and correlation propagation | cb-api |
| cb-event-ingest | role/source validation and immutable event creation | cb-orchestrator |
| cb-session | session aggregate and sequence invariants | cb-orchestrator |
| cb-compaction | eligible window and retention plan | cb-orchestrator |
| cb-summary | claim-bearing compacted representation | cb-orchestrator |
| cb-provenance | source graph, trust classification and approval lookup | cb-approval, cb-orchestrator |
| cb-approval | high-risk authentic approval resolution | cb-orchestrator |
| cb-policy | native fail-closed decision engine | cb-orchestrator |
| cb-regorus | Microsoft Regorus policy execution | cb-orchestrator |
| cb-action-gate | idempotent effect boundary | cb-orchestrator |
| cb-embedding | deterministic claim vectorization | cb-ruvector |
| cb-ruvector | RuVector claim index and nearest-neighbor search | cb-recall, cb-orchestrator |
| cb-recall | trust-fenced analogue projection | cb-orchestrator |
| cb-rvf | RVF append/read archive | cb-repository, cb-orchestrator |
| cb-repository | snapshot serialization and recovery semantics | cb-orchestrator |
| cb-audit | ordered audit entries | cb-orchestrator |
| cb-replay | human-readable replay frames | cb-orchestrator |
| cb-evaluation | laundering and policy-agreement metrics | benchmarks |
| cb-seed | deterministic synthetic acceptance fixture | integration tests |
| cb-orchestrator | application use cases across every context | cb-api |
| cb-api | versioned Axum HTTP service and health | browser and client |

All 25 crates have typed public APIs, direct callers or an executable role, focused tests, and references to the ADR/DDD indexes. cb-orchestrator is the dependency hub by design; adapters do not call the UI or policy around the application layer.

## Deployment

The backend and static frontend are stateless Cloud Run-compatible services. Authorized RVF/RuVector persistence is configured by namespace and mount/adapter. Health is /healthz, readiness is /readyz, frontend is :4173 locally, and API is :8787.
