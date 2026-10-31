# Signed factor-base policy v2: executable activation contract

Read-only source audit at branch `codex/ml-dataset-factor-base-policy-20261030`
on 2026-10-31. This file records the integration dependency; it is **not**
evidence that a v2 dataset cell has been generated or exported. Conductor
T-887 reserves `src/ml_dataset/mod.rs` and `solvers.rs`; T-936 reserves the
plan/policy/curve/type files. The paths below require coordinated ownership.

`PlanDocument` in `src/ml_dataset/plan_v2.rs` already parses v1/v2 strictly,
expands cells, and assigns distinct occurrence identities. The signed orbit
constructor in `curves.rs::generate_with_policy` is implemented. The CLI's
`Plan` and `Preflight` paths in `mod.rs` still emit/parse only `DatasetPlan`
v1. `Generate`, `Worker`, `bound_run`, `verify_cell`, `verify_run`, resumed-cell
verification, and reconciliation also bind plain v1 `CaseConfig` and run
schemas. `Worker` calls `curves::generate`; no CLI route reaches the policy
constructor. `case_binding.rs::BoundCase` provides exact saved-plan-byte,
policy-index, config, manifest-role, and restricted construction replay
helpers, but its worker activation guard intentionally rejects v2.

Capture, report, export, and restore still derive cells using
`DatasetPlan::cell(index)` and v1 run identity. Export's feature projection
rejects the v2 factor-base schema. Its v1 orbit fields are not interchangeable
with v2 `signed_orbits`, `action`, and `domain_digest`; v2 requires an explicit
projection and leakage review. A flag in `Plan` alone would create a plan that
the worker and custody readers cannot verify.

## Coordinated activation sequence

1. Freeze the pure v2 policy/occurrence API under T-936. Preserve v1
   serialization byte-for-byte. Give a v2 `legacy` policy an explicit
   occurrence identity even when its mathematical `CaseConfig` matches v1.
2. Coordinate T-887's `mod.rs` edit after its dynamic-adapter work is stable.
   Dispatch strict `PlanDocument` and bind each cell to the **bytes actually
   saved as that run's plan**, policy index, config, and construction transcript.
   Generation must select `generate_with_policy` for v2 and retain a censored
   constructor outcome for empty signed domains without invoking a solver.
3. Migrate capture, report, export, restore, resumed-cell verification, and
   worker startup under their explicit file owners. Carry the new occurrence
   and signed-orbit fields through restricted artifacts; public projection
   must omit target/scalar leakage and preserve censored outcomes.
4. Remove the v2 activation guard only when the entire read/write/replay path
   is implemented and mutation-checked. Keep the v1 path and fixtures unchanged.

## Acceptance evidence required

- Actual CLI v2 plan → preflight → one generated cell → verify → report →
  strict export → publish → restore/replay, across Koblitz, ordinary binary,
  and prime families and both twists. A plan-only test does not qualify.
- Two policy values with the same `CaseConfig` must have different occurrence
  identities, and replay must reject changed policy/index, saved plan bytes,
  config, artifact role/hash, or construction transcript.
- Empty signed domains must be retained as censored constructor outcomes, with
  no solver invocation or fabricated PDP/scalar label.
- Byte-for-byte v1 plan, cell, manifest, and export fixture regressions; exact
  current-head source/tests after all owner branches are integrated.

The current local campaigns use v1. Their verified native cells do not prove
this v2 activation path.
