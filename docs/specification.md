# Product specification

## Problem and users

Platform engineers and AI safety operators need to run long-lived, tool-using sessions without treating a compacted quotation as new user authority. Existing flat transcript compaction can erase who asserted a sentence. Context Bastion makes provenance visible and enforceable before an effect.

## Capability

Create a governed session, append authenticated user/tool/assistant evidence, compact it into claim-level provenance, recall analogues, evaluate an action through Regorus and a native guard, write the complete state to RVF, and recover it after process restart.

## Acceptance

1. Primary: a browser action traverses auth, typed API, domain crates, RuVector, Regorus and RVF; the rendered result includes a receipt.
2. Denial/recovery: forged tool approval is denied with zero effects; an authentic user event then permits exactly one effect.
3. Durable retrieval: a supervised backend process exits and restarts; refresh retrieves events, summary, decision and receipt from RVF.

Success means all three named Playwright tests pass, real upstream targeted tests pass, and immutable evidence is bound to the tested SHA. This is a synthetic prototype, not a production deployment.
