# RESULT: LEAD-ICB-kic-golden-base-exact-decomp

## Outcome
- Status: ran. Exit code 0. Wall 3 s (run.status). Single run, no retries.

## Statistic vs preregistered prediction
- Prediction (PROTOCOL.md s3): |mean(|S_flat|/|S_null| - 1)| < 0.02; falsified if |z| > 4 and |mean| > 0.01; A/A |z| <= 4; progression control mean < -0.3.
- Measured (n=13, l=6, 120 trials, seed 20261110):
  - flat vs null: mean -0.00138, se 0.00237, z -0.58
  - A/A null vs null: mean +0.00103, se 0.00194, z +0.53
  - positive control (progression): mean -0.9289, z -1096
- Verdict: consistent_with_prediction. Controls passed: null, A/A and positive control.

## Reading
- The toy tests only whether an x-affine 2-flat base changes distinct-sum count for m=2 against a random x-set of equal size. It does not.
- The flat-vs-null effect (-0.14%) is about 0.6 SE, no different from the A/A noise.
- The positive control shrank the sumset by 93%, so the detector can see structure that the group law respects (+-kG progression).
- The verdict field matches the numbers. Nothing surprising.
- Limits (PROTOCOL s6): this is not a review of the KIC runs, and does not cover orbit-canonical lookup, Frobenius orbits or SAT-vs-MITM timing. It uses toy n=13, m=2, and flats in x only.

## Bearing on the sub-sqrt(p) question
- none: x-affine flat structure gives no coverage change at toy size, so it neither helps nor hurts exact decomposition and says nothing about sub-sqrt(p).

## Standing
[pcc:unverified], single local toy run, no independent replay.
