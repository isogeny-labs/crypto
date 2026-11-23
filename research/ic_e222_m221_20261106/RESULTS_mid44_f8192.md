# F = 8192 at 44 to 45 bit subgroups, with a compact pair-sum table

Extends `RESULTS_mid44.md` (F = 4096, 5.2 GB a run). All figures [pcc:unverified], one run per cell, producer-run.

## The memory change

`build_pair_sum_table` (`src/cryptanalysis/ec_index_calculus.rs`) now returns a compact open-addressing table: a 64-bit fingerprint of the x coordinate plus a packed `(i, s_i, j, s_j)`, 12 bytes a slot at load 0.6. The previous `HashMap<(BigUint, BigUint), ...>` is kept as `build_pair_sum_table_exact`, the reference the compact table is tested against. A lookup that matches a fingerprint is checked against the factor-base points (one addition, on a hit only), so a fingerprint collision is a miss, not a wrong relation; among entries with the same x the first one stored for exactly `(x, y)` is returned, as before, and otherwise the first for `(x, -y)` with both signs flipped. The compact table indexes at most 8192 factor-base points (`PAIR_SUM_TABLE_MAX_FB`), matching the command's cap.

- Test `compact_pair_sum_table_matches_the_exact_table`: over every point of a tiny curve's subgroup, the compact and exact tables give the same hit set and the same decomposition, and each decomposition is a true identity; the 52 `ec_index_calculus` library tests pass.
- Regression: E34 H1 at F = 128 reproduces the earlier run exactly (72,132,515 trials), in 253 s against 271 s.
- Measured memory: 2.57 GB resident a run at F = 8192, against 5.2 GB at F = 4096 for the old table; the old layout would need about 20 GB at F = 8192. Three F = 8192 runs ran side by side on a 16 GB machine.
- Counters are unchanged: the table build counts the same additions, inserts and negations.

## Results (new instances, seed `e222m221-mid-v2`, F = 8192, 8194 relations, two matched-rho runs)

| instance | n bits | trials observed | trials predicted | obs / pred | IC (GAE, w=0) | IC (GAE, w=1) | matched rho, median of 2 (the two runs) | IC / rho, rho = 0.886 to 1.8 sqrt(n) | wall time |
|---|---|---|---|---|---|---|---|---|---|
| E44 H1 | 44 | 1.063B | 1.074B | 0.990 | 8.05e10 | 8.42e10 | 7.2e6 (5.4e6, 9.1e6) | 1.1e4 to 2.2e4 | 86 min |
| M44 H1 | 45 | 1.070B | 1.074B | 0.996 | 8.10e10 | 8.47e10 | 2.6e6 (2.65e6, 2.61e6) | 1.1e4 to 2.2e4 | 88 min |
| W44 default | 45 | 1.074B | 1.074B | 1.000 | 8.13e10 | 8.51e10 | 1.4e6 (3.1e5, 2.5e6) | 1.1e4 to 2.2e4 | 89 min |

- All three recovered the planted logarithm and every matched-rho walk ended at it.
- Observed over predicted trials is within 1% in all three, against a sampling spread of about 1.1% for 8194 relations. Doubling F from 4096 to 8192 halves the trials (2.14B to 1.06B), as the model says.
- Elimination at F = 8192 took 2.4 to 3.0 s, still negligible.
- Two matched-rho runs per instance are too few to rank the instances against each other: the W44 pair differs by 8x (3.1e5 and 2.5e6) and the E44 pair by 1.7x. The last column uses the section 5a rho reference band of 0.886 to 1.8 sqrt(n) instead, which brackets the distance at 1.1e4 to 2.2e4 times for all three. The matched rho is cheaper at every weight (IC rises 4.5% at w = 1).

Limits: one instance per family at this F; two rho runs, unweighted GAE; the largest measured subgroup is 44 to 45 bits against 218 to 220 native; the compact table was checked against the exact one on a tiny curve and by the E34 regression, not on a large-curve cross-check of every hit.
