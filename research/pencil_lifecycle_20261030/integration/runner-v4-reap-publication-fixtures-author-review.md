# Runner-v4 reap and publication software fixture proposal

Private, unapplied proposal prepared 2026-10-30. No repository edits, builds, tests, child processes, solvers, or jobs were executed. Rustfmt parsed the private after-file and emitted a separate formatted copy successfully. The patch is intentionally against the exact original private runner-v4.rs, without unrelated formatting changes; root's ChildOutput None literal additions remain independent.

Patch: `runner-v4-reap-publication-fixtures.patch`
SHA-256: `5889cbd8881ca47436e4f23a8cf95be6d02bc8b9f9835a444f2874a725bbbae0`.
Exact base hash/path: `runner-v4-fixture-proposal/base.json`.
Before/after source copies: `runner-v4-fixture-proposal/`.

## Shared production seam

The small private `ReapObservation` wrapper derives its decision from the explicit `Reaped.reap_observed` field, not merely Result::Ok. Its `mark_resources` invokes the existing real ResourceTracking observer only after an observed reap; otherwise it preserves the existing stalled reason. Its generic `when_observed` gates the production ingest_lifecycle_v2 callback. The production source uses this shared gate for both ChildReaped marking and sidecar ingestion, so tests do not reproduce a separate boolean decision that could drift from production.

No admission gate, mathematical result filter, solver path, timing boundary, or output schema is changed.

## Three new software fixtures

1. `diagnostic_reap_publication_regression_tests::ok_without_observed_reap_cannot_mark_or_read_sidecars`: construct Result::Ok with reap_observed=false and true. Exercise the shared gate, actual ResourceTracking, and a counted filesystem-read callback. False produces no read and no ChildReaped; true permits both. Publish the resulting real resource prefix and check the serialized marks. Both incomplete histories remain Failed, with pending closure and unknown elapsed/active totals.

2. `diagnostic_reap_publication_regression_tests::publication_collision_preserves_previous_bytes_and_pending_failure`: inject real exclusive-publication failures by precreating the resource receipt or tail destination. Check previous destination and records bytes remain unchanged. Prefix-stage failure leaves the journal unchanged; tail-stage failure retains the newly published prefix and its explicit injected execution-failure status, while keeping the closure pending and both aggregate totals null. This tests resource status, not the complete Session error handler. It intentionally preserves native partial files rather than removing or rewriting evidence.

3. `robustness_tests::diagnostic_reap_gate_observes_real_shell_child_without_algorithm`: reuse the existing shell-fixture machinery, invoking run_child_observed on a script which only drains stdin and exits. Assert exactly one spawn observation and a true reap observation. A missing executable produces no spawn callback and no reap gate admission. The fixture follows the existing bounded ETXTBSY retry convention. It does not invoke ecbench child_main, parse the mathematical input, construct a workload, or call any solver.

## Validation boundary

These fixtures are written, not run. They include no measured algorithm comparison, cohort generation, or scientific admission. Proposed named filters after root applies the patch:

- `diagnostic_reap_publication_regression_tests` (two tests)
- `diagnostic_reap_gate_observes_real_shell_child_without_algorithm` (one software process fixture)

The shell fixture may spawn /bin/sh and cat when root runs it. Its 30-second watchdog is a software failure bound, not scientific compute authorization. Root retains source ownership and controls compilation and execution of these fixtures.
