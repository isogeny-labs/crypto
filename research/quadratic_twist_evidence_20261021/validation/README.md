# Validation record

The original import's `tooling.log` and native audit receipts remain unchanged.
This continuation installed Rust 1.99.1 and Lean 4.34.1 and ran the previously
unavailable checks. Earlier missing-tool statements describe the earlier environment.

| Check | Actual result | Evidence |
| --- | --- | --- |
| Full native replay | passed; all 434 deterministic fields match | `../native/runs/run-20261021T150125Z/` |
| Independent IC verifier | 3 headline plus 81 complete-log certificates passed | same run's `independent-ic-audit.json` |
| Tower/simple basis | 2,401 elements, 2,475 points, 2,025 group-map checks passed | same run's `basis.json` |
| False new relation / missing coverage / false census count | rejected as expected | `../native/controls-20261021T150300Z/` |
| Metadata source binding, byte replay and selected registry representations | passed; changed native input rejected before emitting metadata | `continuation/metadata-byte-controls.log`, `metadata-source-binding-negative.log` |
| Native metadata/backend tests | 39 passed | `continuation/metadata-tests.log` |
| Required `cargo test --release --lib --locked`, final registry/code | 4,474 passed; 119 pre-existing ignores; no failures | `continuation/cargo-lib-final.log.xz`, exit code 0 |
| Per-model extension registry test | passed for all 2,287 extension models | `continuation/registry-test.log` |
| First Lean group/ring proof gate | 14 theorems, zero extra trusted axioms | `lean-gate/` |
| Latest Lean gate including actual Frobenius point maps | 18 theorems, zero extra trusted axioms | `lean-bridge-gate/` |
| Additional `lean4checker --fresh` | unavailable | not installed; no external recheck claimed |
| Shared Conductor check | unavailable; exit 127 | `continuation/conductor.log`, `conductor.exit` |
| Scoped workflow and shell sources | shell syntax checked; expanded native CI supplied | `.github/workflows/quadratic-twist-evidence.yml`, three thin shell scripts |
| Legacy registry metadata adapter | syntax compiled; no Python arithmetic path executed | metadata-only `harvest_native_quadratic_twists`; native Rust generated the records |
| New PDF | compiled and every page visually inspected | `../reports/native-20261021T150125Z/`, `continuation/pdf-build.log` |
| Expanded cover catalog integration | 4,493 native geometry checks; all prior records retained; independent assessment and missing-membership control pass; 26 affected binary tests pass | `catalog-ci-20261022/README.md`, `../reports/catalog-20261022/` |
| Standards/browser metadata follow-up | all 4,493 rows reproduce; 40 native, 4,474 library and 26 binary tests pass; six metadata tests pass; frozen registry/board/browser values preserved; independent assessment has zero findings | `metadata-ci-20261022T0500Z/README.md`, `../reports/metadata-20261022T0500Z/` |

The full-library run requires permission for its existing localhost socket tests.
A sandbox-restricted invocation failed nine networking tests with `PermissionDenied`.
The complete suite was rerun with networking enabled and passed; no test was
skipped or disabled to obtain that pass. The 119 ignored tests are the baseline's
existing ignores. The only library-test source change updates the exact extension
model count from 1,324 to 2,287 after 963 models were added, preserving every
per-model field, order and generator assertion. Its failed prior full-suite log
and the successful focused and full reruns are retained.

Compressed logs use xz; read them with `xz -dc FILE.log.xz`. They retain failed
runs too: an initial sparse checkout lacked the tracked `isogeny_algos/` dependency;
the first post-registration suite hit the old fixed count; a later run lacked
socket permission. No unrelated production code was patched around those failures.

The first native parity comparator rejected two JSON nulls with different internal
text spellings. Its exact source, reconstructed original header (matching the
recorded hash), failure and input manifest remain under `native/runs/port-attempt-1/`.
The first proof elaboration and two Frobenius bridge attempts remain in their
versioned validation directories, with failed source kept as `.lean.txt` so it is
not represented as a proof target. Failed Lean cache setup, a control CLI argument
mistake before it could run a workload, and the first PDF build are also retained.
The accepted current receipts, not provisional failed-build axiom printouts,
establish the reported checks.

`lean-gate/inputs.sha256` records the original 14-theorem Lake file; its exact
bytes are retained as `lakefile-source.toml`. The latest gate's manifest binds
both theorem modules and the current Lake configuration. Toolchains and Mathlib
revisions are pinned. Source and input hashes distinguish a changed uncommitted
worktree from the pre-existing Git head printed in the execution logs.

The three successful full native runs and failed first port attempt each have a
run-report directory sharing a source-linked series report, quantitative graph
and inspected PDF. Control directory names are stable run IDs; actual UTC
boundaries are those printed in their transcripts. Research records remain loose
where replay, source, digest, report and review links name their individual files.

The old inherited `validation-gate` archive-membership failure was superseded by
the refreshed base. At head `4711f1c73887`, the expanded native parity/audit,
validation-gate, and perf-history workflows passed. The cover workflow found the
catalog consistency failures documented and reproduced in `catalog-ci-20261022/`.
That follow-up reruns the complete library suite (4,474 passed, 119 existing
ignores), 26 affected binary tests, native catalog checks and independent metadata
validation. The old benchmark/gate files remain outside this change.
CI must still complete successfully for the resulting head before any merge.
Shared Conductor access and the exact remaining geometric proof obligations stay
open in the problem record.

At head `3e442fd3310d`, the full native audit and quadratic-twist Lean target
passed CI. The next standards metadata join found compact imported rows; the
fresh-import reproduction and layout correction are retained in
`metadata-ci-20261022T0500Z/`. Native generation now matches the canonical registry
byte for byte. Model, geometry, identity, alias and measured-board values are
unchanged. The current assessment retains the three original Git blobs at
`a11984a57344` and pins corrected inputs at descendant `228cdc0d50f2`.

The same exact-head Lean run failed in the unrelated SEA target after a
successful build because Lean 4.19 rejected `--trusted` before axiom auditing.
Its source directory, gate and workflow are identical at the declared PR base,
CI base, PR head and tested merge. That inherited failure is recorded without
modifying its sources or the gate. Cursor's approval agent could not start due
to its usage limit; a neutral result supplies no approval. Full exact-head check
runs, failed receipts and source comparisons are retained in the new record.
