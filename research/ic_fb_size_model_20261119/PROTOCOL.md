# Factor-base size and the cost of prime-field index calculus: a model test

Frozen 2026-11-19 before any data. Owner request (this session): "take a step
back … can we filter or prune or something differently?", then "All" (run the
factor-base sweep and the filter-economics audit). This file covers part A.

Sources checked: `research/ic_sketch_mixed_solver_20261115` (Part B record,
p24 incumbent: 36,660 trials, hit rate 2.05e-3, 75 relations, 55,506 GAE);
`research/ic_oracle_setup_blocked_20261020` (the oracle's pair-table set-up
charges about one abstract addition per folded pair, ~|F|^2);
`research/ic_planner_exhaustive_20261026` (fitted slope of ln GAE on ln r:
0.619 for the plan, 0.460 for the residual collector). `research-kb` is not
reachable from this container; "not measured before" means not found in this
repository.

## Model [pcc:unverified until measured]

For `prime-abscissa:size=F` (F points, negation-folded), `mitm-prime-blocked:m=2`,
`targets=walk`, target-pinned `incremental-gauss`, group order N:

- pair table: about F^2 counted additions;
- hit rate per walk point: h ≈ 2F^2/N (a point is a sum of two of the ±F points);
- relations to pin the target: R ≈ F/2;
- trials = R/h ≈ N/(4F), one walk addition each.

So GAE(F) ≈ a·F^2 + b·N/(4F), with a ≈ b ≈ 1. Minimum at F* = N^{1/3}/2 with
GAE* ≈ 3F*^2, i.e. S* = GAE*/√N ∝ N^{1/6}: the method cannot reach rho's
N^{1/2} by choosing F.

## Design

Curves: p24 (`icv1-fp24-tm5099-a6f137c6`) and p28 (`icv1-fp28-t28201-65099091`),
explicit parameters as in the 2026-11-15 study. Arms: `rho.negation` (reference)
and the pipeline at F ∈ {32, 64, 128, 256, 512, 1024} (p24) and
{64, 128, 256, 512, 1024, 2048} (p28). 8 targets × 2 measured rounds, warm-up 1,
alternate order, seed 20261119. Specs: `specs/p24.json`, `specs/p28.json`.
Every solve verified (`ecbench verify --replay`).

## Predictions

- **M1 (trials).** At every (curve, F), mean `targets_tried` lies in
  [0.5, 2] × N/(4F).
- **M2 (relations).** Mean `relations_found` lies in [0.4, 1.0] × F.
- **M3 (shape).** A least-squares fit of mean GAE to a·F^2 + b·N/(4F) per curve
  has a, b ∈ [0.5, 2] and relative residual ≤ 15 % at every F.
- **M4 (optimum).** At w = 0 the cheapest measured F is the grid point nearest
  N^{1/3}/2 (p24: 128; p28: 256, F* ≈ 322).
- **M5 (scaling).** S at the measured optimum rises from p24 to p28; the
  ratio lies in [1.2, 2.2] (model: 16^{1/6} ≈ 1.59). Rho's S stays flat.
- **M6 (band).** Uncharged native work (lookups, square roots, Legendre
  symbols) grows with trials, so the cheapest F at w = 1 is ≥ the cheapest
  F at w = 0.

## Classification

A model test, not an improvement claim. Distance to rho is reported for every
row. No leaderboard row. Figures are `[pcc:unverified]` until a validator
report exists.

## Deviations allowed

None to the predictions. A spec that fails to run is recorded in README.md
with its fix before re-running.
