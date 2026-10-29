# Research and composition

## Frontier mechanism

The implementation applies the Pixee Compaction Laundering finding at commit 9ce87b71301c628dfcd9232b01f1ccbdb0e67b16: flat summaries can preserve words while losing authority boundaries. Context Bastion changes the architecture by making every summary a graph of claims with typed source edges and monotone trust.

## Integration matrix

| Ingredient | Pin / license | Executed capability | Boundary and test | Ablation |
|---|---|---|---|---|
| ruvnet/RuVector | 4f6bbf9086e97fb6d1560e211aa740e5e7a75917; Apache-2.0 | ruvector-core 2.4.0 nearest claim recall | cb-ruvector; real index/search test | no semantic analogues |
| ruvnet/RuVector RVF | same repository; Apache-2.0 | rvf-runtime 0.4.0 append/read recovery | cb-rvf; real process restart journey | no durable recovery |
| Microsoft Regorus | a31eeb4cad1a7c917d98a73be7d8be8075bbe775; Apache-2.0/MIT notices | Rego-compatible policy evaluation | cb-regorus; policy agreement test | native policy only |
| Pixee mechanism | 9ce87b71301c628dfcd9232b01f1ccbdb0e67b16; MIT | typed compaction provenance | cb-provenance; forged-prefix denial | flat transcript baseline |

No single ingredient provides the product: RuVector supplies intelligence and archive primitives, Regorus supplies enterprise policy execution, and typed provenance supplies the safety mechanism.
