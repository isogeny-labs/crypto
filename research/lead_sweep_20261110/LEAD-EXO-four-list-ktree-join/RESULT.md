# RESULT: LEAD-EXO-four-list-ktree-join

## Outcome
- status: ran (single run, no retries); exit code 0; wall 8 s (run.status: exit=0 wall=8); run.stderr empty.
- run.py verdict field: consistent_with_prediction.

## Statistic vs preregistered prediction
Prediction (PROTOCOL.md s3): all r_p = W_xmod/W_null in [0.8,1.25] and OLS log-log slope of W_xmod in [-0.65,-0.35]; null slope in [-0.6,-0.4]; planted W > 3 W_null at every size; A/A ratio in (0.7,1.43); any control failure gives inconclusive.

Measured (p = 4721, 19301, 78401, 302111, 1281649; 6 curves each):
- r_p = 1.219, 1.089, 0.958, 1.030, 1.000 (all inside the band; the smallest size is near the upper edge at 1.219 and shrinks toward 1 as N grows).
- slope W_xmod = -0.514 (inside [-0.65,-0.35]); W_null slope = -0.481 (inside [-0.6,-0.4]); W_planted slope = +0.498.
- A/A ratios = 0.888, 1.024, 0.982, 1.018, 0.999 (all in (0.7,1.43)).
- planted/null ratio is about 200 to 50000 at every size, far above 3.
Controls: null passed, positive passed, A/A passed. Verdict: consistent_with_prediction.

## Reading
- The numbers support run.py's verdict field; no discrepancy found. W_xmod tracks both W_null and the 1/mean Poisson prediction (W_pred_1_over_mean) closely at every size.
- The 1.219 ratio at p~2^12 is within the band but is the largest deviation; with only 6 curves per size and one seed this is plausibly noise (r_p converges to 1.00 at 2^20), but the run does not test it.
- The toy does not test the lead's claim. x mod B is not a sum-compatible filter on the group, it is not the O2 agreement kernel (whose definition was not found), and it is not a Wagner four-list join with a cost exponent. It shows only that the abscissa-mod-B bucketing of whole point sets on random curves is Poisson-like, which is expected. Exhaustive enumeration at N <= 2^20 says nothing about p > 2^30 or sampled lists.
- Nothing surprising. The planted control's W grows with N (slope +0.5), as expected for a fixed 20% concentration, which confirms the detector sees concentration.

## Bearing on the sub-sqrt(p) question
none: the stand-in filter shows no bucket concentration, which is the expected null, and it neither extends nor refutes the recorded bucket-averaged barrier for sum-compatible or character filters.

## Standing
[pcc:unverified], single local toy run, no independent replay.
