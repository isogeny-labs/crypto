# Full-D18 phase-first public natural screen

Freeze this L0 screening pair before either retained arm. The prior
[phase-first planted control](phase-first-20261025/RESULT.md) passed with all
32,643 selectors and all 82 Frobenius phases per summand admissible. This
extension uses the same shared-root-bit circuit and search policy on the
primary Koblitz `a=0` GF(2^83) curve, the complete S3-custodied D18 base,
`m=4`, identity mask zero, 100,000 conflicts and the public fixture
`(a=0, fixture=0)` from `pilot-01/probe-corpus.json` (raw SHA-256
`760ae5c2e88615152e2569653a0e0c0cf3b2f84691f2b0731247f9ee3759dcbd`).
The public target point is
`(1834913121178197242856238, 8797964190732846213981377)`. The worker
uses its point, never its known-answer scalar.

After committing and testing this opt-in mode, build one executable from the
clean source and freshly fetch the exact S3 object and hash-bound manifest.
Require compressed SHA-256, provider checksum, byte length, producer-manifest
binding and the importer's full orbit/row replay. Use separate new output
directories, but the same executable and S3 input, on the same L0 Mac host.
Run serially:

1. `primary-full-d18-m4-l0-shared-phase-first BASE_DIR FIXTURE_DIR NEW_JSON`.
   Require `PASS_CONSTRUCTED_CONTROL`, original-field model and target checks,
   row digest, changed-target rejection and rank one before interpreting the
   natural arm.
2. `primary-full-d18-m4-l0-shared-phase-first-natural BASE_DIR FIXTURE_DIR
   NEW_JSON`. This arm has no summand or intermediate witness pins. Retain
   exactly one search on the public target.

Each worker has a 300-second internal whole-process cap and independent
310-second supervisor cap. Retain source and binary hashes, fresh S3 GET
metadata, full import/model/search/process charges, UTC intervals, raw
stdout/stderr, exit codes, phase/shape/result or cap files, RSS and all raw
file SHA-256 hashes. L0 is a shared host with no hard memory cgroup: report
that isolation honestly. A cap, incomplete model, group rejection, invalid
receipt or supervisor failure is censored/invalid, never an observed zero
yield. If a natural SAT result passes the worker's group and rank checks,
keep `PENDING_INDEPENDENT_NATURAL_REPLAY` until a separately implemented
original-field replay checks its exact stored points, signed-orbit row,
target sum and tamper rejection. One natural query cannot estimate a useful
relation rate or select a factor base. Full rank growth, factor-base logs,
unseen-target descent, complete cold one-target accounting and matched rho
remain required. `selected_best_total_runtime` stays `null`.
