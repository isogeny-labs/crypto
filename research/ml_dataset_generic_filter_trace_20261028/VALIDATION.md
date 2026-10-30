# Completed software validation

Runtime source `58bd5ab64424f7710540e0aea502225de6e6c438` passed the
source-bound release gates on 2026-10-28. This supersedes the pending status in
the historical implementation checkpoint; its initial failures remain retained.

| Gate | Result |
| --- | --- |
| Dataset tests | 165 passed, 0 failed, 1 existing ignored |
| Native filter tests | 13 passed, 0 failed |
| CLI integration | 8 passed, 0 failed |
| Strict release Clippy, library/tests/crypto binary | exit 0 |
| Full library | 5198 passed, 0 failed, 123 ignored, exit 0 |

The immutable full-library executable SHA-256 is
`170b963e08d8570ff1efdb175390b151c40d9d2d9156524366923d8090efdece`.
The runtime was compiled from a clean source identity. The delivery merge
`1e8096394fdd51a2e33fd47b6b21ef47d848602f` adds the parent PR's documentation;
its runtime, build and test paths are byte-equivalent to the tested source.
Later changes in this directory are validation documentation only.

The independent source review found no actionable issue in the inspected
binding, cumulative admission, prefix, payload-size and replay paths. This
source review is separate from executing an independent consumer: generic-v2
support in that consumer is still pending.

## Durable evidence

`source-archive.json` identifies the immutable source, binaries, focused logs,
initial failures and source review archive (48,958,863 bytes). Its original
pending-full-gate status is historical. `full-gate-archive.json` identifies the
completed full log and source/binary-bound validation (89,380 bytes), uploaded
and verified at 17:39:09 UTC. Both use the canonical bucket
`setec-astronomy-artifacts` in `us-east-2`, with conditional PUT, fresh
ETag-bound GET, byte comparison and contained-file hashes checked.

The full-gate object is:

`s3://setec-astronomy-artifacts/ml-dataset/20261028/software/generic-filter-trace-full/9b103b7264a1960dd81d7a819c0422f9c89378cd3dc69cb788bf2f8dc0a6ad3a/validation.tar.gz`

Its SHA-256 equals the content-addressed directory name. These are software
validation artifacts. No new scientific corpus cells, cloud jobs, PMU totals,
first/last-fall-degree result or complete-execution proof is claimed.

## Behavior and limits

Generic-v2 cells bind cumulative native batch traces to the public instance,
configuration, source relations and exact seeded query replay. Native event,
byte and named logical-work allowances do not reset between batches. Complete
batches retain restricted certificates; a censored batch cannot supply a final
rank or recovery certificate. Compact v2 serialization respects the unchanged
32 MiB reader bound. Oversize output fails while retaining the captured prefix.

Legacy v1 replay and Koblitz behavior remain supported. Static public feature
values exclude trace, outcome, witness and provenance data. The trace checks
supplied native operations and named accounting; it does not prove total
machine cost, exhaustive relation search or a complete dependency basis.

Current-head hosted CI and review are separate from these local gates. The PR
is stacked on the native trace parent PR2790; main-branch delivery remains
dependent on the existing producer stack. The original USD200 total ceiling,
worker-publication permission gate and durable quota requirement are unchanged.
