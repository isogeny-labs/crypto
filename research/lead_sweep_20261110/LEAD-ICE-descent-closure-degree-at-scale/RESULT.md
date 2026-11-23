# RESULT: LEAD-ICE-descent-closure-degree-at-scale

## Outcome
- status: ran (single run, no retries). exit code 0. wall 6 s (run.status: exit=0 wall=6). run.stderr empty.

## Statistic vs preregistered prediction
- Prediction (PROTOCOL.md s3): consistent if max D <= 4 and slope <= 0.2; falsified if max D >= 5 and slope > 0.2; otherwise inconclusive.
- Measured: max D = 4 (all arms). Mean D(l) for A: 2.778, 2.952, 3.850, 4.000 for l=3..6. slope = (4.000 - 2.778)/3 = 0.407 per l.
- Kept (unsatisfiable) draws per l of 24: A 18/21/20/20, A_prime 22/17/19/18, null 15/20/16/17.
- Verdict (run.py): inconclusive. Max <= 4 holds, but the slope exceeds 0.2, so neither consistent nor falsified.
- Controls: positive (a) satisfiable never refuted: pass (24 draws). Positive (b) overdetermined random system refuted, D = 2: pass. A/A max mean gap 0.107 <= 0.5: pass. Null (reported, not a gate): means 3.0, 4.0, 4.0, 4.0, i.e. null D >= descended D at every l (descended is not harder than random).

## Reading
- The toy tests a sharpened proxy (m=2, n=2l+1, l<=6, plain XL), not the blocked C1 cell (m=3, l=7).
- run.py's verdict matches the numbers. The slope is inflated by the low-l end (D=3 at l=3,4, then D=4 at l=5,6). The mean reaches the ceiling of 4 and flat at l=6 (all 20 draws at 4), with no draw at 5 or 6.
- D <= 4 held at every l, but we see saturation at 4 rather than evidence of a plateau below it; with D capped at 6, no draw needed 5 so no growth beyond 4 appeared at l=6.
- The null arm reaches D=4 already at l=4, so closure degree here largely reflects the system's size and quadratic structure, not descent-specific structure. Many draws were dropped by the unsatisfiable filter (15-22 of 24 kept).
- Slope is a coarse measure: it counts the step from 3 to 4, which is expected from growth in variable count at fixed n=2l+1.

## Bearing on the sub-sqrt(p) question
none: at l<=6 the toy shows D saturating at 4 for descended and random systems alike, which says nothing about degree 5 at the m=3, l=7 cell or about any sub-sqrt(p) gain.

## Standing
[pcc:unverified], single local toy run, no independent replay.
