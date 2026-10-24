# Validation record

Current actual-system continuation: [s3-yield-20261024](s3-yield-20261024/README.md).
The unmodified twelve-module gate passes144 named research theorems /185
audited declarations, zero failing/extra trusted. Exact native replay checks124
base comparisons /1,904 systems, two-way coefficient/degree transport controls,
all eleven independently solved F43 targets, two source-bound certificates and
four rehashed semantic-corruption controls. Backend22 and metadata4 tests pass.
The fresh full release library suite passes4,479 with119 existing ignores after
granting loopback network access; the initial nine socket-denied failures remain
recorded. No source/test patch or weakened gate was used. Earlier receipts below
are historical and remain unchanged.

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

The [2026-10-23 group-map/eigenspace continuation](group-map-20261023/README.md)
passes the unmodified proof gate with 49 named theorems / 57 audited declarations,
zero failing/extra trusted, and a fresh 4,479-pass library suite (119 existing
ignores). Actual addition/scalars, source annihilators, the full explicit
short-model minus-group isomorphism and rational fixed eigenspace are checked.
Four new source-bound polynomial certificates, exact bytes, corruption controls
and 22 isolated native backend tests pass. The PR-caused stale Jacobian catalog
is repaired natively, preserving all 2,572 prior objects; the strict cover
assessment's source bindings are refreshed without changing any non-source
semantic section or existing transfer status. Raw failures and all earlier
reports remain unchanged. General normalization, exceptional lifting, pairing,
trace-zero and shared-Conductor gates stay open.

The [2026-10-24 pairing continuation](pairing-20261024T0442Z/README.md) passes
the unchanged seven-module proof gate with 68 named theorems / 85 audited
declarations, zero failing/extra trusted. It checks the actual F43 twist source,
Frobenius signs, explicit Miller function, all 121 generator pairs and exact
finite logarithm transport. A full native sweep of 942 models and 4,616 auxiliary
points passes independent coefficient-pair/linear-Miller replay, exact frozen
bytes, four corrupted-payload controls and the new norm-product certificate.
All eleven fixture IC certificates and their six-dimensional ranks pass the
independent auditor; the IC algorithm is not formalized. The fresh full library
suite passes 4,479 with 119 existing ignores; 22 backend and four workflow tests
pass. Earlier failed attempts and all frozen sources remain unchanged. General
geometric pairing theory and practical advantage remain distinct open questions.

The [relation-yield continuation](yield-20261024/README.md) passes the unchanged
ten-module proof gate:106 named research theorems /127 audited declarations,
zero failing/extra trusted. Matched actual twist factor bases preserve the
accepted coefficient set, useful yield under the same predicate, row span and
rank. Whole-group raw rates are inverse group orders, with exact twist/main
ratio N/Nt. The useful-yield/degree cost and overhead theorems are explicitly
conditional. The F5 raw-yield control and synthetic budget controls are kernel
checked. A new source-bound cleared-budget polynomial certificate, exact byte
replay, false identities/source bindings and22 native backend tests pass.
All failed elaborations and sparse-checkout validation failures are retained;
no frozen record or actual speed claim is rewritten.
