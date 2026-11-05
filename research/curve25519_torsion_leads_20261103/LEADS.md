# Curve25519 index calculus: next goals (2026-11-03)

Written after `research/curve25519_frame_search_20261103` (#3359).

## Where things stand

- **Generic track (the baseline).** It is 7.8–24.7× rho at k = 20–32. No
  coordinate or torsion lever exists there: for any N-column base,
  P(decomposition) ≤ #signed pairs / r, and the projected base already
  reaches that bound. Gains there must come from elsewhere: the walk, the
  table, the linear algebra.
- **Algebraic track.** Its best arm is 18× rho at w = 0 and 9.6·10⁵× at
  w = 0.1. F4 work is the cost.
  - Frame selection cuts F4 work per relation by 3.3–4.7× (k = 20).
  - Coverage there depends on cofactor **cosets**, which opens torsion levers.

## Ranked leads

| # | Lead | Status |
|---|---|---|
| T1 | Torsion-shifted targets: decompose R + T_c, with c chosen from coset labels | **closed**: 1.00–1.12× on top of frame selection (Q1 failed) |
| T2 | 2-descent-filtered base, u = w² (even cosets) | **closed in bit-encoded F4**: coverage ×2.0, but F4 per relation is 1.28–1.71× worse even at T2's oracle-best frame (Addendum A) |
| T3 | m = 3 frame and coset selection (triples must cancel in Z/8) | **closed** (wave 2): selection survives the engine (F4/relation 7.6× and 11.8× lower than lo = 0), but m = 3 stays 2,300–6,600× costlier per relation than selected m = 2 |
| T4 | Select the PKM isogeny tower `seed` by coset-cancelling J | **done** (`curve25519_isogeny_seed_20261103`, #3363): F4/relation 3.10× lower at k = 20; end-to-end S 0.64× at w ≥ 0.1 (CI includes 1); replicated on k = 16, 18, 22, 24 (3.20×, 2.43×, 2.10×, 1.41×) |
| T5 | Re-price the best algebraic arm end-to-end with selection | **done** for the isogeny seed (ecbench `ECBS1h405f20219036`); bits needs an interval factor base in ecbench first |
| T6 | Edwards t = 5, G = 0 per-target 0.646× outlier, over more seeds | **real but useless** (wave 2): 0.6386× on 7/7 seeds (k20) and 3/3 (k24), uniform per target (F4 refutes in 10 steps vs 11); gone at G ≥ 1 and at t = 4, 6, so no gain at the best cell |
| T7 | Frame selection on the T₀-invariant base | **closed** (wave 2): coverage 20.5×/5.9×/2.9× over lo = 0 and 0.93–0.95× the best bits frame per relation, but the isogeny tower is 3.9–79× cheaper |
| T8 | 4E / 8E filtering through the halving tower (u of [2]Q, degree 4): the cosets are concentrated further, at the price of polynomial degree | **deprioritised**: T2 already loses at degree 2 in w; degree 4 is worse |

Closed (not pursued): coordinate model choice (#3359), Hessian, Jacobi
intersection and Huff (not defined over F_p for cyclic Z/8), E[8]-invariant
keys on the generic track (projection already gives the same thing), and
twist x-values (a mixed relation would force a 2-torsion summand).

## Lesson from T1 and T2 (2026-11-03)

- Restricting cosets *algebraically* raises the solving degree and loses.
- *Selecting* among parameterisations (frame `lo`, tower `seed`) leaves the
  system unchanged and wins.
- Next leads should search the free parameter of each membership, ranked by
  the log-free coset count J.

## After wave 2 (2026-11-03)

- The isogeny tower with a selected seed is the cheapest algebraic arm in
  every measured cell.
- **T9 (next).** Select the seed by J ÷ columns, not by J.
  - End-to-end cost scales as columns × r / coverage.
  - The coverage-per-column model predicts 0.497× seed 0 for seed 1308
    (k = 20, t = 6, G = 4), against 0.599× predicted and 0.641× measured
    for seed 1093.
- **T10.** Search the guess split G jointly with the seed. T6 shows the
  per-target F4 cost depends on (form, t, G).
- **T11.** An interval factor base in ecbench, so that selected `bits` and
  `t0-bits` frames can be priced end-to-end.
