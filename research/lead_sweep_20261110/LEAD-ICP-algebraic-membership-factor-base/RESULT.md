# RESULT: LEAD-ICP-algebraic-membership-factor-base

## Outcome
- status: ran; exit code 0; wall 37 s (run.status), elapsed_s 37.1 inside run.py. run.stderr empty.
- Single run, no retries, PROTOCOL.md and run.py untouched.

## Statistic vs preregistered prediction
Prediction (PROTOCOL.md s3): for all six families, mean ratio < 1.02 or Stouffer z <= 5. Falsified only if a family has ratio >= 1.02 AND z > 5, with controls passing.

Measured (12 cells each: 4 primes x 3 seeds), mean ratio / Stouffer z:
- x_Dickson3_image: 1.00018 / -0.25
- x_QR_char_const: 0.99958 / -0.76
- x_cube_or_QR: 0.99995 / -1.00
- x_image_t2_plus_t: 1.00049 / +1.56
- x_interval_lowest: 1.00076 / +1.54
- x_square_image_t2: 0.99948 / -0.94

gain_families = []. Largest pooled excess is 0.08 percent, 25x below the 2 percent threshold.

Controls:
- Null (random reference): by construction; random mean coverage about 0.990 of N.
- A/A pooled Stouffer z = -0.008 (limit 4): pass.
- Positive control (planted subgroup): positive_control_ok = true; ratios 0.03 to 0.24 in cells where a divisor exists (limit 0.8): pass.

Verdict: consistent_with_prediction; controls passed.

## Reading
- The run.py verdict field matches the numbers. No discrepancy found.
- The test measures only the coverage version of the obstruction (distinct +-f_i +-f_j sums at m=2, B=1.5 sqrt N, N <= 2^17) for x-coordinate loci chosen by ascending x. The positive control shows the detector sees genuine additive structure (subgroups), so the null result is informative at this scale.
- Several families coincide in some cells (identical ratio/z for QR, cube_or_QR, t^2 image): the loci overlap heavily, so the six families are not independent tests.
- Per-cell z values reach about 2 to 2.9 occasionally (also in A/A), consistent with noise across 72 family-cells.
- Does not test KN-OPEN-020 proper (sub-B-degree membership surviving substitution, Groebner degree), larger N, m>=3, or j=0 orbits/true coset bases. Coverage excess growing with N is not excluded.

## Bearing on the sub-sqrt(p) question
closes_or_confirms_obstruction: algebraic x-loci bases give coverage indistinguishable from random bases at toy scale, repeating the recorded EV-FBG-001-type evidence; no signal.

## Standing
[pcc:unverified], single local toy run, no independent replay.
