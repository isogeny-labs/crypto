# Additive child/runner/dispatch source review

Read-only review on 2026-10-30. No builds, tests, workload generation, solvers, cloud calls or Git changes. This supplements rather than replaces lifecycle-launch-runtime-source-review.md.

Observed SHA-256:
- diagnostic_launch.rs e111ff117e23224b371541f0df75aa5e3c99885c70af495076a1a1f26b9cd714
- record.rs ff3dc59607925b9baf99dd489084de475f9c0f96ef171c51e4a00f404c2365ee
- runner.rs 73c20bae3415b0ca741ecd519552e0d29cbc2bea67efa1560256bed97849cd84
- pencil.rs 218a93151c984b1e527abef5532dfcecd06315cf4e0631ef601d5118145a35b2
- three_leads.rs b56b4c0139ff5c1eb3e1066f2858c83bdfd65da5efc90c75772f06443a40a888

## Explicit activation blocker: full workload binding remains outside helper

record.rs DiagnosticChild::validate checks schema, exact transported diagnostic object, launch/envelope, resolved method, method ID and algorithm seed, but not curve, target_seed, target_index, target_kind or expected_workload_id against prefix.identity.workload_sha256. Mutating those workload fields leaves this metadata helper's checks unchanged. The generic callback helper therefore does not establish workload correspondence by itself. This does not expose a presently enabled scientific path: child_main explicitly rejects diagnostics before workload construction, and runner's pre-plan gate also refuses activation.

Required before activation: retain the existing Workload::build and expected_workload_id check, recompute the SHA-256 of the full rebuilt Workload serialized through JSON Value exactly as prepare_lifecycle_child does, and reject a mismatch before the solver callback. Do not require workload construction in the pre-plan metadata gate. Parent scheduling must still associate session/sequence/arm with its exact planned execution; prepare_lifecycle_child currently receives those from the actual indexed loop.

## Other inspected behavior

Diagnostic root validation requires bounded absolute canonical paths, rejects symlink components and traversal, enforces owner0700 and protects against nonsticky writable ancestors. Same-UID executable/input replacement remains explicitly trusted. Runner creates a separate private root and passes it explicitly. Exclusive lifecycle/prefix leaf creation prevents overwriting prior observations.

Method configuration uses the shared full ResolvedMethod JSON Value digest. Runner's internal setup checks actual capsule binary hash before resolved plan binding. Current pre-plan diagnostic branch deliberately passes no binary hash and refuses; it cannot be cited as successful activation or validation of a usable launch.

The timeout remains measurement.timeout_seconds.max(1), passed unchanged to run_child. No diagnostic-derived longer timeout is introduced by the inspected wiring.

ScopedPencilProgress uses try_borrow_mut, checks token against independent owner Cell, and clears ownership on Drop without IO. Nested dispatch fails with a Result rather than RefCell panic. A guard dropped during a borrow invalidates ownership immediately; stale storage is reclaimed only after borrow ends. The dispatch branch is pencil-only; absent scope uses the original solver. Generic lifecycle callback setup is common across all three arms, candidate prefix only. Error and panic handling do not manufacture scientific completion; returned lifecycle/prefix writer data are separate diagnostic summaries.

No additional concrete source defect identified within the explicit metadata-only, disabled-child boundary. This is not end-to-end qualification, compilation evidence, timeout measurement or full accounting validation. Donor producer fidelity is not reproved here.
