# Policy Governance bounded context

## Ubiquitous language

- **PolicyInput**: a domain value whose identity and lifecycle are owned here.
- **NativeDecision**: a domain value whose identity and lifecycle are owned here.
- **RegoDecision**: a domain value whose identity and lifecycle are owned here.
- **Agreement**: a domain value whose identity and lifecycle are owned here.

## Boundary and ownership

This context owns policy governance rules and publishes typed results; it does not infer authority from display text. Owning implementation: **cb-policy, cb-regorus**. Cross-context work is composed by cb-orchestrator, not by reaching into another crate's storage.

## Commands and events

Primary command: evaluate risk and approval through both engines. Successful commands emit state used by the browser journey and append a recovery snapshot through the archive boundary. Rejected commands return an explicit error or denied decision with no forbidden side effect.

## Invariants

Identity, source role, authentication state, and correlation are preserved. Trust may stay equal or decrease during derivation; it cannot increase because of compaction or semantic similarity. Effects require a preceding allowed decision and an idempotency key.

## Failure model and tests

Required failures: engine disagreement; malformed policy input. Unit tests target the aggregate rule; integration tests cross the real upstream boundary where applicable; Playwright demonstrates the visible outcome without intercepting first-party routes.
