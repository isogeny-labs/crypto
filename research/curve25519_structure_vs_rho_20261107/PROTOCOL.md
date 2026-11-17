# Frozen protocol: structure levers against rho-neg, k = 20 miniature (frozen 2026-11-07, before any data)

Curve: `y^2 = x^3 + 926 x^2 + x` over `F_p`, `p = 1,048,573`; claimed `#E = 8r`, `r = 131,203`
(`research/curve25519_ic_baseline_20261102/analysis/family.json`, unverified assertion). ecdb was not
reachable (no `ECDB_TOKEN`), so every fact is recomputed by `scripts/levers.py` (pure Python, exact
arithmetic, seed 20261107). No ecbench run is planned unless a lever passes checks C1-C8 with a
predicted GAE factor below 1 across w = 0, 0.1, 1.

Predictions (each is recorded as passed or failed in the README; a failed prediction stays failed):

- C1 point count: `#E = 8r`, `E(F_p)` 2-part cyclic `Z/8`; twist order `2p + 2 − 8r = 4r'` with
  `r' = 261,881` prime.
- C2 `x(P + T0) = 1/x(P)` for every sampled affine `P` with `x(P) ≠ 0`, `T0 = (0, 0)` (10,000 samples).
- C3 the order-r subgroup's abscissa set `X(<G>)` and its image `1/X(<G>)` are disjoint (exact, all r
  points). Prediction: intersection size 0, so `x -> 1/x` gives no class beyond `{±P}` on `<G>`.
- C4 exact class counts. Negation on `<G>`: `(r − 1)/2` nonzero classes. `{±P, ±P + T0}` on
  `<G> + <T0>` (order 2r): `(r − 1)/2` classes of size 4 plus the 2-element class `{O, T0}`.
  `{±P + E[8]}` on `E(F_p)`: `(r − 1)/2` classes of size 16 plus one. Ratio of classes to group
  order beyond projection: identical, so predicted GAE factor 1.00.
- C5 Monte Carlo (uniform draws, the class key computed from real coordinates, every recovered
  logarithm checked): mean draws to the first usable collision divided by `sqrt(r)`: 0.886 ± 0.03
  for the three folded keys of C4 and 1.253 ± 0.04 for the unfolded key, 2,000 trials per key.
  This checks the class counts, not walk overhead.
- C6 endomorphisms: trace `t = −1050`, `t^2 − 4p = −1597·22^2·4`; the eigenvalue of
  `omega = (pi + 525)/22` on `<G>` satisfies `lambda^2 = −1597 (mod r)`. Optimistically granting
  `End(E) = O_K`, the least norm of an endomorphism whose eigenvalue on `<G>` has multiplicative order
  m in {3, 4, 6, and the other divisors of r − 1 up to 12} is at least 1597, predicted of order 10^6.
- C7 `E' = E/<T0>`: `#E' = 8r`, full rational 2-torsion, `j(E') ∉ {0, 1728}`, so `Aut(E') = {±1}`.
- C8 `j(E) ∉ {0, 1728}`.

Kill rule per lever: factor on expected GAE per solve ≥ 1 at any of w = 0, 0.1, 1, or the lever acts
outside the order-r subgroup with no class merge inside it, or it is a counting change only.
