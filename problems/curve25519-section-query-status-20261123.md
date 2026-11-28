# Section-query component status, 2026-11-23

This addendum updates the implementation gap in the frozen
[curve25519-section-coefficient-selection.md](curve25519-section-coefficient-selection.md),
without changing that claim-pinned historical file or its original criteria.
The earlier [cost erratum](../research/curve25519_torus_native_20261115/reports/section-search-v1/ERRATA.md)
still applies: a sufficient upper-budget target is not a necessary lower bound.

| Requirement | Status/evidence |
|---|---|
| Native quotient products and occupied-interval restriction | Implemented;118 fixed native arms pass, producer unsigned PCC-20261123-be5351 |
| Supplied-section original witness replay | Fixed toy/actual q213156431,k31/k20633 controls, actual labels j1..6,t0; no coefficient discovery |
| Repeated supports,raw aliases,-R chart | Fixed controls retained; identity-R/arbitrary-sign query coverage remains open |
| Exact independent degree6 full-k direct reference | Not attempted; full-length degree1 and degree6 n<=31 references plus original full-k row replay are distinct scopes |
| Target-conditioned coefficient algorithm | Open; concrete unknown-coefficient compiler contract in NEXT.md |
| Full cold bitop/yield/family/selection/kernel/recovery | Open; partial diagnostics are not a performance result |
| Independent signed/formal native validation | Open; old residual identities/unit cancellation are partial |

[Inspected report](../research/curve25519_torus_native_20261115/reports/section-query-v1/REPORT.md)
and [next technical obligation](../research/curve25519_torus_native_20261115/section-query-v1/NEXT.md).
No solver campaign or paid run is scheduled. [pcc:unverified]
