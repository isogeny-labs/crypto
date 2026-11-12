# Relation point at native size: E-222 and M-221 (local, unvalidated)

**Status:** unvalidated (pre-gate). Local unisolated host, single machine, wall time in microseconds; no replay certificate, validator report or matched-rho band. Not a leaderboard row.

**What was measured.** The cost of one relation point R = aG + bQ (the dominant per-trial cost once the pair-sum table replaces the factor-base sweep) on the real registry curves: the Weierstrass images of E-222 (Edwards, p = 2^222 - 117, h = 4) and M-221 (Montgomery, p = 2^221 - 3, h = 8), generator mapped, `[r]G = O` checked on the image (`native/native_params.py`). Three paths, all returning the same point (asserted on every scalar pair before timing): the former default (two `scalar_mul` ladders and `add`), two `scalar_mul_vartime` ladders and `add_vartime`, and `Point::double_scalar_mul_vartime`. Above one word `scalar_mul_vartime` is `scalar_mul` itself, so the word-sized gains of `RESULTS_shamir.md` did not carry to these sizes; the joint pass was therefore extended to `BigUint` Jacobian points (one interleaved pass, `G + Q` precomputed, one inversion for the whole sum). Unit test `point_double_scalar_mul_vartime_matches_two_ladders_multiword` (secp256k1 and P-256, `Q = -G`, zero scalars) passes.

**Run.** `examples/relation_point_native.rs` (built in release), 40 random scalar pairs below the subgroup order, 6 repetitions with the order of the three paths rotated each repetition:

| curve | former us | two vartime ladders us | joint us | joint / former | joint / two ladders |
|---|---|---|---|---|---|
| E-222 | 5343.0 | 4775.7 | 2826.6 | 0.529 | 0.592 |
| M-221 | 4972.3 | 4959.6 | 2890.1 | 0.581 | 0.583 |

Raw output: `native/native_timing.txt`; parameters: `native/params.txt`.

**Reading.** At native size the joint pass costs 0.53 (E-222) and 0.58 (M-221) of the former relation point, about 1.9x and 1.7x faster, and 0.59 and 0.58 of two separate ladders, so the interleaving itself accounts for the whole gain (the two-ladder variable-time path is not faster than the former one above a word). Per relation point at native size: about 2.8 ms.

**Limits.** One host, no CPU pinning, single run of 240 timed relation points per path; no confidence interval (the repetition-to-repetition spread was not recorded). Only the relation point was timed, not a full solve: a solve at these sizes needs about 2^110 operations and was not run. The Edwards and Montgomery analogs in the other notes are toy sizes; this is the only native-size measurement. The costs of the factor-base table lookup and elimination at native size were not measured.

## Addendum: native-size fixed costs of a table-collector solve and the per-trial lookup

**Run.** `examples/native_fixed_costs.rs` (release) on the same E-222 and M-221 Weierstrass images, fb = 32, 5 repetitions; output `native/native_fixed_costs.txt`. Same host, no CPU pinning, no confidence intervals. `PairSumTable` and `build_pair_sum_table` were made `pub` for this (no behaviour change).

| item | E-222 | M-221 |
|---|---|---|
| factor base, plain (32 small-x points) | 2.7 ms | 6.2 ms |
| factor base, projected ([h]F, H1) | 12.5 ms | 14.3 ms |
| pair-sum table build, plain factor base (2,048 entries) | 4.7 ms | 4.2 ms |
| pair-sum table build, projected factor base (2,048 entries) | 42.9 ms | 37.6 ms |
| one table lookup (key clone + hash probe, a miss) | 0.076 us (plain), 0.102 us (projected) | 0.085 us, 0.086 us |
| elimination 34 x 33 mod n (220 / 219 bits) | 10.5 ms | 9.3 ms |
| relation point (previous section) | 2.83 ms | 2.89 ms |

Elimination recovered the planted last unknown in all repetitions on both curves.

**Reading.**
- A trial at native size is its relation point plus a lookup: 2.83 ms and 2.89 ms against 0.1 us for the lookup, so the lookup is a few parts in 10^5 of a trial and the trial cost is the relation point.
- The plain-factor-base table build looks about ten times cheaper than the projected one (4 to 5 ms against 38 to 43 ms). That is an artefact of the points: the plain factor base is made of points with tiny x, so most additions invert a small difference of abscissae, which is cheap; the projected points have full-size coordinates. The projected figure is the representative cost of the table build; the plain one understates it. The factor-base construction time also differs for the same reason (a square root per candidate x, plus one [h]F per entry when projected).
- The fixed cost of a solve with the H1 projection at fb 32 is about 66 ms on E-222 (12.5 + 42.9 + 10.5) and 62 ms on M-221 (14.3 + 37.6 + 9.3), once per solve, against a per-trial cost of 2.8 ms. They only matter if a solve needs fewer than a few tens of trials, which it does not at these sizes.

**Limits.** No full native-size solve (about 2^110 operations, not run); no multiple hosts; the lookup was timed on misses only (a hit costs more, a rare event); elimination timed on random systems, not on collected relations.
