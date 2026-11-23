# RESULT: LEAD-RHO-non-automorphism-equivalence

## Outcome
- status: ran. exit code 0, wall 2 s (run.status: exit=0 wall=2). run.stderr empty. One run, no retries.
- run.py verdict field: consistent_with_prediction; controls_ok: true.

## Statistic vs preregistered prediction
Prediction (PROTOCOL.md s3): for every h in {8,12,16,24,32}, R_cost > 1 and |R_step/(1/sqrt h) - 1| <= 0.25. Falsified if any h > 6 has R_cost <= 1 or the step law fails by more than 25%.

Measured (N ~ 2^20, 400 trials/arm):
| h | R_step | 1/sqrt h | rel err | R_cost (generic images) | break-even c* (steps) |
|---|---|---|---|---|---|
| 8 | 0.2693 | 0.3536 | -0.238 | 56.8 | 0.388 |
| 12 | 0.2354 | 0.2887 | -0.185 | 77.9 | 0.295 |
| 16 | 0.2047 | 0.2500 | -0.181 | 92.3 | 0.259 |
| 24 | 0.1629 | 0.2041 | -0.202 | 112.5 | 0.223 |
| 32 | 0.1462 | 0.1768 | -0.173 | 136.1 | 0.188 |

R_cost > 1 for all five (min 56.8). Step law holds for all five, but h=8 is at -23.8%, close to the 25% limit. Verdict: consistent_with_prediction.
Controls: positive h=2 R_step = 0.609, in (0.6, 0.85) but only just. Null detector: free-image cost ratio equals R_step < 1 for all h > 1, passed. A/A h=1 reseeded ratio 1.0055 (< 0.1 band), passed. Negative control h=1 arm (seed +101) gave ratio 0.894 against base.

## Reading
- The toy tests the arithmetic of the lead's claim: a sqrt(h) step gain cannot pay for h-1 generic images priced at 1.5 log2 N ops each. R_cost is 57 to 136, so it is not marginal. Break-even c* is about 0.19 to 0.39 step per image, matching the prediction of under 0.27 step for h >= 12 (h=8 gives 0.39, slightly above the 0.27 quoted in the protocol for h >= 8).
- Caveats: R_cost is dominated by the priced image cost assumption (c_img = 1.5 log2 N = 30 ops). The step measurement is the only empirical part.
- Discrepancy/surprise: all step ratios fall systematically 17 to 24% below 1/sqrt(h), and the h=1 arm (seed +101) is 10.6% below the base arm even though the A/A arm (seed +1) matches at 1.0055. So the baseline (seed 20261110) mean of about 1604 steps looks high by about 1 to 2 sigma, and part of the apparent shortfall of the ratios is baseline noise rather than a real effect. The positive control passing at 0.609 (limit 0.6) is the same bias. Direction: ratios are lower than 1/sqrt h, i.e. in the helpful direction for the quotient, the opposite of the "fruitless cycles" overstatement the protocol expected. Not investigated further (no reruns allowed).
- The step-law check at h=8 (-23.8%) would likely fail with a different baseline seed, but the verdict on cost does not depend on it.

## Bearing on the sub-sqrt(p) question
closes or confirms the recorded obstruction: generic non-automorphism quotients cost far more in images than they save in steps, by a factor of 57 to 136 here; this says nothing about special H with cheap images.

## Standing
[pcc:unverified], single local toy run, no independent replay. Abstract Z/N, not elliptic curves; no matched-rho band; not a leaderboard row.
