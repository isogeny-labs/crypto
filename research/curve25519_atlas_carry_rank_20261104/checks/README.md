# Development checks, 2026-11-04

Scope: local functional checks. All outcomes below are producer observations,
not independent verdicts [pcc:unverified]. No clock comparison, normalized
cost, scaling law or scientific experiment is reported.

## Native checks

Command: `make test` from the study directory. Outputs:
`native-test.log` (earlier revision) and `native-test-final.log` (current
development revision). See `SOURCE.sha256` for source identity and
`tool-versions.txt` for GCC, GMP and native Z3 versions.

The RFC fixture checks the source-declared generator/subgroup and finite
2-isogeny transport cases with GMP; it is not an independent point-count
certificate. Small-curve checks exercise the entire homomorphism and dual
tables at p=101 and 211, and inverse transport at p=1019. Carry witnesses
cover every product at p=7,31,101 and radix=2,4,16. Rank tests distinguish
dependent and independent coefficient rows. Replay tests include a valid
native model and malformed/unknown/UNSAT rejection. Tests report success;
this is a self-check, not a proof of the general construction [pcc:unverified].

## Isolated carry encoding checks

```
bash test-products.sh /tmp/curve25519-atlas-carry-rank \
  /tmp/curve25519-dev-tools/bin/z3 /tmp/curve25519-product-check-final
```

At p=7 and radix=2, all 49 input pairs have one correct and one deliberately
incorrect residue, each encoded both ways: 196 expected SAT/UNSAT answers.
`product-test-final.log` records completion; `products.tsv` retains each
expected and actual solver answer. The test checks these finite systems;
it does not establish unrestricted relation-encoding performance or a
universal equivalence theorem [pcc:unverified].

## Full relation queries and pinned witnesses

All fresh directories below were local development fixtures, not remote
experiment runs. Every invocation used native Z3 5.3.3 and `-T:30`.

| Attempt | Fixture | Outcome | Retained input/output |
| --- | --- | --- | --- |
| 1 | p=1019, unpinned source UNSAT | timeout; subsequent cases not attempted | attempt-1/ |
| 2 | p=101, unpinned source UNSAT | timeout; subsequent cases not attempted | attempt-2/ |
| 3 | p=101, redundant curve equations removed | unpinned source UNSAT timeout | attempt-3/ |
| 4 | added pinned witnesses before unrestricted cases | source pinned direct/carry SAT replayed; unrestricted source UNSAT timeout | attempt-4/ |
| 5 | earlier fail-fast pinned-only script | stopped at a timeout; not the complete pinned panel | pinned-test.log |
| 6 | complete pinned-only panel, retaining every case | 9 SAT models replayed; mixed carry timeout | pinned-summary.tsv and pinned-models/ |

The product checks isolate multiplication; pinned relation queries isolate
known-witness constraint checking. Neither replaces unpinned decomposition.
Timeout is not UNSAT. The full-query integration test remains unresolved.
No timed winner is selected and no failed case is excluded from a result.
These are producer observations [pcc:unverified].

Reproduction, with a fresh output directory:

```
bash test-encodings.sh /tmp/curve25519-atlas-carry-rank /path/to/z3 /tmp/new-panel pinned-only
bash test-encodings.sh /tmp/curve25519-atlas-carry-rank /path/to/z3 /tmp/new-full-panel
```

The final runner retains a summary for every attempted case and returns
nonzero if any outcome differs from its native expected answer, times out,
or fails witness replay. The full scope is left intact; pinned-only is an
explicit additional diagnostic. The general `--replay` command accepts the
small-prime run's model/base/target and checks membership, subgroup and sum;
its external model syntax is the native Z3 fixture syntax, not every solver.

## Infrastructure and whole-library checks

`aws-identity.log`: `aws sts get-caller-identity --region us-east-2` has no
credentials. `checkpoint-push.log` and `checkpoint-gate.log`: no AWS upload
profile or checkpoint token, so the study was not checkpointed. No paid job
was submitted [pcc:unverified].

An initial `cargo test --release --lib` could not start because Cargo was
absent. Installed stable Rust 1.99.1 with the official installer. Sparse
dependency misses are retained in `cargo-release-lib.log` and
`cargo-release-lib-2.log`; the subsequent whole-library attempt is retained
as `cargo-release-lib-3.log`. Its final state is recorded in STATUS.md.
These sparse-setup errors are not failures of the mathematical hypotheses.

There are no touched Rust examples and no added Python research code.
The installed Python wheels only supplied native Z3 and the AWS CLI; all
cryptographic arithmetic, encoding, rank and witness checks are compiled C++.
