# Final validation, 2026-10-26

- **Shared solver/library:** post-change `cargo +1.90.0 test --release --locked --offline --lib`
  passes **4,494**, with 119 existing ignored tests and no failures. Socket
  access was enabled. Exact logs/exit: `checks/post-linear-root-library.*`.
- **Experiment package:** every release test target passes after the solver
  change (`checks/post-linear-runner-tests.*`); the main runner has 116 tests.
  The separate opt-in `F4_FP_LINEAR_ROOTS=1` F4 suite passes all 21 tests
  (`checks/linear-policy-f4-tests.*`). Two new tests check exhaustive linear
  coefficients and recursive/inconsistent differential answers without
  mutating the test-process environment.
- **Scientific admission:** the seven audits linked in README pass; separate
  native raw-counter, finite-output and PARI gates remain distinct. The box
  audit retains 146 cooperative-budget failures. Control aggregate v1 is
  superseded by v2 without changing raw runs. No failed case is a zero-cost success.
- **Formal proof:** five classical F3 theorems pass the recorded Lean build
  and freshly compiled native axiom audit, with zero failing/trusted proofs.
  IDC1hfab3acdc297476e5 passes issuer/extra-checker probes and rejects plus one.
- **Artifact transport:** deterministic extracted-tree repack matches;
  conditional upload, native study-index registration, fresh-cache download,
  remote check and all 8,729 archived/loose member hashes pass in
  `setec-astronomy-artifacts`. The inherited uploader's unknown source revision
  is disclosed independently of the pinned scientific binaries.
- **Reports:** eight final PDFs, all 18 pages opened and read; units, graph
  labels, ids, bounds and references checked. Initial layout/reference errors
  are recorded in `checks/report-render-attempt-v1/`. Final graph/PDF copies
  match the study bit for bit; PNGs use the same SVG source at 150 dpi.
- **Tracking:** all sixteen blocked/unfunded cards validate, keeping their
  failing run sentinel and zero funding. New report/reproduction links resolve;
  touched Rust formatting passes. Conductor remains unavailable (exit 127).

Rust 1.90.0 and the persistent tool paths used in the native receipts are
recorded with the protocols. All final shared solver code is frozen in
7d6f09c8d before the last scientific run and post-change test build. Later
changes are evidence/report/tracking work. Earlier pre-feature test logs are
retained separately and do not substitute for the post-change checks.

No calibrated primitive accounting, complete IC/rho reference, external
full-root fastest-tool comparison, heldout final selection or independent-host
replication is admitted. Benchmark copies are unadmitted diagnostics; canonical
leaderboard/scoreboard, progress timeline and curve-map data have no promoted
change. Size, exact pushed head, current-head CI/review and existing main-branch
conflicts are checked separately in delivery receipts. Local tests do not
authorize a merge.
