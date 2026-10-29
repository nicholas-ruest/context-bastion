# Implementation contract

- The browser uses only /api/v1 through the checked TypeScript client.
- Every event stores typed role, source, authentication, sequence and digest.
- Compaction retains originals and emits source-linked claims.
- Similarity never changes trust; high-risk approval resolves one exact event ID.
- Native and Regorus decisions must agree; denial writes no receipt.
- Every mutation appends a recoverable RVF snapshot.
- The three named Playwright journeys call the real backend and upstreams.
- Evidence is generated outside a clean checkout, schema version 2, exact source SHA.
- BUILT additionally requires independent crate/call-path and UI review plus CI artifact readback.
