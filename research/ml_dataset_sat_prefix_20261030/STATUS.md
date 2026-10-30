# Causal SAT implementation status

Contractf71728b66558 was committed before source edits. The native opt-in gate
map API is implemented and source-reviewed; five meaningful native tests are
authored, with formatting/whitespace checks passing. No new-source compilation
or test execution has occurred. See encoder-source-review.json for exact hash.

Exporter integration for sat-features.jsonl and restricted/sat-records.jsonl,
strict allowlistv7, checked source inventories, joins and immutable artifacts is
implemented but uncompiled. The independent normalizer/replayer module remains
under implementation and review. Existing static/prime/algebraic input identity
must remain unchanged; runtime tests will verify that obligation.

Shared solvers.rs is still owned by T887. The reviewable proposal in
solvers-integration-proposed.patch is NOT applied. Coordination permission is
pending. Its map+formula aggregate budget is64MiB, with native events separately
capped16MiB; neither is a whole-process RSS or all-artifact size guarantee.

The actual PDP CLI fixture gate is still missing. Targeted retained inventories
contained only primeF4/BSGS fixtures. Existing planted SAT unit controls do not
prove nonzero decision rows, certified learning followed by another decision,
or nonzero twist coverage. A future explicitly bounded deterministic fixture
selection must satisfy these requirements; empty output is not a substitute.

Parent consequence validation remains active in the other worktree (handle24426,
full03). No competing builds have started. Sourcecheckpoint01 was freshly
restored and its Git bundle verified; it contains a committed design/handoff and
in-progress source, not a validated runtime. Verified cloud dataset cells0.
