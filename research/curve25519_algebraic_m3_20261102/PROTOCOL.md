# Protocol: three-term algebraic decompositions (m = 3) on the Curve25519 miniature, k = 20

Frozen before any data. Written 2026-11-02, branch `lead3-algebraic-m3`.

## Question
At m = 2 the isogeny-tower form costs about 6e9 F4 multiply-adds per relation
at k = 20 (t = 6: 5.6e7 per target, lambda_2 ~ 0.0095; t = 7: 7.5e8 per target,
lambda_2 ~ 0.063; `research/curve25519_algebraic_ladder_20261102/analysis/`).
Coverage at m = 3 grows ~N^3, so a much smaller tower may reach useful yield
with a small system. Does any m = 3 cell beat the best m = 2 per-relation cost?

## Instrument
`curve25519_family f4cost <k> <form> <t> <guess> <targets> <seed> <cap_s> <m> <mode>`
(new optional trailing arguments; default `2 full` reproduces the old runs).
Oracle `pkm-semaev:m=3,mode=<chain|full>[,guess=G]`, `src/cryptanalysis/ic_framework/pkm_semaev.rs`.
Curve: `build_miniature(20)` (#E = 8r). Targets: random subgroup points
`a*G`, `a` from an LCG seeded with the cell seed.
Unit: F_p multiply-adds in F4 elimination only (`solver_totals().ops`), the
same unit as the m = 2 data. Not charged: system construction, root lifting,
group re-adds (these are native work to be priced at 0 / 0.1 / 1 GAE per op
under AGENTS.md 5a before any cross-method ranking; this protocol compares
F4 multiply-adds only, m = 3 against m = 2, same instrument).

## Verification gate (before data)
Unit tests (all in `cargo test --release --lib pkm_semaev`): for isogeny and bits
(guess 0, 1, 2) at m = 3, chain and full F4 verdicts equal the grid verdict
equal membership in `decomposable_points(curve, fb, 3, ..)`; every relation
sums to the target. Each data run additionally re-adds every returned relation
(`hits_verified == hits`).

## Cells
k = 20; 100 random targets per cell; seed = 2026111300 + t; per-cell cap 600 s
(a capped cell reports `targets_done < 100` and is analysed on the targets done).
- isogeny, t in {3,4,5,6}, G = 0, mode chain; mode full also attempted for every t.
- bits, t in {4,5,6}, G in {0,3}, mode chain; mode full attempted where it
  finishes inside the cap.
Cells that do not complete even a handful of targets in the cap are reported
as intractable, not extrapolated.

## Measured per cell
columns (factor-base size), F4 multiply-adds per target, maximum solving
degree, undecided systems, hits / targets, hit rate, per-relation cost =
multiply-adds per target / hit rate.

## Preregistered comparison
C = min over m = 3 cells (with hits >= 1) of per-relation cost; B = best m = 2
per-relation cost, taken as ~6e9 (isogeny t = 6, from the ladder data; the
ladder file is re-read and the best recorded m = 2 row used).
- Improvement (verified, measured): C < B / 2 (a factor of two, beyond the
  small-sample noise of ~100 targets, and with hits >= 5 in the winning cell).
- Match: B/2 <= C <= 2B. Regression: C > 2B.
- Hit-rate sampling: a cell with fewer than 5 hits has a per-relation cost
  with relative error > 45%; its estimate is flagged and cannot be the winner.
Gap to matched rho is not computed here: no rho reference run is part of this
experiment and no leaderboard row is claimed (validation gate not satisfied).

## Deviations policy
Any change of cells, caps or seeds after data is listed in README.md.
