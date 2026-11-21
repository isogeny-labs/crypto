# RESULT: LEAD-ICP-isogeny-transported-relation-yield

## Outcome
- status: ran to completion; exit code 0; wall 37 s (run.status: exit=0 wall=37); run.stderr empty/uneventful.
- All 5 primes finished (1009, 2003, 4001, 8009, 16001); run.py verdict: "falsified".

## Statistic vs preregistered prediction
Prediction (PROTOCOL s3): |z_mean| < 4 and var_ratio in [0.4, 2.5] at every p; positive control z < -3; A/A |z| < 3.5.
Falsified if |z_mean| >= 4 at any p, or var_ratio CI lower bound > 1 at both two largest p with larger-p ratio > smallest-p ratio.

| p | N | B | curves | z_mean | var_ratio [95% CI] | z_AA | z_pos |
|---|---|---|---|---|---|---|---|
| 1009 | 1030 | 39 | 21 | -0.46 | 4.42 [2.10, 7.40] | -0.40 | -14.3 |
| 2003 | 2040 | 54 | 24 | -1.85 | 4.52 [2.15, 7.37] | -0.25 | -13.9 |
| 4001 | 4032 | 76 | 24 | -2.42 | 3.54 [2.06, 5.01] | -0.17 | -10.0 |
| 8009 | 8040 | 108 | 24 | -1.14 | 6.31 [2.90, 10.52] | -0.92 | -33.6 |
| 16001 | 15972 | 152 | 24 | -1.47 | 4.49 [1.53, 8.79] | +0.87 | -44.7 |

- Mean part: holds. |z_mean| <= 2.42 everywhere (all below 4), all z_mean negative (class mean coverage ~0.1 to 0.4% below null, e.g. 0.5186 vs 0.5202 at p=16001).
- Variance part: var_ratio is outside [0.4, 2.5] at every p (3.5 to 6.3). CI lower bounds exceed 1 at all five p. Falsification clause 2 is met on its letter: lower bound > 1 at 8009 and 16001, and ratio(16001)=4.49 > ratio(1009)=4.42.
- Controls: positive control passed (z < -3 at every p), A/A passed (|z_AA| < 3.5 everywhere). Null control is the matched random-subset arm itself; no anomaly.
- Verdict: falsified (per the frozen rule), controls OK.

## Reading
- The falsification is driven by a flat excess of about 4.5x in across-curve variance, not by growth. Ratios go 4.4, 4.5, 3.5, 6.3, 4.5 with no trend in p. The "growing with size" content of the lead's hypothesis is not supported; the rule's second condition is satisfied only by a 0.07 difference between the endpoints (noise-level, CIs overlap heavily). The run.py verdict is therefore faithful to the preregistered letter but overstates a size-growing signal.
- Likely mundane explanation (not tested): the x-ordered factor base is a structured set whose coverage depends on the curve (and on the generator's discrete-log permutation), whereas the null is i.i.d. random subsets. Variances computed over <=24 curves (vs 5 null draws per curve) can differ for design reasons, e.g. curve-to-curve differences in the generator choice. The toy has no arm that separates "isogeny-class structure" from "any x-ordered point basis on a cyclic group", so the excess is not attributable to isogeny transport.
- Mean coverage slightly below null (z -0.5 to -2.4, same sign at all p) is a small consistent deficit, still under the 4-sigma threshold.
- The toy does test the sharpened question (coverage mean/variance of x-interval FBs within a class) but not arity 3, volcano level, real solve cost, or p >= 2^20 (PROTOCOL s6). Few curves per class limit variance power.

## Bearing on the sub-sqrt(p) question
constant_factor_only: a size-independent variance excess of ~4.5x in pair-sumset coverage with unchanged mean (and a slight mean deficit) cannot change an asymptotic exponent; yield is class-invariant on average. Possible follow-up would need a non-isogeny x-ordered control before reading the variance as isogeny-related.

## Standing
[pcc:unverified], single local toy run, no independent replay.
