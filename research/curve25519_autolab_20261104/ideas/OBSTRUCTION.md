# Why the generic track has stalled near 1.8x matched rho (derived, not a proof)

Written 2026-11-05 after rounds 25 to 35, R4-I1, the `seed=run` re-baseline and R4-I2. Measured numbers are [pcc:unverified]; the formula below is a heuristic derivation in the random-collision model, not a theorem about every algorithm.

## The count

Every arm the loop has measured on the generic track gets its relations from coincidences among pseudo-random group elements:
- box or walk table entries colliding with each other (free rows);
- table entries colliding with probes (hit rows).

Relations come from no other source. Take a factor base of C seed points with unknown logarithms, plus the target. Then K = C + 1 logarithms are unknown, and an elimination needs K independent rows.

In the random-collision model, pool all points the method touches (table and probes alike) and fold the negation, giving about r/2 classes. Each new point collides with an earlier one at rate roughly t/(r/2), so the collision count after t points grows like t^2 / r. The K-th collision then needs about

    E[t_K] = sqrt(r) · Γ(K + 1/2) / Γ(K)

points, because t_K^2 / r is Gamma(K, 1) distributed. Splitting the pool into a table and probes, interleaving them, or reweighting their sizes does not lower the expected pair count needed. It only moves it between phases.

| K = C + 1 | floor S = Γ(K+1/2)/Γ(K) | floor / rho-neg floor (0.886) | what reaches it |
|--:|--:|--:|:--|
| 1 | 0.886 | 1.00 | Pollard rho with negation (no unknown seeds) |
| 2 | 1.329 | 1.50 | one unknown seed |
| 3 | 1.662 | 1.875 | two unknown seeds (the `wwalk g=1 grow=2` incumbents) |
| 4 | 1.939 | 2.19 | three unknown seeds (the `wsum` C = 3 boxes) |

## How the measurements sit against it

- **C = 2 incumbents:** `wwalk g=1,grow=2,seed=run` measures S 2.11 / 1.88 / 1.84 at k = 24 / 28 / 32, against matched rho-neg 1.08 / 1.04 / 1.01 [pcc:unverified, PCC-20261104-d91a51].
  - Above the K = 3 floor of 1.66 by set-up (jump building, row checks) and by the gap between finite and asymptotic sizes.
  - The ratio to matched rho, 1.8x to 1.9x, matches the floor ratio 1.875.
- **x-grid in round 35:** S is flat in x [pcc:unverified, PCC-20261104-d8bbec], as expected once the method sits at its floor.
- **Interleaving (P3):** it lost [pcc:unverified], consistent with the pair count being fixed while probes made before the seeds were pinned were wasted.
- **Fixed-table rounds 31 to 34:** they looked better (1.43x to 1.55x) only because the table was one favourable draw (R4-I1).

## Consequence for rho parity

Within this model, parity needs K = 1: no unknown factor-base logarithms. A table method with K = 1 is baby-step giant-step or rho itself, which is a relabelling, not an index-calculus result. The repository's success rule says relabelling does not count.

A method at or below matched rho therefore has to obtain relations some other way than collisions among pseudo-random points:
- polynomial systems (the algebraic track, about 18x at k = 20 [pcc:unverified]);
- an endomorphism or quotient that rho cannot also use (ruled out for these curves in STRUCTURE-R4, and R4-I2 falsified at Stage 0 [pcc:unverified]);
- some structure not yet named.

This is the obstruction future rounds should be measured against. The audit trigger is the absolute cost, not the ratio to measured rho: audit an arm for accounting or sampling before anything else when its S falls below the floor for its K (1.662 for two unknown seeds, 1.329 for one), as R4-I1 was. A ratio to measured rho is a poor trigger because measured rho-neg carries 14 to 65 percent overhead above its 0.886 floor at these sizes (1.45 at k = 20, 1.01 at k = 32), so the current incumbents (1.76x to 1.94x measured rho [pcc:unverified]) already sit near the 1.875x floor ratio without breaking the floor.

## What this note does not show

- It is not a lower bound for all algorithms. Shoup's bound covers generic algorithms, and these curves are concrete.
- It does not exclude a decomposition whose relations come from algebra rather than collisions and that costs less than sqrt(r) per relation. That is the open problem the algebraic track measures.
