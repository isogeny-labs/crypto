# Independent review: bounded unknown-runtime characterization gate

Reviewed at `2026-10-30T05:36:22Z`. This is a read-only review of the narrow native admission exception for the exact `p186379` target-0 characterization gate. No solver, trial, replay, cloud job, or scientific workload was run for this review.

## Result

No remaining concrete defect was found in the scoped source admission path. The reviewed exception is suitable for validating a corrected protocol only when that protocol and every bound attachment exactly satisfy the reviewed opt-in.

The exception records runtime as unknown. It does not relabel the 10,800- or 21,600-second resource ceiling as an estimated runtime. Ordinary frozen protocols with a null runtime estimate still fail admission.

## Reviewed source

| File | SHA-256 |
|---|---|
| `native_canonical_export/src/protocol_lint.rs` | `acf731fd71387496a2f2d4605c1bc87a76abbc8f4fdadf44f4e87cd5d81d7613` |
| `native_canonical_export/src/lib.rs` | `67bd9a11ede14a4e9addfe8855fa85234810fa8a5ff9a6a13b9fec12be7b5e53` |
| `native_canonical_export/tests/unit_contracts/mod.rs` | `2aece5a99abafc1ae1fdc1956b2e42f6b194b56e613274edfe46daeb969a3620` |
| `native_canonical_export/tests/unit_contracts/unknown_runtime.rs` | `2a3081078f2b189120ae43783f77d48abeecb7c4ecaa33f39002d6951dff0f9c` |

The files were uncommitted at review time; these byte hashes are authoritative. The repository HEAD used as context was `9ab05b985d13489ffc55338934a2f7cb858a2b04`.

## Admission properties checked

- The exception requires the exact schema `pencil-bounded-unknown-runtime-gate/v1`, explicit `runtime_estimate_status: unknown`, and an assertion that resource caps are not runtime estimates.
- Scope is fixed at one workload, three named arms, three warmups, fifteen measured executions, eighteen total executions, five measured pairs, and thirty-six finite-cost cells.
- The sole primary statistic is exact finite algorithm GAE. Every clock remains unqualified and diagnostic.
- The prepared specification, plan, source, binary, runtime image, arm roles, pairings, flags, environment, method parameters, finite-cost policy, grids, and decision statistic are checked.
- Official protocol attachments and the exception's bindings must agree; synchronized mutations of both copies are still rejected by semantic body checks.
- The prior `b1` run identity, terminal-state digest, failure reason, two observed warmups, zero measured rows, and sixteen missing rows are fixed.
- The amendment is prospective, permits one replacement only, excludes prior rows from the replacement, and forbids early-rejection integration and automatic scientific retry.
- The resource authorization remains resource-only: it cannot authorize scientific replacement, the remaining cohort, a timing design, or launch readiness.
- Retry, checkpoint, restore, memory, execution, cumulative-resource, absolute-date, and single-payload restrictions are checked.
- A complete gate, all fifteen measured independent non-wall replays, independent review, and a separate prospective cohort admission remain prerequisites. Scientific admission and launch authorization remain false.

## Validation evidence

| Check | Evidence | Result |
|---|---|---|
| Release library tests | `native_canonical_export/validation/unknown-runtime-gate/release-lib-tests.log`, SHA-256 `41368304046684f0c8cbedb37b5830d30e75b883b46e3a0263179fb8b9c88af9` | 59 passed, 0 failed, 0 ignored |
| Strict all-target Clippy | `native_canonical_export/validation/unknown-runtime-gate/clippy.log`, SHA-256 `b03766b32a1e902cc9a355c64e48508d44489362e09f5981b7fc7679500f7adf` | Passed |
| Rustfmt | `native_canonical_export/validation/unknown-runtime-gate/fmt.log`, SHA-256 `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855` | Passed |
| Diff whitespace check | `git diff --check` | Passed |
| Build receipt | `native_canonical_export/validation/unknown-runtime-gate/build-receipt.json`, SHA-256 `2880aa1f81c859941caab3de9cdba82725e5ddb48b4f5dc48d17ab463ccda4a5` | Bound |
| Release binary | `/tmp/pencil-unknown-runtime-target/release/pencil-native-canonical-export` | SHA-256 `b904a0da30851284da6301d4c97117cf427d6e3d32ea0ff242d90a928367bf7e`, 7,673,312 bytes |

The adapter digest recorded by the build receipt is `5d14e1a9ea20e56c016cf80a12c85ac37d6b4653a6caf53530da69f9a78bc4e3`.

## Earlier review status

The earlier `pencil-six-hour-design-review.{json,md}` remains unchanged historical design evidence. It is superseded only for the freeze-suitability decision because it predates both the observed frozen-validation failure (`frozen protocol preflight estimate absent`) and the reviewed narrow source exception. Its hashes remain bound in the companion JSON review.

## Limits

This review supplies no runtime estimate, timing qualification, scientific result, cohort admission, launch authorization, or live infrastructure guarantee. It does not reuse the failed `b1` rows as prospective data. A later change to the source, protocol, or any bound attachment requires another review.
