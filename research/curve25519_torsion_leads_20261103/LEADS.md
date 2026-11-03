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
| T3 | m = 3 frame and coset selection (triples must cancel in Z/8) | **open, promising**: selection gain 2.8–5.4× at t ≤ 5 (Q4 held); m = 3 engine cost next |
| T4 | Frame and coset selection on the PKM isogeny tower (best algebraic arm): select its `seed` by coset-cancelling J | **top next lead**, engine |
| T5 | Make frame selection the `pkm-semaev` default; re-price `curve25519_algebraic_ecbench` end-to-end | ecbench |
| T6 | Edwards t = 5, G = 0 per-target 0.646× outlier, over more seeds | engine, minutes |
| T7 | FGHR-style symmetrisation under T0 (x ↦ 1/x) on a frame-selected base; the T0 closure doubled yield on bits | engine |
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
