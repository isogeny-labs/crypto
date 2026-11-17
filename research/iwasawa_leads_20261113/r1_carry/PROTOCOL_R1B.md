# R1b protocol: non-affine leakage of ψ

Frozen 2026-11-14, before any data was taken. This extends `PROTOCOL.md` (R1), which tested only
affine leakage. Same object ψ, same lift L, same exact local toy computation, no timing.

## Statistics

- **S4, bit leakage.** p is a prime in [2¹³, 2¹⁴]. Over all x = 1..n−1, form the 2×2 table for every
  pair (bit i of x, bit j of ψ(Q_x)), with i < ⌈log₂ n⌉ and j < ⌈log₂ p⌉, about 196 pairs. Report
  the largest χ² (1 degree of freedom).
  - Bonferroni threshold at family-wise 0.001: χ² = 20.8.
- **S5, degree.** p is a prime in [2¹¹, 2¹²]. ψ is odd (L(−Q) = −L(Q)), so f = ψ/y is a function of
  the x-coordinate alone. Over the set X of x-coordinates of E(F_p) \ {O} with y ≠ 0
  (m = |X| = (n − 1)/2), compute the degree of the interpolating polynomial of f by Newton divided
  differences.
  - Report the deficit m − 1 − deg.
  - For a function with no structure, the deficit is k with probability about p^(−k).

## Arms

- **Test arm:** 12 non-anomalous prime-order curves per statistic, from seeded search
  (seed 20261114).
- **Negative control (S4):** the ψ table under a seeded permutation of x.
- **Positive controls:**
  - S4: on 4 anomalous curves use ψ(Q)/ψ(P), which equals x exactly.
  - S5: interpolate the known function x³ + ax on the same X; its degree must be 3.

## Predictions

- **Instrument check.** The S4 positive control exceeds 20.8 by far. The S5 positive control has
  degree 3. If either fails, the run is void [pcc:unverified].
- **Null (lead closes).**
  - S4: max χ² ≤ 20.8 on at least 11 of 12 curves.
  - S5: deficit ≤ 2 on all 12 curves.
- **Lead survives.** Any test curve breaks a null bound. Then reproduce on 12 fresh seeds before
  saying anything.
