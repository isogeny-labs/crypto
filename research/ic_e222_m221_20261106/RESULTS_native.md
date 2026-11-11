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
