# Protocol: torsion levers for the Curve25519-family algebraic factor base (frozen 2026-11-03)

Frozen in git before any count. There is **no pilot**: every prediction below
comes from theory and from the parent study
(`research/curve25519_frame_search_20261103`, merged in #3359). Every curve
is therefore a confirmation curve.

## Background

On the algebraic track (`pkm-semaev`, base = {P : x(P) in a frame}, no
cofactor projection), the exact coverage of a base depends on its column count
**and** on the cofactor cosets of its points. E(F_p) ≅ Z/8 × Z/r is cyclic.
A sum of base points is a usable target only if its coset is right. The parent
study chose frames by the log-free count J of coset-cancelling pairs. Three
torsion levers follow from that and are untested.

- **T1, torsion-shifted targets.** Decompose R + T_c for a rational torsion
  point T_c of coset c, instead of R. Because log T_c ≡ 0 mod r, a relation
  R + T_c = ±P_i ± P_j is a valid relation for R. The oracle then hits at
  rate N_c / r, where N_c is the number of covered points in coset c. The
  plain oracle hits at N_0 / r. Choosing c* = argmax_c J_c (pairs landing in
  coset c, from coset labels only) costs one point addition per target.
- **T2, 2-descent-filtered base.** On v² = u³ + Au² + u with A² − 4 a
  non-square, (0,0) is the only rational 2-torsion point. The 2-descent map is
  P ↦ u(P) mod squares, so u(P) is a square ⟺ P ∈ 2E(F_p), which is the even
  cosets. The base {P : u(P) = w², w in a frame} sits in even cosets.
  - For uniform even cosets, the probability that a pair cancels (c_a ≡ ±c_b)
    is 24/64, against 14/64 for uniform cosets: a predicted ×1.71 in coverage
    per column².
  - The cost is that the Semaev system in w has degree 4 per variable instead
    of 2. That F4 cost is **not** measured here; the census only decides
    whether T2 is worth the engine work.
- **T3, m = 3.** Coverage scales as cols³ and triples must cancel in coset
  (c_a ± c_b ± c_c ≡ 0). Selecting the frame by the count J3 of
  coset-cancelling signed triples should gain more than at m = 2.

## Design

- **Curves.** The five family-rule miniatures (`curve25519_family family`):
  k = 16 (A = 1058), 18 (78), 20 (926), 22 (43186), 24 (318).
- **Instrument.** `scripts/frame_census.py`, the parent study's frozen
  instrument, provides the exact full-group logs. `scripts/torsion_census.py`
  adds the new measurements. All counts are exact; there is no timing. Seed
  `20261104 + k`, which is disjoint from the parent's seeds.
- **T1 and T3 frames:** K = 2000 random translation frames x ∈ [L, L + 2^t)
  per (curve, t), t ∈ {4, 5, 6, 7}.
  - m = 2: compute N_c for all c ∈ Z/8 (coset 0 is the subgroup), J_c, and J.
  - Frame-only selection (the parent's rule): argmax J, then cols, then
    lowest L. Its coverage is N_0.
  - Joint selection: argmax over (frame, c) of J_c, with the same
    tie-breaks, then lowest c. Its coverage is N_c.
- **T3 frames:** for t ∈ {4, 5, 6}, the first 300 of the same frames. Measure
  exact m = 3 subgroup coverage (distinct ±a ± b ± c in E[r] ∖ O,
  repetition allowed) and J3 (coset-cancelling signed triples), and select
  by argmax J3. The m = 2 comparison uses the same 300 frames, selected by J.
- **T2 frames:** the same K = 2000 translation frames in w, with base
  {P : u(P) = w², w ∈ [L, L + 2^t)}, t ∈ {4, 5, 6, 7}. Record cols, the coset
  histogram and exact m = 2 coverage. The comparison is the plain
  translation frames in x.

## Predictions (each decided by exact counts)

- **Q1 (T1).** Joint (frame, coset) selection's coverage is ≥ 1.10 × the
  frame-only selection's coverage in at least 12 of the 20 cells
  (5 curves × t = 4..7).
- **Q2 (T1 selector).** The joint selection's coverage is ≥ 0.9 × the
  oracle-best (frame, coset) coverage in every cell.
- **Q3a (T2, a theorem check).** Every T2 base point lies in an even coset, in
  all cells. A single odd coset falsifies the 2-descent claim as implemented.
- **Q3b (T2).** The mean, over frames, of covered/cols² for T2 bases divided by
  the same for plain frames is in [1.4, 2.1] in every cell with
  t ∈ {5, 6, 7}. (Predicted 1.71 from 24/64 ÷ 14/64.)
- **Q4 (T3).** The m = 3 frame-selection gain (selected ÷ mean frame,
  coverage) is greater than the m = 2 gain (selected ÷ mean, same 300 frames)
  in at least 12 of the 15 cells (t = 4, 5, 6).

## Failure handling

Failed predictions are reported as failed and not retuned. If Q3a fails, T2 is
dropped and the 2-descent claim goes to an errata note.

## Scope

- These are coverage counts only, the same instrument as the parent study,
  which was validated against the engine's `coverage_exact` there.
- T2's F4 cost and T1's per-target F4 cost (an unchanged system, since the
  target abscissa only differs) are claims for a later engine step.
- This is not a leaderboard row. The algebraic track's distance to rho is
  unchanged by a census.

## Addendum A (frozen before its runs): T2 engine cost, k = 20

The exploratory engine pilot in `engine_pilot/` (square-bits at lo = 0,
compared with the parent's bits at lo = 0) showed F4 mul-adds/target rising
2.1–5.1× for T2: solving degree 6 → 9 and terms 256 → 961 at t = 5, G = 0.
That eats most of the 2× coverage.

- **Confirmation.** k = 20, G = 3, t ∈ {4, 5}, 100 targets, seed
  2026111500 + t. Compare:
  - `bits@L` at the frame-only selected plain frame from `data/k20.json`:
    L = 797820 (t = 4), L = 233902 (t = 5);
  - `square-bits@L` at the **oracle-best** T2 frame from the same file:
    L = 984071 (t = 4), L = 414271 (t = 5). This choice is deliberately
    favourable to T2.
- **E1 (prediction from the pilot).** T2's F4 mul-adds per relation (at exact
  coverage) is ≥ the plain frame's in both cells. Then T2 is closed in the
  bit-encoded F4 form. If T2 is cheaper in either cell, T2 stays open and goes
  to a full confirmation.
