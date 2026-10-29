# Traceability

| Requirement | UI / API | Crates | Test evidence |
|---|---|---|---|
| Typed session | Command / sessions, events | types, event-ingest, session | primary journey |
| Safe compaction | Provenance / compact | compaction, summary, provenance | laundering unit + primary |
| Real recall | Provenance | embedding, ruvector, recall | integration + ablation |
| Policy denial | Action gate / actions | approval, policy, regorus, action-gate | denial-recovery |
| Durable audit | Audit / session read | rvf, repository, audit, replay | durable-retrieval |
| Auth/correlation | all routes | auth, middleware, api | middleware check + trace |
| Evidence | acceptance scripts | evaluation, seed | schema-v2 artifact + checker |
