# NIST BEST2 DP bias and successful-collision GPU validation

- **Status:** investigating
- **Kind:** unknown cause; implementation gap; access/tool limit
- **Affected work:** [PR #2689](https://github.com/isogeny-labs/crypto/pull/2689), [seeded audit](../research/nist_best2_small_batch_20261027/seeded-audit-v1/README.md)
- **Next action owner:** authorized GPU/EKS experiment runner; independent validator
- **Last updated:** 2026-10-27

## Problem

The P-256 BEST2 DP deficit cannot yet be attributed to selection bias versus
sampling, and the derived 0.947 cost estimate cannot be promoted to a measured
GPU cost per successful collision. The repeated timing cells share walks;
there is no independently seeded exact-kernel panel or validated GPU scalar
recovery/replay receipt. P-384 is a required control, W=16/32/64 remain in scope.

## Evidence and uncertainty

- **Observed:** historical obs/exp 0.923/0.946/0.974 (one deterministic sample
  each W), as recorded in the original study. No new statistical panel ran.
- **Source-verified:** DP eligibility excludes detection/seek; cycle hash
  overlaps the DP coordinate. Warmup DPs were mixed into a measured-only
  denominator (corrected). The original standalone model uses different
  scoring, DP, restart and escape semantics. These facts do not prove bias.
- **Correctness:** local native CPU suites and small OpenSSL P-256/P-384
  trajectories pass. An infinity-start host restart-identity defect is fixed
  and has a constructed regression. Evidence and scope are in the audit.
- **Missing capability:** this session has no authenticated GPU/EKS route or
  workflow-dispatch tool; no nvcc/device; available Cargo 1.75 cannot parse
  root lockfile v4. The new ecbench model's dependency-free bookkeeping test
  passes, but root integration and sealed-session replay remain unverified.
- **Missing implementation:** the GPU throughput driver only counts DPs;
  full-size host coefficient recovery and a GPU-to-host replay path do not
  exist in `RhoHost` (orders must be <2^63). Generic model counts are not a
  replacement. No feasible full-size NIST rho solve is claimed.
- **Not established:** DP independence, persistent selection bias, 5% actual
  savings, complete successful-collision accounting, default enablement.

## What would unblock it

- [ ] Execute both fixed-table/varying-start and varied-table/start panels in
  the [frozen protocol](../research/nist_best2_small_batch_20261027/seeded-audit-v1/PROTOCOL.md), retaining all seeds/failures and independent summaries.
- [ ] Obtain a current toolchain root build, registered native ecbench model
  plan/session and second-host exact replay; report its model-only scope.
- [ ] Validate a bounded GPU known-answer collision workload, exact DP replay,
  independently recovered/verified scalars and fully charged effective cost.
- [ ] Resolve any material eligible-step deficit and supply independent
  validator/replay artifacts before any performance promotion/enablement.

Those conditions qualify their measured workload only; small-order fixtures do
not establish a full-size NIST solve rate or a cryptanalytic security change.

## Next discriminating step

Execute the fixed-table/varying-start seeded panel first, with P-384 and plain
controls, physical/eligible denominators, and per-seed DP counts. Then execute
the separately frozen varied-table panel and GPU cells. Timing repetitions are
never new samples. Resource bounds, stop conditions, and decision rules are in
the protocol. A persistent eligible-step deficit motivates a preregistered
alternative scoring/DP-predicate control; disappearance is only sampling
evidence within the measured panel. Access failures preserve the open work.

## Resolution and history

2026-10-27: audit implementation and local correctness tests added. Statistical
and end-to-end validation requirements remain unresolved. BEST2 stays off.
