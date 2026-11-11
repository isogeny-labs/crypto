# RESULT: LEAD-ICP-gfp5-oa-sdhp-jv

## Outcome
- status: ran (single run, no retries). exit code 0, wall 9 s (run.py reports 9.3 s).
- Files: RESULT.json, run.stderr (empty), run.status.

## Statistic vs preregistered prediction
Prediction (PROTOCOL.md s3): R = distinct 4-sums of FB / mean distinct 4-sums of 3 random negation-closed bases of equal size; consistent if every R in [0.9,1.1]; falsified if any R outside while controls pass; positive control (AP base) pos/null < 0.5; A/A rel diff < 0.05.

Measured (6 curves, p=7 x4, p=11 x2): R = 1.0, 1.0, 1.0, 1.0, 1.0042, 0.9912.
- max A/A relative difference 0.008 (< 0.05, pass).
- max pos_ctrl/null 0.2941 (< 0.5, pass).
- null control: random bases, counts reported in RESULT.json; controls_ok = true.
Verdict: consistent_with_prediction (run.py field), controls passed.

## Reading
- Base sizes m are tiny (6 to 16 points; FB 85 to 2919 distinct 4-sums). For p=7 the FB, all three nulls give identical counts (85, 85, 85, 85 or 501 x4). That identity suggests the count is saturated at the combinatorial maximum of 4-multisets of a negation-closed base, so there are no collisions to differ on. The ratio is then 1.0 almost by construction and the test has little power to see extra collisions in FB. Only the two p=11 curves show small variation (+0.4%, -0.9%).
- The positive control does show the detector can see strong additive structure (AP base collapses counts to 0.29 of null).
- run.py's verdict matches the numbers. Discrepancy/caveat: m is determined by what the run found and appears far smaller than the full FB (p^1-ish points); the reported decomp_prob_times_p (0.03 to 0.21) is not near the Poisson prediction (125 to 3830 vs observed 85 to 2919), because of repeats/negation in a tiny base.
- It tests only decomposition density at toy size, not C_pdp4, oracle-assisted static DH, or p = 2^64 - 2^32 + 1.

## Bearing on the sub-sqrt(p) question
none: toy result is saturated and tests only relation existence, not solving cost, which is the lead's actual open quantity.

## Standing
[pcc:unverified], single local toy run, no independent replay.
