# PROTOCOL: N4 iteration 2, double large primes (B2) and structured elimination (B1) against the recorded B0

Frozen 2026-10-21, before any grid data of this study. Branch `claude/quartic-ic-n4-la-20261021`
(stacked on #2227, which carries #2283). Baseline: **B0** of `research/quartic_n4_20261019/`
(recorded records `results/grid/all_records.jsonl`: reduced factor base `N = ceil(3.0 q^0.6)` with single large
primes, Lanczos mod `ell`). Success criterion (AGENTS.md): a measured improvement over the current baseline on
the same instances, every answer verified.

## Why

B0 spends 57 to 74% of its cost in linear algebra (`n` equal to the factor-base size, `n^2` with `N = q^0.6`)
and its relation phase produces about 3x more rows than columns for free. Two candidate changes:

* **B1**: structured Gaussian elimination (singleton columns, weight-2 merges, heaviest-row pruning) of all rows
  before Lanczos, with back-substitution. Pilot (q = 4051, 16363, seed 101): correct, about 25% less linear algebra.
* **B2**: double large primes (the Gaudry-Thome-Theriault-Diem variation). Each line through two factor-base
  points `A, B` meets `C` in two more rational points `X, Y` (roots of a residual quadratic computed in closed form from
  `F(D)`, `grad F(A) . D`, `F(A - D)`, `D = B - A`: about 85 multiplications and a square root per pair). Both outside the
  base: an edge between two large primes. Breadth-first potentials `p_v` (sparse vectors over factor-base logs and `eta`)
  from grounded vertices (one point in the base) turn every non-tree edge into a relation among factor-base logs only;
  rows are a random subset of 1.2 n + 30 of them (lightest-first selection was tried and rejected in a pilot: rank
  deficient). The factor base shrinks to `N = ceil(2.0 q^0.5)`. Large-prime logs by propagation. Descent as in B0
  (known set: every vertex reached from a grounded vertex).

## Arms and instances (frozen)

Same instances as the B0 grid (`Inst::search(q, seed)`, sources `common.rs` and `ghs.rs` byte-identical to
`quartic_n4_20261019`; the `rho` function identical): `q` the largest prime below `2^k` with `q = 7 (mod 12)`,
k = 8..18 as before plus **k = 19, 20 (q = 524287, 1048507)**; seeds 1..5 for k <= 16, 1..3 for k >= 17.

* **B2**: all sizes k = 8..20, `c = 2.0`, retry with `c += 0.4` (all attempts charged), up to 6 attempts.
* **B1**: B0 parameters (`c = 3.0`, `q^0.6`), structured elimination on, `maxw = 12`; sizes q >= 4051 up to 262111.
* **B0n**: B0 re-run in the new binary (deterministic relation order): the control that the new build did not change B0.
  Sizes 4051..65479, seeds 1..3.
* **rho**: the recorded matched rho runs of the B0 grid for k <= 18 (same instances, same code; counts are load
  independent); new rho runs for k = 19 (6 per instance) and k = 20 (4 per instance), seeds 1..3.

## Cost, pricing, controls

Unchanged from `quartic_n4_20261019/PROTOCOL.md`: M-eq = mul + r_inv inv + r_lmul lmul; GAE = counted E0 addition;
unpriced hash operations (and the rho distinguished-point tests) at w = 0, 0.1, 1 GAE; ranking only where the order
holds across the band with the bootstrap interval excluding 1. The structured elimination and the graph search count
their list operations as hash operations (unpriced band). Controls: every unit verifies `kG = Q`; sampled relations
(full, one-prime and **graph-edge** relations) are summed through the canonical `Pic^0` group law (`relations_bad = 0`);
60 hold-out rows must lie in the kernel; rho calibration steps/sqrt(ell) in [1.0, 1.5] at the new sizes; instance control
(`n4 check`) at the new sizes.

## Pre-registered predictions

* **P1** every B2, B1 and B0n unit verifies (retries allowed, charged).
* **P2** B2 / B0 cost ratio <= 0.35 for q >= 65479 and <= 0.5 at q = 4051 (paired by size, mean over instances).
* **P3** B2 exponent on q >= 4051 in [0.95, 1.15] (model: pair stage `N^2 ~ q`, linear algebra `~ N^2 ~ q`).
* **P4** B1 / B0 >= 0.6 (the elimination gains at most 40%: columns have weight about 6, few of weight <= 2).
* **P5** B0n / B0 within [0.9, 1.1] at every size (control).
* **P6** B2 is band-resolved below rho for every q >= 2011 (B0 was resolved from 4051).

## Decision rules

* B2 is a measured **improvement** over B0 iff all its units verify and the paired cost ratio B2/B0 has a bootstrap
  95% interval entirely below 1 at every q >= 4051; then B2 is the new baseline and the distance to rho is reported
  against it. Otherwise it is recorded as a null or trade, with the arm table.
* The B1 result is stated as a stage improvement of linear algebra and as a pipeline change only through its B1/B0 row.
* The exponent claim covers primary range q >= 4051 (8 or 10 sizes, bootstrap over units and runs) next to the
  prediction; extrapolations to larger q are marked as extrapolations.

## Claim hygiene

As for #2283: GHS-weak family only (`4 | #E0`, `E0/F_{q^3}`), plain rho (a negation map could lower rho by up to
sqrt 2), L0 shared VM, wall time a practicality note, no external implementation comparison, no generic-curve,
prime-order, key-recovery or deployed-curve claim.
