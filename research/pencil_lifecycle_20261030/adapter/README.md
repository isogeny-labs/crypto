# Optional portable pencil adapter

T-953 source preparation only. No build, solver test, experiment, plan generation or job. Original research sources remain unchanged. Scope: existing ecbench/pencil.rs and two new sibling source files.

The nested ecbench::three_leads::pencil adapter now exposes solve_with_progress(..., Option<&mut dyn progress::Sink>). Existing solve delegates with None and calls the original native solver. Some(sink) calls a portable copy of the previously reviewed diagnostic fork. No environment variable, TLS lookup, implicit sink creation or runtime dispatch is introduced. Root integration must supply an explicitly validated active context; passing a sink alone is not admission.

Shared aliases remain root engine and measurement plus the existing single pencil_symbolic_graph kernel. The copied native fork changes ONLY crate::progress:: to super::progress::. The sink is byte-identical to reviewed source5415467. No producer-feature dependency or separate engine crate is added.

NativeResult owns either complete native PencilResult. Accessors borrow setup/detail and return scalar flags; mem::take moves the JSON fields into the report without cloning or prematurely dropping private LinearSolver/GraphSolver owners. The enum remains in scope through additional source verification, calibration, geometry accounting and replay digests. Both branches use the same existing report construction and cost boundaries. Runtime-lifecycle overhead remains a separate enclosing obligation, not part of accepted mathematical results.

Source-reference review: saved all-arm-lifecycle-and-cost-patch-spec.md; existing pencil.rs owner lifetime and native PencilResult private fields; retained progress source/read-only independent reviews. No new mathematical formulas or pricing weights.

Mechanical source parity checked without execution: git show HEAD:research/pencil_progress_20261030/src/native_solver.rs with exact namespace substitution SHA25625649c210fdc9a3c5d4f2c136c4adc3693b7dfcdc27368eafe26b06edccb5c12 equals copied file. Sink exact SHA2565415467accaf0e8b6426a8834838ac6fc7e7a463520b02dfcbc8eb7d1338cd89. This is source parity, not empirical deterministic parity or successful compilation.

Root must register/build/lint/test the final integrated tree. Copied source contains historical solver tests; do not execute those under diagnostic-fixture authorization. Only previously defined diagnostic fixtures or new fake-source/ledger fixtures are within this task. Full instrument/source/protocol binding and scientific admission remain pending. Planning expiry22:08:40 UTC unchanged.

## Explicit runtime dispatch increment

T953 additionally reserves ecbench/three_leads.rs. `three_leads::ScopedPencilProgress::install(path, diagnostics::Identity)` creates a private unique prefix sink after exact identity mapping. Caller must validate the candidate ChildEnvelope first and provide a runner-owned path. No activation is inferred from environment. Same-thread ownership and non-Send guard prevent cross-thread sink access; nested install and callback reentrancy fail. Drop invalidates ownership with no IO; explicit finish returns separate IO totals. Panic cleanup does not synthesize final evidence.

The actual pencil method branch now uses the optional adapter only while this explicit scope is active; absent scope calls the original adapter. All rho branches are unchanged. Lifecycle remains separately common to all arms and is not this candidate-specific prefix writer. Two new fixture sources use fake callbacks only, check ownership/reentrancy/panic cleanup, and tick the shared kernel semantic meter without group arithmetic. They have NOT been executed here. Original source.sha256 is retained; source-dispatch.sha256 records the increment.
