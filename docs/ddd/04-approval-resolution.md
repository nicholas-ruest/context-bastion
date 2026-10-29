# Approval Resolution bounded context

## Ubiquitous language

- **Approval**: a domain value whose identity and lifecycle are owned here.
- **AuthenticUserSource**: a domain value whose identity and lifecycle are owned here.
- **ApprovalEventId**: a domain value whose identity and lifecycle are owned here.

## Boundary and ownership

This context owns approval resolution rules and publishes typed results; it does not infer authority from display text. Owning implementation: **cb-approval, cb-provenance**. Cross-context work is composed by cb-orchestrator, not by reaching into another crate's storage.

## Commands and events

Primary command: resolve approval against one immutable event. Successful commands emit state used by the browser journey and append a recovery snapshot through the archive boundary. Rejected commands return an explicit error or denied decision with no forbidden side effect.

## Invariants

Identity, source role, authentication state, and correlation are preserved. Trust may stay equal or decrease during derivation; it cannot increase because of compaction or semantic similarity. Effects require a preceding allowed decision and an idempotency key.

## Failure model and tests

Required failures: tool source; unauthenticated user; absent ID. Unit tests target the aggregate rule; integration tests cross the real upstream boundary where applicable; Playwright demonstrates the visible outcome without intercepting first-party routes.
