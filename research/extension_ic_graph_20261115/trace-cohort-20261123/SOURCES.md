# Source and prior-result review

No KB MCP is loaded and Conductor command is unavailable. Reuse the earlier
Joux–Vitse review and inspect source-native counter semantics, rather than
claiming novelty or completeness from the field names.

- Pinned Root41source archive (retained from Run41), f4_fp.rs and jv_cover.rs:
  ZeroDimStats.f4_muls is the F4 report's field_ops counter (row-reduction
  Fp multiplications); lin_muls covers normal forms, Krylov, roots/eigenvectors.
- leads-20261118/descent-finder/README.md and CANDIDATES.md: a separate existing
  traced-six study already analyzed seed389/3415 and seed401/1506. Its source
  reports learning tracked-RREF work outside field_ops and supplies its own
  upper bound. That bound is not transferred to this binary or five-variable arm.
- leads-20261118/descent-finder/traced_descent_check.rs: applies a report
  acceptance predicate, then explicit fallback on rejection. Root41f4_replay
  normally finishes a diverging trace as a full run, but the invalid recorded
  row-index branch can return an early report with trace_mismatch=true. The
  captured root-set controls match throughout; that observation does not prove
  completeness of every future malformed/exceptional system. No generic
  correctness theorem is claimed. A focused regression/certification audit is
  a distinct next obligation before broadening the backend's domain.

This analysis reads saved outputs only; it does not execute a new F4solver,
apply another study's pricing, repair old observations or infer whole-pipeline
cost. It preserves all statuses and separates replay, mismatch and learning cases.
