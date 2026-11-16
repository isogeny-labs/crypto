# Development verification receipts

These receipts cover native correctness checks, not a performance experiment
or an independent metric/cost validation. No speed or scaling result is claimed.

Completed checks:

- `aps-tests.txt`: 34 passing APS library tests, including 14 new replay
  controls. Complete point-set comparisons, corrupted labels/metadata/signs,
  missing roots, duplicate keys, bounds and the B=0/correct-answer control pass.
- `example-tests.txt`: 10 passing fixture-runner tests. The matrix exercises
  all twelve arms in both enumeration modes; another test reads and replays
  all four shipped transcript/receipt pairs byte-for-byte.
- Strict final-source Rust 1.98 Clippy passes with library/test/example
  targets. Rust 1.99 strict Clippy passed before the final artifact-only test
  addition; final runtime example tests use Rust 1.99.1. Detailed dependency
  build output remains local under the prior publication restriction.
- Rust 1.98 changed-file formatting, actionlint, all four existing workflow
  YAML tests and all twelve CI-scope metadata tests pass. The earlier PR's
  CI-scope archive failure is fixed in this newer main baseline.
- The native executable generated the four fixed public controls and their
  independent replay receipts. The no-relation solve returns exit 1 as
  designed, retaining its JSON; standalone replay returns success with the
  explicit `no_informative_relation` outcome and no recovered logarithm.
- Local changed-blob size and artifact-policy checks pass. These do not
  substitute for the required hosted `size` and `artifacts` checks.

The full release library suite, compiled against initial main `91902d0` plus
the pinned task source, **passed: 5,220 passed, 0 failed, 123 ignored**.
`full-library-summary.txt` records the source boundary and actual exit 0.

After integrating published main `992c2579` (including its changed F4 visitor),
APS/F4/example checks passed **34/67/10** and strict Rust 1.98 lint passed.
`integration-summary.txt` records those separate results. The full run used
the earlier compiled binary and is not claimed as a new-main full suite.

`source-sha256.txt` preserves the earlier validated Rust/Cargo-lock snapshot.
`merge-source-sha256.txt` pins the current combination after inherited main
lock changes; the three Rust hashes remain identical. Commands are in
`commands.txt`. Published focused outputs omit build-directory metadata and
elapsed durations. Original outputs, including draft failures, remain local;
`original-log-sha256.txt` binds those unmodified files. These reduced receipts
are not a full raw archive or a cost/physical-execution attestation.

## Retained failures

1. Initial compilation exited 101: five tracked current-main compile-time
   fixtures were absent in the sparse checkout, and two fixture coordinates
   inferred an unsupported signed integer conversion to BigUint. No test ran.
   Annotated the fixed tuple as u64 and materialized the tracked inputs.
2. A retry started before the fixture restore finished and exited 101 on
   the same five absent inputs. It is not a completed test invocation.
   After verifying all five files present, the subsequent APS invocation
   passed all 34 tests without changing any test or assertion.

Raw draft logs are retained, not relabeled as passed or silently discarded.
No frozen input was fabricated, test disabled, rejected upload retried or
benchmark launched. The source-checkpoint and shared-file publication
restrictions remain unresolved.

An optional repository-wide artifact/workflow wiring diagnostic was stopped
while it attempted serial lazy hydration of off-disk manifests in this sparse
checkout. It has no pass verdict; the hosted artifacts workflow remains
required. Initial staging of the new study also needed Git's `--sparse`
option. Neither issue required a source, workflow condition or assertion change.

## Main-conflict follow-up

`merge-validation-summary.txt` records fresh 34/67/10 APS/F4/example passes,
strict Rust 1.98 lint, actionlint, four YAML checks and twelve CI-scope checks.
These ran on the main `78f84234` working merge. The subsequent main `8171fb53`
refresh changes other examples/study files; source/lock/APS-fixture/workflow
equality was checked. The old full-library receipt keeps its original scope.

Two local network-denied CI-scope invocations failed at the file-runner boundary
before reporting subtests; the unchanged suite passed all twelve with network
permission. Another read-only Node/Git metadata helper reported `spawnSync git
EPERM` in that restricted context. No assertion or script changed, and no base
code failure is claimed. Original outputs and hashes are retained.
