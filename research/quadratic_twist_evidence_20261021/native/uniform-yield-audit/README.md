# Independent uniform-yield auditor: compiled, not executed

This version is prepared for `uniform-yield-v2.cpp` under the frozen
`UNIFORM_YIELD_20261025_PROTOCOL.md` and the compile-only implementation addendum.
No producer, audit, recurrence, corruption control or new proof has run on the
study inputs. Checkpoint-only upload authorization/token access remain absent.
Do not invoke these experiment paths until the unmodified mandatory checkpoint
gate succeeds under explicit authorization.

The field/group/polynomial and semantic-witness prefix is derived from the
previous independent Rust auditor, not the C++ producer. New exact fractions
use arbitrary-precision Rust BigInt with a separately implemented Euclidean
gcd normalization. Reachable subspaces are enumerated breadth first and solved
in descending rank, separately from the C++ recursive GMP implementation.
The code is intended to reconstruct all responses/solver witnesses, actual
base logs/source scalars, reachable state transitions/rationals, policy
comparisons/frontiers and exact file/manifest/source membership.

Actual compiler-only commands:

```
cargo build --release --locked --offline --manifest-path research/quadratic_twist_evidence_20261021/native/uniform-yield-audit/Cargo.toml
cargo test --release --locked --offline --no-run --manifest-path research/quadratic_twist_evidence_20261021/native/uniform-yield-audit/Cargo.toml
```

The test harness compiled; its tests were NOT executed. Type checking is not
mathematical replay or validation of the recurrences, roots, ranks or scalars.

Planned CLI, NOT run:

```
uniform-auditor audit ROOT CASE_DIRECTORY FRESH_RECEIPT
uniform-auditor corrupt KIND CASE_DIRECTORY
```

Corruption modes implemented but untested: false-row, false-degree,
false-witness, missing-arm, missing-target, false-transition, false-rational,
missing-query, false-log, false-scalar, false-source, false-frontier and
missing-failed-query. Each modified case is rehashed; a missing-failed-query
control returns an explicit null if no such query exists. The intended auditor
writes its fresh receipt only after all expected payloads and index checks.
No rejection count or independent-verification result is claimed.

The faithful Lean recurrence/probability interpretation and exact study runner
remain unimplemented. All prior science/proofs/17identity records are unchanged.
See the dated `reports/uniform-audit-preflight-20261026/` and matching validation
record for exact hashes, compiler logs, mandatory ordinary library checks and
new required-check evidence. No scientific result, speedup, cost ranking,
intrinsic regularity, GHS or general-hardness claim follows from this code.
