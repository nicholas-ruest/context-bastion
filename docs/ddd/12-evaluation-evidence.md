# Evaluation Evidence bounded context

## Ubiquitous language

- **Fixture**: a domain value whose identity and lifecycle are owned here.
- **LaunderingRate**: a domain value whose identity and lifecycle are owned here.
- **PolicyAgreement**: a domain value whose identity and lifecycle are owned here.
- **ArtifactDigest**: a domain value whose identity and lifecycle are owned here.

## Boundary and ownership

This context owns evaluation evidence rules and publishes typed results; it does not infer authority from display text. Owning implementation: **cb-evaluation, cb-seed, scripts**. Cross-context work is composed by cb-orchestrator, not by reaching into another crate's storage.

## Commands and events

Primary command: benchmark and record immutable evidence. Successful commands emit state used by the browser journey and append a recovery snapshot through the archive boundary. Rejected commands return an explicit error or denied decision with no forbidden side effect.

## Invariants

Identity, source role, authentication state, and correlation are preserved. Trust may stay equal or decrease during derivation; it cannot increase because of compaction or semantic similarity. Effects require a preceding allowed decision and an idempotency key.

## Failure model and tests

Required failures: stale SHA; mocked first-party API; absent trace. Unit tests target the aggregate rule; integration tests cross the real upstream boundary where applicable; Playwright demonstrates the visible outcome without intercepting first-party routes.
