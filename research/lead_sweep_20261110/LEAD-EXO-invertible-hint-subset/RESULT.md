# RESULT: LEAD-EXO-invertible-hint-subset

## Outcome
- Status: ran to completion, exit code 0, wall 8 s (run.status: exit=0 wall=8). The estimate was 2-6 min, so it ran much faster than estimated. run.stderr was empty.
- Curves: p = 809, 1103, 1409; N = 829, 1091, 1439; n = 414, 545, 719. T = 2500, seed0 = 20261110.

## Statistic vs preregistered prediction
Prediction (PROTOCOL.md section 3): z <= 4 for every d in {1,2,3}; A/A |z| <= 4; planted 25% polynomial recovered (max agreement >= 0.15 n). Falsified only if z > 4 with both controls passing.

Measured:
| d | z_EC_vs_null | z_AA | mean excess EC | mean excess null | positive-control max agreement |
|---|---|---|---|---|---|
| 1 | -0.925 | -0.338 | 0.494 | 0.505 | 105 |
| 2 | 0.177 | 1.015 | 0.495 | 0.493 | 104 |
| 3 | 0.011 | 0.118 | 0.503 | 0.503 | 104 |

Max agreement over all trials was 6-9 points for every EC and null arm.
Verdict: consistent_with_prediction (flags falsify=false, pos_fail=false, aa_fail=false).
Controls: null arm gives the comparison; A/A passed (|z_AA| <= 1.02); positive control passed (104-105 vs threshold 0.15 * 414 = 62).

## Reading
- The toy tests what the protocol says it tests: low-degree (<= 3) RANSAC agreement with the half-log graph at n ~ 400-720. It found none beyond the null.
- run.py's verdict matches the numbers. Excess agreement is about 0.5 in every arm, which equals the chance level (about n/p ~ 0.5 extra random matches), so the EC and null arms are indistinguishable.
- Positive control reached about 105 of 414 (the planted 25% is 103.5), so the detector sees a 25% planted polynomial.
- Caveats: the "arms" field in RESULT.json is an empty dict, so per-arm raw data is not dumped. Only summary statistics are available. RANSAC cannot see dense subsets below about 15% density at d=3. Toy size and d <= 3 only. No Lange-Winterhof theorem was read or checked.

## Bearing on the sub-sqrt(p) question
none: a null result at toy scale (p ~ 10^3, d <= 3) says nothing about asymptotic A(d), higher degrees or other invertible subsets, and is not a cost claim.

## Standing
[pcc:unverified], single local toy run, no independent replay.
