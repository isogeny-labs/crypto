# RESULT: LEAD-ICP-full-width-arity-grid

## Outcome
- Status: ran to completion. Exit code 0. Wall 92 s (run.status: exit=0 wall=92). run.stderr empty.
- One run only, seed 20261110, no retries or edits.

## Statistic vs preregistered prediction
Prediction (PROTOCOL.md section 3): |R_m - 1| <= 0.05 in every cell with null coverage < 0.9; positive control (arithmetic-progression base) deviates > 0.2 at m=2 on every curve; A/A (ninth null draw) within 0.05 in informative cells. Falsified if any informative cell has |R_m - 1| > 0.05.

Measured R_m (actual / null mean), z against null sd:
| p (N) | m=2 | m=3 | m=4 (null cov) |
|---|---|---|---|
| 40009 (N=40009) | 1.0000 (z 0.00) | 0.9947 (z -0.23) | 0.9937 (z -0.39), null 0.928 (saturated) |
| 52837 (N=52813) | 1.0044 (z 0.69) | 1.0043 (z 0.22) | 0.9989 (z -0.06), null 0.873 |
| 65521 (N=65071) | 1.0069 (z 1.98) | 1.0260 (z 2.75) | 1.0234 (z 1.90), null 0.803 |

Largest informative deviation: 2.6% (p=65521, m=3), below the 5% threshold. The p=40009 m=4 cell is saturated (null 0.928 >= 0.9), reported only; it is also within 0.7%.
Controls: positive control ratios 0.10 (m=2) on all curves, so |ratio-1| ~ 0.90 > 0.2: detector sees structure. A/A ratios 0.997 to 1.012: within 0.05. Null control built in. checks = all true.
Verdict: consistent_with_prediction (run.py field matches the numbers).

## Reading
- The toy tests only the narrow sharpened claim (sumset coverage of a small-x base vs random symmetric subsets). That claim holds at these sizes. It does not test relation yield with signs/ordering, coset, Koblitz or binary bases, and says nothing on 63-bit solves (PROTOCOL.md section 6).
- Caveats: the p=65521 curve shows mildly positive z (1.9 to 2.7) at m=2..4, the same sign in all three cells (the cells are not independent, being nested sumsets). The size is small (about 2.6% at most) and under the preregistered threshold, but it is the only hint of non-genericity. Three curves cannot say whether it is noise.
- The first curve has N = p = 40009, so it is an anomalous (trace 1) curve, not a typical one. It is still valid for this combinatorial test, but it is an unusual member of the sample.
- The m=2 cell on p=40009 shows ratio exactly 1.0 and z = 0: the actual coverage equals the null mean to all digits (795 or so points). At arity 2 with about 40 points, collisions are rare, so coverage is nearly deterministic in f. This is expected and not a sign of an error, but it means m=2 cells carry little power.
- The m=4 cells are near saturation, so power is also limited there; m=3 is the most informative arity.

## Bearing on the sub-sqrt(p) question
none: this confirms only that generic coverage is not contradicted at toy scale for small-x bases (no speedup, nothing about the 63-bit panel).

## Standing
[pcc:unverified], single local toy run, no independent replay.
