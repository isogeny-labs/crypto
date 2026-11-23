# PROTOCOL: kappa at genus 3 (frozen before any data) [pcc:unverified]

Date: 2026-11-22. Host: this VM (4 vCPU, no hardware PMU), single thread, wall clock only.

## Question
kappa = cost of one counted mod-r multiplication (`la_mul`) in units of one counted F_p
multiplication (`cantor_mul`), at genus 3, p in {65537, 131101, 262147}. Reference: main's
PR #4852 measured kappa ~ 2 (genus-4 calibration). The light-row selection wins only for
kappa > 2.5-4.1 (light_floor replay).

## Method
1. t_F in situ: time batches of `JCurve::cantor`/`mul_rtl` group operations on a real genus-3
   instance (f, p from `research/genus_ladder_g3_highp_20261121/run_131101.jsonl`; other p
   from the same family), divided by (fmul + 10*finv) from the instrument's own counters
   (`JCurve::fmul/finv`). Bare comparison: micro-benchmark of `(a*b) % p` chain on u64.
2. t_R in situ: `run_ic` with SGE(8,3) on the instance. `IcResult` gives only whole-run
   `wall_ns`, so LA time is NOT separable directly: t_LA = wall(full run) - wall(relation-
   phase-only run), where the relation-only run is the same seed with the same relations
   but stopped before LA (not exposed); if it cannot be built without source changes, the
   fallback is replaying the LA phase on a synthetic core with the real shape (rows, cols,
   nnz, generic fraction from `IcResult`), calling the real `wiedemann` via a test-module.
   Whichever is used is stated in RESULTS.md. t_R = t_LA / la_mul (counted, from IcResult).
   Bare: chain of `mulmod(a,b,r)` + `addm` with the same r (r < 2^50: f64 Barrett).
3. kappa_wall = t_R / t_F, in situ and bare. Additions: la_add vs F_p add reported
   separately if separately counted (Nat.add exists; cantor adds are not counted, so the
   F_p-add side is a bare micro-benchmark only).
4. Trials: warm-up, >= 15 trials, median; trials interleaved F/R; load average read before
   and after each trial.

## A/A noise
Every timing is run twice as arms A and A'; spread = |median_A - median_A'| / median.

## Prediction (before data)
Cantor's F_p multiply is u128 accumulate plus a `%` by a runtime p per product (~4-8 ns
inside Cantor with overhead from poly bookkeeping and the 10*finv charge); the LA mulmod
is f64 Barrett (~2-3 ns) but mat_vec is memory-bound (random gather on x, 16-byte (u32,u64)
entries). Predicted kappa_wall in situ 0.5-3, central about 1.5; bare 0.5-2.

## Decision rule
kappa_wall (in situ, median over p) < 2.5: light-row floor is not an improvement on this
host; > 4.1: it is; between: unresolved. Gaps judged against the A/A spread.

## Limits (declared in advance)
One host, no PMU, wall time on a shared VM, one instance per p, no claim about other hosts.
All figures [pcc:unverified].
