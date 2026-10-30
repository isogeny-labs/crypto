# Causal SAT implementation status

Contractf71728b66558 was committed before source edits. The native opt-in gate
map API is implemented and source-reviewed; five meaningful native tests are
authored, with formatting/whitespace checks passing. No new-source compilation
or test execution has occurred. See encoder-source-review.json for exact hash.

Exporter integration for sat-features.jsonl and restricted/sat-records.jsonl,
strict allowlistv7, checked source inventories, joins and immutable artifacts is
implemented but uncompiled. The independent normalizer/replayer module is implemented and source-reviewed;
11 tests are authored, not executed. See replay-source-review.json for the
exact final hash and narrow review scope. Existing static/prime/algebraic input identity
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

Parent consequence full03 terminated with exit101: 5458 passed, 1 failed,
123 ignored. The one failure is a missing sparse-checkout archive manifest
directory, before its width rejection test can run. Both source checks passed.
Fixture-only correction is being prepared separately. No competing builds
have started. Sourcecheckpoint01 was freshly
restored and its Git bundle verified; it contains a committed design/handoff and
in-progress source, not a validated runtime. Verified cloud dataset cells0.

## First execution attempt

Standalone sourcef3747f693fc8 and validation freeze27f12537ea4a are committed.
SAT dataset phase01 ended06:53:21Z with exit101 before compilation: the sparse
checkout lacked tools/factor-base/Cargo.toml. Both source guards passed; the
151-entry fixture inventory did not include build dependencies previously
selected by sparse paths in the parent worktree. The failed receipt and log are
retained with fresh S3 readback. A complete local dependency-tree materialization
repair is in progress; no tests have run.

Independent semantic review found two additional corruption checks to tighten:
reject duplicate literals in exact original/learned Clause declarations so
arena IDs cannot drift, and reject a SAT endpoint when normalization retained
a sticky contradiction. The author is adding controls before a new source freeze.
Earlier review receipts remain point-in-time scoped reviews, not final certification.
Consequence continuation04 now owns the shared target; no SAT build can start
until it terminates.

Sourcecheckpoint03 was freshly downloaded and its bundle verified:
SHA2459dd5eb4f4422b478b834f4a4bdd56e1484a4bf92baa70c2228b62d9b7bbb5.
It captures27f12537ea4a before these review corrections and materialization repair.

Semantic corrections are now implemented and independently re-reviewed at
SHA19f180408a0e3b1114cb510386741d02d5ea951acc4e9f6b3fcc9a047f9c21db.
No outstanding blocker was found in that scoped correction review. There are
13 authored replay tests; none has executed. See replay-semantic-review-02.json.

Dependency restoration02 is complete and byte-verified:2722 files,253892127 bytes,
SHA d72f4ce44858dd813031eba098b618a0c02c58d2f521c655cfbbc647455eb004.
It covers eight recursive Cargo manifests, complete local dependency trees and
maintained targets. Every selected dependency blob is identical between source
27f12537ea4a and corrected runtime356e6af19ff7. No modified source was replaced.
The first pass retained its expected HEAD-change rejection; a separate parity
pass issued the immutable inventory. Dataset02 is prepared, not started.

The installed protocol skill's referenced official schema/checker is absent from
the inspected local repositories; PROTOCOL-TOOLING.md records the search scope.
CLI preparation drafts a substantive contract and separates freeze/commit from
execution. Official schema/export validation remains unavailable, not fabricated.
