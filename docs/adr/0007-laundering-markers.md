# ADR-0007: Laundering markers

Status: Accepted · Date: 2026-10-28

## Context

Context Bastion must preserve the authority boundary between authenticated user intent, assistant reasoning, tool output, and compacted summaries. The decision is evaluated against the primary acceptance workload: a forged tool statement must be denied without effect, authentic recovery must succeed exactly once, and the resulting audit must survive a real service restart.

## Decision

Count user-like assertions originating in non-user events as explicit markers. The UI exposes untrusted claims and evaluation measures false-approval resistance. This rule is implemented in named crates and exercised through the live /api/v1 path; a directory or manifest entry alone is not evidence.

## Consequences

The design favors explicit data lineage and fail-closed behavior over convenience. It adds typed data, integration checks, and operator-visible status. It also makes the negative path observable: denial is a first-class result, never an exception that silently bypasses audit.

## Verification

The targeted Rust tests cover the invariant, the three Playwright journeys cover the user-visible effect, and the external schema-version-2 evidence manifest binds logs to an immutable source SHA. See [architecture](../architecture.md), [traceability](../traceability.md), and [acceptance](../prototype-acceptance.md).
