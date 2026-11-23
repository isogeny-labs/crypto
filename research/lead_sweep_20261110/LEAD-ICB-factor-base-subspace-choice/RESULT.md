# RESULT: LEAD-ICB-factor-base-subspace-choice

## Outcome
- status: ran (single run, no retries). exit=0, wall=17 s (run.status). stderr empty/tail uneventful.

## Statistic vs preregistered prediction
- Prediction (PROTOCOL.md s3): "for all three structured families t <= 4.0 (one-sided, above random)"; A/A |t|<=4; positive control enrichment >= 5x; verdict consistent_with_prediction.
- Measured (m=16, d=8, 12 curves, seed 20261110):
  - random_AA: t=0.476, ratio 1.009 (A/A ok)
  - geometric_progression: t=1.04, ratio 1.053
  - min_dimVV_of_30: t=0.596, ratio 1.018
  - trace0_window: t=49.04, ratio 2.068 (log ratio 0.727)
  - positive control (GF(2^8) subfield) closure enrichment 127.45 (>=5 ok)
- Verdict (run.py): falsified, because trace0_window t > 4 with A/A and positive control valid. Controls passed: null (random V), A/A, positive control.

## Reading
- Prediction is falsified by the preregistered rule, via one family only; two of three structured families are null.
- The trace0_window ratio of about 2.07 is very likely a structural identity, not a discovered property. For x1,x2 in V with Tr=0 and a with Tr(a)=0 (a in {0,1}, m even), x3 = lam^2+lam+a+x1+x2 has Tr(x3)=0 automatically, since Tr(lam^2+lam)=0. So landing in a trace-0 V is twice as likely as for a random V (probability 2^(d-m+1) vs 2^(d-m)). Observed ratio 2.07 matches this factor 2. This is my analysis, not tested by run.py; the run did not separate it from a genuine effect.
- It is a fixed constant factor (about 2x), independent of m and d, so it does not grow with size. The lead's reopening condition needs an effect that grows with size. The toy cannot show growth (single m).
- The yield proxy is a 3-sum count normalised by |S|^3, not real decomposition cost; a 2x pair-count gain also halves the usable V dimension in a real setting (V sits in a hyperplane, losing one bit of dimension), which the toy does not price. No rho gap measured.
- run.py's verdict field matches the numbers.

## Bearing on the sub-sqrt(p) question
constant factor only: trace-parity forces a factor of about 2 in toy 3-sum closure, with no evidence of growth in m or exponent change.

## Standing
[pcc:unverified], single local toy run, no independent replay.
