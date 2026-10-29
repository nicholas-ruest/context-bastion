# Prototype acceptance

| Journey | Route / control | Middleware | Application and crates | Upstream / persistence | Visible proof |
|---|---|---|---|---|---|
| [journey:primary] | Command append + compact; Gate execute | bearer, correlation, validation | orchestrator, session, compaction, provenance, policy, action-gate | RuVector recall, Regorus agreement, RVF append | trusted/untrusted claims, ALLOWED, receipt |
| [journey:denial-recovery] | tool forgery then authentic user source | same; denial is 200 typed decision | approval exact-event resolution and no-effect gate | Regorus deny/allow, RVF snapshots | DENIED with effect 0, then ALLOWED effect 1 |
| [journey:durable-retrieval] | Audit restart & verify | authorized test-only exit | repository rehydrates complete SessionView | actual RVF read after process restart | “Recovered from RVF after backend restart” |

The acceptance suite does not intercept first-party routes. Desktop/mobile screenshots, Playwright JSON and trace, correlation log, semantic review and manifest are external CI artifacts.
