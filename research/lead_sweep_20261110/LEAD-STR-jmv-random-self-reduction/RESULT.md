# RESULT: LEAD-STR-jmv-random-self-reduction

## Outcome
- status: ran. exit code 0. wall 4 s (run.status: exit=0 wall=4). stderr empty. One run only.

## Statistic vs preregistered prediction
Prediction (PROTOCOL s3): |ratio-1| <= 0.15 in every expander cell (h=16411, 65537, 262147; delta=0.05; 20000 walks;
random and interval sets); positive control k=1 ratio > 3; A/A gap <= 0.30; verdict consistent_with_prediction.

Measured ratio P_hit/delta:
- h=16411:  random 0.919, interval 0.934 (extra null 0.936)
- h=65537:  random 0.973, interval 1.051 (extra null 0.995)
- h=262147: random 1.037, interval 1.000 (extra null 0.980)
- Max |ratio-1| = 0.081 (h=16411 random), inside the 0.15 band.
- A/A gaps: max 0.082 (extra null, h=65537), all <= 0.30. aa_ok true.
- Positive control (k=1, interval): ratios 20.01, 20.005, 20.0005 (> 3). positive_control_ok true.
- Null (random set, extra seed): inside band.
Verdict: consistent_with_prediction. Controls passed: null, positive, A/A.

## Reading
- run.py verdict matches the numbers; no discrepancy.
- Positive control ratio is 1/delta = 20 to three-plus digits with A/A gap 0: with k=1 and L steps of +-1, the walk never leaves the interval around the start, so P_hit = 1. It shows the detector sees non-mixing, but it is a trivial extreme, not a graded sensitivity test.
- The small-h cells sit slightly low (0.92-0.93) and the A/A noise is a few percent, so the 0.15 band is not tight; a mild deficit at h=16411 (both random and interval, same direction) may be finite-L/mixing residue or noise, not resolved here.
- The toy tests only mixing on a cyclic Cayley graph Z/h. It does not model isogeny cost, non-cyclic class groups, or the existence of a delta-density oracle.

## Bearing on the sub-sqrt(p) question
closes or confirms the recorded obstruction: the walk is a neutral L-factor wrapper hitting at rate delta, so the self-reduction adds overhead L/delta and no gain over the oracle's own cost.

## Standing: [pcc:unverified], single local toy run, no independent replay.
