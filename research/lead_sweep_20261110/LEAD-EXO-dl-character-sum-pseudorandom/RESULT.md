# RESULT: LEAD-EXO-dl-character-sum-pseudorandom

## Outcome
- Status: ran to completion. Exit code 0. Wall 6 s (run.status: exit=0 wall=6). run.stderr empty.
- One run only, seed 20261110, 40 curves (10 per size, b = 11..14).

## Statistic vs preregistered prediction
Prediction (PROTOCOL.md section 3): CI contains 1 (or lies below), and slope <= 0.01 per bit.
Falsified only if CI lower bound > 1 AND slope > 0.01 per bit, with valid controls.

Measured:
- Pooled R = 1.0278, 95% CI [0.9978, 1.0577]. The CI contains 1 (lower bound 0.9978).
- Slope of per-size mean R on log2 p = -0.0063 per bit (<= 0.01).
- Per-size mean R: 2^11 1.039 (se 0.031), 2^12 1.015 (0.032), 2^13 1.051 (0.036), 2^14 1.006 (0.026).
- Controls:
  - A/A (5/5 null split) mean = 0.990, inside [0.9, 1.1]: pass.
  - Positive control (interval of h consecutive DL indices) mean ratio = 2.417 (> 1.4): pass.
    It rises with size, 1.98, 2.24, 2.54, 2.91.
  - Null: negation-closed random subsets, 10 per curve, same curve: used as the R denominator.
- Verdict (run.py): consistent_with_prediction. Controls passed.

## Reading
- The toy tests the recorded toy-scale claim: max |additive-character sum| of the folded DL over the
  smallest-x set versus a negation-matched null. It does not test the 2^30 reopen condition.
- The verdict field matches the numbers. No discrepancy between it and the data.
- Caveats:
  - The CI lower bound (0.9978) sits just under 1, and the pooled mean is 2.8% above 1. This is
    marginal. It is not significant, but the CI does not clearly center on 1.
  - Per-size means are all >= 1.006. This agrees with the earlier 2^20 value (1.040 [0.999, 1.092]).
  - The slope is negative and the per-size se (about 0.03) makes it noisy. About 10 curves per size
    give little power to detect a trend of 0.01 per bit.
  - The positive control grows with size, so the instrument is not blind to a gross bias.
  - It was not run against weak-class curves, only random ones.

## Bearing on the sub-sqrt(p) question
closes or confirms the recorded obstruction: at toy scale the DL character-sum bias on the smallest-x
factor base is indistinguishable from the negation-matched null, with no upward trend.

## Standing
[pcc:unverified], single local toy run, no independent replay.
